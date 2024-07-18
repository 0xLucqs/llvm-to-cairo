use inkwell::values::{AsValueRef, InstructionValue, PhiValue};

use super::CairoFunctionBuilder;
use crate::builder::get_name;

impl<'ctx> CairoFunctionBuilder<'ctx> {
    pub fn process_phi(&mut self, instruction: &InstructionValue<'ctx>, is_loop: &bool) -> String {
        let phi = unsafe { PhiValue::new(instruction.as_value_ref()) };
        // name of the result variable
        let phi_name = get_name(phi.get_name()).unwrap(); // variable to store the result in
        // Incomming values (basic block + variable to set the value to)
        let first = phi.get_incoming(0).unwrap();
        // Name of the variable we should set the value to.
        let left_var = get_name(first.0.get_name()).unwrap(); // phi left variable

        // Incomming values (basic block + variable to set the value to)
        let second = phi.get_incoming(1).unwrap();
        // Name of the variable we should set the value to.
        let right_var = get_name(second.0.get_name()).unwrap(); // phi right variable

        self.variables.insert(first.0, left_var.clone());
        self.variables.insert(second.0, right_var.clone());
        self.variables.insert(phi.as_basic_value(), right_var.clone());
        // If we're in a subscope we don't need the `let` because we declared the variable above the scope.
        format!(
            "{}{} = if is_from_{} {{ {} }} else if is_from_{} {{ {} }} else {{ panic!(\"There is a bug in the \
             compiler please report it\")}};",
            if *is_loop { "" } else { "let " },
            phi_name,
            get_name(first.1.get_name()).unwrap(), // phi left basic block
            left_var,
            get_name(second.1.get_name()).unwrap(), // phi right basic block
            right_var
        )
    }
}
