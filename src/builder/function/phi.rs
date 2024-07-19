use inkwell::basic_block::BasicBlock;
use inkwell::values::{AsValueRef, BasicValueEnum, InstructionValue, PhiValue};

use super::CairoFunctionBuilder;

impl<'ctx> CairoFunctionBuilder<'ctx> {
    pub fn process_phi(&mut self, instruction: &InstructionValue<'ctx>, bb: &BasicBlock<'ctx>) -> String {
        let annoying_phis = self.bblock_variables.get(bb).cloned().unwrap_or_default();
        let phi = unsafe { PhiValue::new(instruction.as_value_ref()) };
        // name of the result variable
        let phi_name = annoying_phis
            .get(unsafe { &BasicValueEnum::new(instruction.as_value_ref()) })
            .cloned()
            .unwrap_or_else(|| {
                let name = self.get_name(phi.get_name());
                // if it was not in the mapping insert it. In theory we could insert it in any case but we don't
                // want to do that it would poisin the regular variable mapping with the annoying phis and would
                // mess everything up
                self.variables.insert(phi.as_basic_value(), name.clone());
                name
            }); // variable to store the result in

        // Incomming values (basic block + variable to set the value to)
        let first = phi.get_incoming(0).unwrap();
        // Name of the variable we should set the value to.
        let left_var = self.variables.get(&first.0).cloned().unwrap_or_else(|| {
            let name = self.get_name(first.0.get_name());
            self.variables.insert(first.0, name.clone());
            name
        }); // phi right variable

        // Incomming values (basic block + variable to set the value to)
        let second = phi.get_incoming(1).unwrap();
        // Name of the variable we should set the value to.
        let right_var = self.variables.get(&second.0).cloned().unwrap_or_else(|| {
            let name = self.get_name(second.0.get_name());
            self.variables.insert(second.0, name.clone());
            name
        }); // phi right variable
        // If we're in a subscope we don't need the `let` because we declared the variable above the scope.
        format!(
            "let {} = if is_from_{} {{ {} }} else if is_from_{} {{ {} }} else {{ panic!(\"There is a bug in the \
             compiler at var {} please report it\")}};",
            phi_name,
            self.get_name(first.1.get_name()), // phi left basic block
            left_var,
            self.get_name(second.1.get_name()), // phi right basic block
            right_var,
            phi_name
        )
    }
}
