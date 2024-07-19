use inkwell::basic_block::BasicBlock;
use inkwell::values::InstructionValue;

use super::CairoFunctionBuilder;

impl<'ctx> CairoFunctionBuilder<'ctx> {
    /// Process a branch instruction. If there is only 1 operand without condition it'll translate
    /// the basic block it jumps to and will move on to the next basic block.
    /// If there is an if/else it will process all the basic blocks that are involved.
    pub fn process_branch(
        &mut self,
        instruction: &InstructionValue<'ctx>,
        bb: &BasicBlock<'ctx>,
        is_loop: &bool,
        is_else: &bool,
    ) -> String {
        // Get all the annoying variables that require to be declared in a bigger scope and will update
        // their value.
        self.bblock_variables.get(bb).cloned().unwrap_or_default().into_values().for_each(|val| {
            self.push_body_line(format!("{} = {};", val.trim_end_matches("_temp"), val));
        });
        self.set_basic_block_booleans(bb);
        // Case were there is an inconditionnal jump.
        if instruction.get_num_operands() == 1 {
            self.process_basic_block(&instruction.get_operand(0).unwrap().right().unwrap());
            "".to_owned()
        } else {
            // There is a condition could either be a loop break or if/else
            let cond = instruction.get_operand(0).unwrap().left().unwrap();
            // If we're in a loop this is the exit condition so we break.
            if *is_loop {
                let res = format!("if {}\n{{break;}}", self.variables.get(&cond).unwrap());

                res
            } else {
                self.close_scopes(bb, is_else, is_loop);
                // else it means that we're in a if/else case and the first block is the if the 2nd is the else.
                self.if_blocks.insert(instruction.get_operand(1).unwrap().right().unwrap(), cond);
                self.process_basic_block(&instruction.get_operand(1).unwrap().right().unwrap());
                self.else_blocks.insert(instruction.get_operand(2).unwrap().right().unwrap());
                self.process_basic_block(&instruction.get_operand(2).unwrap().right().unwrap());

                "".to_owned()
            }
        }
    }
}
