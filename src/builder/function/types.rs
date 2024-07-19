use inkwell::values::{FunctionValue, InstructionValue};

use super::{CairoFunctionBuilder, CairoFunctionSignature, CairoParameter};

impl<'ctx> CairoFunctionBuilder<'ctx> {
    /// Translate the LLVM function signature into a Cairo function signature.
    ///
    /// # Arguments
    ///
    /// * `function` - The function we want to translate the signature of.
    /// * `fn_id` - Is the index of the function in our file but it can be any number it's just in
    ///   case the llvm function name is empty.
    ///
    /// # Returns
    ///
    /// * `String` - The cairo function signature in the form
    /// `pub fn <name>(<param1>: <type1>,<param2>: <type2>,) -> <return_type>`
    pub fn process_function_signature(
        &mut self,
        function: &FunctionValue<'ctx>,
        fn_id: usize,
    ) -> CairoFunctionSignature {
        // Get the function name and if it's empty call it "function{fn_id}"
        let name = function.get_name().to_str().map(|val| val.to_string()).unwrap_or(format!("function{fn_id}"));
        let mut parameters = Vec::<CairoParameter>::with_capacity(function.count_params() as usize);
        // Extract each parameter and its type.
        function.get_param_iter().for_each(|param| {
            let param_name = self.get_name(param.get_name());
            let param_type = param.get_type().print_to_string().to_string();
            self.variables.insert(param, param_name.clone());
            parameters.push(CairoParameter { name: param_name, ty: param_type });
        });
        // Get the return type of the function. If it's Some it means that the function returns a value else
        // it returns void.
        let return_type = if let Some(ty) = function.get_type().get_return_type() {
            ty.print_to_string().to_string()
        } else {
            "()".to_string()
        };
        CairoFunctionSignature::new(name, parameters, return_type)
    }

    /// Translate an LLVM Return instruction in cairo.
    pub fn process_return(&mut self, instruction: &InstructionValue) -> String {
        format!(
            "return {};",
            self.variables
                .get(
                &instruction
                        .get_operand(0)
                        .expect("Return opcode should have exactly 1 operand")
                        .left()
                        .expect("Return can only return a value hence left")
                )
                // TODO handle const
                .expect("Return a variable")
        )
    }
}
