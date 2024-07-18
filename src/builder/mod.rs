use std::collections::HashMap;
use std::ffi::CStr;

use function::CairoFunctionBuilder;
use inkwell::values::{FunctionValue, InstructionOpcode};

pub mod function;

/// Struct containing helpers to translate LLVM IR to cairo
#[derive(Default)]
pub struct CairoBuilder<'ctx> {
    /// Cairo functions
    pub(crate) functions: HashMap<FunctionValue<'ctx>, String>,
    pub(crate) code: Vec<String>,
}

fn get_name(name: &CStr) -> Option<String> {
    (!name.is_empty()).then_some(name.to_str().expect("Variable name for binary op should be uft-8").to_owned())
}
impl<'ctx> CairoBuilder<'ctx> {
    /// Translates an LLVM function to a cairo function and return it as a string.
    pub fn translate_function(&mut self, func: &FunctionValue) -> String {
        // Create a cairo function builder that will help us to build the function.
        let mut function_builder = CairoFunctionBuilder::default();
        // Start by extracting the signature and translating it to cairo. (All functions will be public the
        // original compiler already checked that there is no illegal call)
        let fn_sig = function_builder.process_function_signature(func, self.functions.keys().count());
        // append the function signature in the code vec.
        function_builder.code.push(fn_sig);
        // Iterate over the basic blocks of the function. Each function is composed of one or more basic
        // blocks. Basic blocks have one entry and one exit. If there was no return
        // instruction in the bb you'll need to jump to another bb at the end. For more information
        // read this https://llvm.org/doxygen/group__LLVMCCoreValueBasicBlock.html#details
        for bb in func.get_basic_block_iter() {
            // Iterate over each instruction of the basic block. 1 instruction == 1 LLVM code line
            for instruction in bb.get_instructions() {
                // Get the opcode of the instruction
                let code_line = match instruction.get_opcode() {
                    InstructionOpcode::Add => function_builder.process_binary_op(&instruction, "+"),
                    InstructionOpcode::Sub => function_builder.process_binary_op(&instruction, "-"),
                    InstructionOpcode::Return => function_builder.process_return(&instruction),
                    _ => "".to_owned(),
                };
                function_builder.code.push(code_line);
            }
        }
        function_builder.code.join("\n")
    }
}
