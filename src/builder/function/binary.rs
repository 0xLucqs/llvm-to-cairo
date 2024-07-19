use inkwell::basic_block::BasicBlock;
use inkwell::values::{AnyValue, BasicValueEnum, InstructionValue, IntValue};

use super::CairoFunctionBuilder;

impl<'ctx> CairoFunctionBuilder<'ctx> {
    fn extract_const_int_value(val: IntValue) -> String {
        // An llvm int constant is represented like this <type> <value> ex: i128 1234
        // First we get the value by getting the last chunk of its string representation
        let const_val = val.print_to_string()
                    .to_string()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    // Sanity check
                    .parse::<i128>()
                    .expect("Rust doesn't handle numbers bigger than u128");
        // Then get the type
        let ty = val.get_type().print_to_string().to_string();
        // Format it cairo style.
        // We add the type to have more type safety and detect bugs.
        format!("{const_val}_{ty}")
    }
    /// Translates an LLVM binary operation to cairo. This can be anything that expects exactly 1
    /// operator with a left and right operand.
    pub fn process_binary_int_op(
        &mut self,
        instruction: &InstructionValue<'ctx>,
        operator: &str,
        bb: &BasicBlock<'ctx>,
    ) -> String {
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

        let instr_name = {
            let basic_val: BasicValueEnum = instruction.as_any_value_enum().try_into().unwrap();
            // Try to get the variable from our variables mapping. If not found create it and insert it in the
            // mmaping.
            self.variables.get(&basic_val).cloned().unwrap_or_else(|| {
                let instr_name = self.get_name(instruction.get_name().unwrap_or_default());
                if let Ok(basic_value_enum) = instruction.as_any_value_enum().try_into() {
                    // Save the result variable in our mapping to be able to use later.
                    self.variables.insert(basic_value_enum, instr_name.clone());
                }
                format!("let {}", instr_name)
            })
        };

        let annoying_phis = self.bblock_variables.get(bb).cloned().unwrap_or_default();
        // The operand is either a variable or a constant so either we get it from our mapping or it's
        // unnamed and it's a const literal.
        // TODO(Lucas): a variable can surely be in the variables mapping, try to get it from there as well.
        let left_name = if left.into_int_value().is_const() {
            Self::extract_const_int_value(left.into_int_value())
        } else {
            // If it's not a const might be in our annoying phi mapping.
            annoying_phis.get(&left).cloned().unwrap_or_else(|| self.get_name(left.get_name()))
        };
        let right_name = if right.into_int_value().is_const() {
            Self::extract_const_int_value(right.into_int_value())
        } else {
            // If it's not a const might be in our annoying phi mapping.
            annoying_phis.get(&right).cloned().unwrap_or_else(|| self.get_name(right.get_name()))
        };

        format!("{} = {} {} {};", instr_name, left_name, operator, right_name)
    }
}
