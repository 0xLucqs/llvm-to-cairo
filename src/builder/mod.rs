use std::collections::{HashMap};
use std::ffi::CStr;
use std::fmt::Display;

use function::{CairoFunction, CairoFunctionBuilder};
use inkwell::values::{FunctionValue, InstructionOpcode};
use inkwell::IntPredicate;

pub mod function;

/// Struct containing helpers to translate LLVM IR to cairo
#[derive(Default)]
pub struct CairoBuilder<'ctx> {
    /// Cairo functions
    pub(crate) cairo_fn_from_llvm: HashMap<FunctionValue<'ctx>, String>,
    pub(crate) functions: CairoFunctions,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct CairoFunctions(Vec<CairoFunction>);
impl CairoFunctions {
    pub fn functions(&self) -> &[CairoFunction] {
        &self.0
    }

    pub fn count_functions(&self) -> usize {
        self.0.len()
    }
}

impl CairoFunctions {
    pub fn push_function(&mut self, function: CairoFunction) {
        self.0.push(function)
    }
}

impl Display for CairoFunctions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n"))
    }
}

fn get_name(name: &CStr) -> Option<String> {
    (!name.is_empty()).then(|| {
        let mut val = name.to_str().expect("Variable name for binary op should be uft-8").to_owned();
        val.retain(|c| c.is_alphanumeric() || c == '_');
        val
    })
}
impl<'ctx> CairoBuilder<'ctx> {
    /// Translates an LLVM function to a cairo function and return it as a string.
    pub fn translate_function(&mut self, func: &FunctionValue) -> CairoFunction {
        // Create a cairo function builder that will help us to build the function.
        let mut function_builder = CairoFunctionBuilder::default();

        function_builder.preprocess_function(func);

        // Start by extracting the signature and translating it to cairo. (All functions will be public the
        // original compiler already checked that there is no illegal call)
        function_builder.function.signature =
            function_builder.process_function_signature(func, self.cairo_fn_from_llvm.keys().count());

        // To understand that we need to know what the phi instruction does. It approximately does the
        // following:
        // %res = phi i32 [ %a, %bb1 ], [ %b, %bb2 ]
        // if the last instruction executed was in bb1 the value of res will be a,  if the last instruction
        // executed was in bb2 it will be b.
        // we'll call bb1 and bb2 the incoming basic blocks.
        // I recommend to read the official doc though https://llvm.org/docs/LangRef.html#phi-instruction
        // We'll create a mutable boolean that we'll initiate to false to know from which basic block we're
        // coming from as once again basic blocks don't exist in cairo.
        for bb in function_builder.phis_bblock.iter() {
            let code_line =
                format!("let mut is_from_{} = false;", get_name(bb.get_name()).expect("Basic block should be named"),);
            function_builder.function.body.push_line(code_line);
        }
        // Gets the index of the last basic block.
        let bb_num = func.count_basic_blocks() - 1;
        // Iterate over the basic blocks of the function. Each function is composed of one or more basic
        // blocks. Basic blocks have one entry and one exit. If there was no return
        // instruction in the bb you'll need to jump to another bb at the end. For more information
        // read this https://llvm.org/doxygen/group__LLVMCCoreValueBasicBlock.html#details
        for (index, bb) in func.get_basic_block_iter().enumerate() {
            // Boolean that let's us know if we need to wrap our basic block translation with a loop { bbcode };
            let is_loop = function_builder.bb_loop.contains(&bb);
            // Is this block the else clause of an if/else
            let is_else = function_builder.else_blocks.contains(&bb);
            // TODO(Lucas): in preprocess function declare all the variables that will be in subscopes and then
            // stop worrying about it + only use is_subscope
            let _is_subscope = is_loop || is_else || function_builder.if_blocks.contains_key(&bb);

            // Prepare for loops/if/else
            function_builder.prepare_new_scopes(&bb, &is_else, &is_loop);

            // Iterate over each instruction of the basic block. 1 instruction == 1 LLVM code line
            for instruction in bb.get_instructions() {
                // Get the opcode of the instruction
                let code_line = match instruction.get_opcode() {
                    InstructionOpcode::Add => function_builder.process_binary_int_op(&instruction, "+", is_loop),
                    InstructionOpcode::Sub => function_builder.process_binary_int_op(&instruction, "-", is_loop),
                    InstructionOpcode::Return => function_builder.process_return(&instruction),
                    InstructionOpcode::ICmp => {
                        // we just matched on ICmp so it will never fail
                        match instruction.get_icmp_predicate().unwrap() {
                            IntPredicate::EQ => function_builder.process_binary_int_op(&instruction, "==", is_loop),
                            IntPredicate::NE => function_builder.process_binary_int_op(&instruction, "!=", is_loop),
                            _ => "".to_owned(),
                        }
                    }
                    InstructionOpcode::Br => function_builder.process_branch(&instruction, &is_loop),
                    InstructionOpcode::Phi => function_builder.process_phi(&instruction, &is_loop),
                    _ => "".to_owned(),
                };
                // Add the line to the function body
                function_builder.push_body_line(code_line);
            }
            // If we're in an else close it, if we're in the return block don't do anything because we didn't
            // open the else.
            if is_else && function_builder.return_block.unwrap() != bb {
                function_builder.push_body_line("}".to_string());
            }
            // If we're not in the last basic block set all the booleans to the right value to know what basic
            // block we were in so we can process the phi instruction can work correctly
            if index as u32 != bb_num {
                for bblock in function_builder.phis_bblock.iter() {
                    // If we were in this basic block
                    if get_name(bblock.get_name()).unwrap() == get_name(bb.get_name()).unwrap() {
                        let code_line = format!(
                            "is_from_{} = true;",
                            get_name(bblock.get_name()).expect("Basic block should be named"),
                        );
                        function_builder.function.body.push_line(code_line);
                    } else {
                        // if we were not in this basic block
                        let code_line = format!(
                            "is_from_{} = false;",
                            get_name(bblock.get_name()).expect("Basic block should be named"),
                        );
                        function_builder.function.body.push_line(code_line);
                    }
                }
            }
            // If we were in a loop, close it
            if is_loop {
                function_builder.push_body_line("};".to_string());
            }
        }
        function_builder.function
    }
}
