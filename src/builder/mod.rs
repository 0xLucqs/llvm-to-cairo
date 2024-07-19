use std::collections::HashMap;
use std::fmt::Display;

use function::{CairoFunction, CairoFunctionBuilder};
use inkwell::values::FunctionValue;

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
            let code_line = format!("let mut is_from_{} = false;", function_builder.get_name(bb.get_name()),);
            function_builder.function.body.push_line(code_line);
        }
        // get the first basic block and process it. As they are all linked together we'll process all of
        // them recursively by calling `process_basic_block` before jumping to another block. Each function
        // is composed of one or more basic blocks. Basic blocks have one entry and one exit. If
        // there was no return instruction in the bb you'll need to jump to another bb at the end.
        // For more information read this https://llvm.org/doxygen/group__LLVMCCoreValueBasicBlock.html#details
        function_builder.process_basic_block(&func.get_first_basic_block().unwrap());
        function_builder.function
    }
}
