use std::collections::HashMap;
use std::fmt::Display;

use inkwell::values::{BasicValueEnum, FunctionValue, InstructionValue};

use super::get_name;
pub mod binary;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct CairoFunctionBuilder<'ctx> {
    pub(crate) variables: HashMap<BasicValueEnum<'ctx>, String>,
    pub(crate) function: CairoFunction,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct CairoFunction {
    pub(crate) signature: CairoFunctionSignature,
    pub(crate) body: CairoFunctionBody,
}

impl Display for CairoFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {{\n{}\n}}", self.signature, self.body))
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct CairoFunctionBody(Vec<String>);

impl CairoFunctionBody {
    pub fn new(body: Vec<String>) -> Self {
        Self(body)
    }
}

impl CairoFunctionBody {
    pub fn push_line(&mut self, line: String) {
        self.0.push(line)
    }
}

impl Display for CairoFunctionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.join("\n"))
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct CairoFunctionSignature {
    pub(crate) name: String,
    pub(crate) parameters: CairoParameters,
    pub(crate) return_type: String,
}

impl CairoFunctionSignature {
    pub fn new(name: String, parameters: Vec<CairoParameter>, return_type: String) -> Self {
        Self { name, parameters: CairoParameters(parameters), return_type }
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct CairoParameters(Vec<CairoParameter>);

impl Display for CairoParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.iter().map(ToString::to_string).collect::<Vec<_>>().join(","))
    }
}

impl Display for CairoFunctionSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("pub fn {}({}) -> {}", self.name, self.parameters, self.return_type))
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct CairoParameter {
    pub(crate) name: String,
    pub(crate) ty: String,
}
impl CairoParameter {
    pub fn new(name: String, ty: String) -> Self {
        Self { name, ty }
    }
}

impl Display for CairoParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}: {}", self.name, self.ty))
    }
}

impl<'ctx> CairoFunctionBuilder<'ctx> {
    pub fn name(&self) -> &str {
        &self.function.signature.name
    }

    pub fn arg(&self, parameter_nb: usize) -> &CairoParameter {
        &self.function.signature.parameters.0[parameter_nb]
    }

    pub fn return_type(&self) -> &str {
        &self.function.signature.return_type
    }
}

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
        let name = get_name(function.get_name()).unwrap_or(format!("function{fn_id}"));
        let mut parameters = Vec::<CairoParameter>::with_capacity(function.count_params() as usize);
        // Extract each parameter and its type.
        function.get_param_iter().enumerate().for_each(|(index, param)| {
            let param_name = get_name(param.get_name()).unwrap_or(index.to_string());
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
