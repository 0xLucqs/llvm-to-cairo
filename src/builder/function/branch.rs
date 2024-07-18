use inkwell::values::InstructionValue;

use super::CairoFunctionBuilder;

impl<'ctx> CairoFunctionBuilder<'ctx> {
    pub fn process_branch(&mut self, instruction: &InstructionValue<'ctx>, is_loop: &bool) -> String {
        let cond = instruction.get_operand(0).unwrap().left().unwrap();
        // If we're in a loop this is the exit condition so we break.
        if *is_loop {
            format!("if {}\n{{break;}}", self.variables.get(&cond).unwrap())
        } else {
            // else it means that we're in a if/else case and the first block is the if the 2nd is the else.
            self.if_blocks.insert(instruction.get_operand(1).unwrap().right().unwrap(), cond);
            self.else_blocks.insert(instruction.get_operand(2).unwrap().right().unwrap());

            "".to_owned()
        }
    }
}
