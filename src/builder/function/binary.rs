use inkwell::values::{AnyValue, InstructionValue};

use super::CairoFunctionBuilder;
use crate::builder::get_name;

impl<'ctx> CairoFunctionBuilder<'ctx> {
    /// Translates an LLVM binary operation to cairo. This can be anything that expects exactly 1
    /// operator with a left and right operand.
    pub fn process_binary_op(&mut self, instruction: &InstructionValue<'ctx>, operator: &str) -> String {
        // Get th left operand.
        let left = unsafe {
            instruction
                .get_operand_unchecked(0)
                .expect("Add should have a left operand")
                .left()
                .expect("left operand of add should be a basic value")
        };
        // Get the right operand.
        let right = unsafe {
            instruction
                .get_operand_unchecked(1)
                .expect("Add should have a right operand")
                .left()
                .expect("right operand of add should be a basic value")
        };
        // Get the name of the variable we want to store the result of the operantion in.
        let instr_name = get_name(instruction.get_name().unwrap_or_default()).unwrap_or("result".to_owned());
        if let Ok(basic_value_enum) = instruction.as_any_value_enum().try_into() {
            // Save the result variable in our mapping to be able to use later.
            self.variables.insert(basic_value_enum, instr_name.clone());
        }
        // The operand is either a variable or a constant so either we get it from our mapping or it's
        // unnamed as it's translated into a literal.
        let left_name = self
            .variables
            .get(&left)
            .cloned()
            .unwrap_or_else(|| get_name(left.get_name()).unwrap_or("left".to_owned()));
        let right_name = self
            .variables
            .get(&right)
            .cloned()
            .unwrap_or_else(|| get_name(right.get_name()).unwrap_or("right".to_owned()));

        format!("let {} = {} {} {};", instr_name, left_name, operator, right_name)
    }
}
