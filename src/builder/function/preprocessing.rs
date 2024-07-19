use std::collections::{HashMap, HashSet};

use inkwell::basic_block::BasicBlock;
use inkwell::values::{AnyValue, AsValueRef, BasicValueEnum, FunctionValue, InstructionOpcode, PhiValue};
use petgraph::algo::{has_path_connecting, tarjan_scc};

use super::CairoFunctionBuilder;

impl<'ctx> CairoFunctionBuilder<'ctx> {
    /// Construct a graph of basic blocks and detects loops.  It will detect if bb1 jumps to bb2
    /// which jumps back to bb1 or if bb1 jumps to itself. Will also get the return block (basic
    /// block that all paths leads to) Also collects all the `phi` instructions (more precisely
    /// the 2 incomming basic blocks
    pub fn preprocess_function(&mut self, function: &FunctionValue<'ctx>) {
        // Put all the basic blocks in the graph and save their id.
        for bblock in function.get_basic_block_iter() {
            let node_index = self.bb_graph.add_node(bblock);
            self.node_id_from_name.insert(bblock, node_index);
        }

        for bblock in function.get_basic_block_iter() {
            // Those 2 helper variables will help detect if a phi depends on a previous phi because this
            // amazingly well though language can have blocks like that:
            // %res1 = phi i128 [ %var1 , %bb2 ], [ %var2, %start ]
            // %res2 = phi i128 [ %res1, %bb2 ], [ %var3, %start ]
            // And yes you guessed it it doesn't want the latest value of res1 but the value from the last time
            // we executed this block. Well done Chris
            // So we need to detect those to have temp variables to store the new value until we reach the end
            // of the BB and then we'll update variables that live in a greater scope.
            let mut bblock_phis_inc: HashSet<BasicValueEnum> = HashSet::default();
            let mut bblock_phis: HashSet<BasicValueEnum> = HashSet::default();
            for instr in bblock.get_instructions() {
                match instr.get_opcode() {
                    InstructionOpcode::Br => {
                        // Get the br instructions (jumps) to add a link with a direction in the graph
                        // Let's say we have
                        // bb1
                        //   br i1 %0, label %bb2, label %bb3
                        // bb2:
                        //    stuff
                        // bb3:
                        //    stuff
                        // we'd add a link from bb1 to bb2 and from bb1 to bb3
                        let bb_index = *self.node_id_from_name.get(&bblock).unwrap();
                        for operand in instr.get_operands().flatten() {
                            if let Some(target) = operand.right() {
                                let target_index = *self.node_id_from_name.get(&target).unwrap();
                                self.bb_graph.add_edge(bb_index, target_index, ());
                            }
                        }
                    }
                    InstructionOpcode::Phi => {
                        // Get the phis incomming basic blocks because we'll add booleans to track from which block
                        // we're comming from as this doesn't exist in cairo.
                        let inc1 = unsafe { PhiValue::new(instr.as_value_ref()).get_incoming(0).unwrap() };
                        let inc2 = unsafe { PhiValue::new(instr.as_value_ref()).get_incoming(1).unwrap() };
                        bblock_phis_inc.extend([inc1.0, inc2.0]);
                        bblock_phis.insert(unsafe { BasicValueEnum::new(instr.as_value_ref()) });

                        self.phis_bblock.extend([inc1.1, inc2.1]);
                    }
                    _ => (),
                };
            }
            // Get the variables that are referenced by a phi in the bb they're declared and add them in the
            // annoying phi hashmap with a suffix so we can create the correct variable in the previous scope
            // and update it.
            bblock_phis.intersection(&bblock_phis_inc).for_each(|annoying_phi| {
                let phi_name = format!("{}_temp", self.get_name(annoying_phi.get_name()));
                self.bblock_variables
                    .entry(bblock)
                    .and_modify(|hm| {
                        hm.insert(*annoying_phi, phi_name.clone());
                    })
                    .or_insert_with(|| {
                        let mut nhm = HashMap::new();
                        nhm.insert(*annoying_phi, phi_name);
                        nhm
                    });
            })
        }
        // Detect the strongly connected components (strongly connected basic blocks == loops)
        self.bb_loop = tarjan_scc(&self.bb_graph)
            .into_iter()
            .filter_map(|nodes| {
                // nodes.len() > 1 <=> bb1 jumps to bb2 which jumps to bb1
                // self.bb_graph.contains_edge(a: nodes[0], b: nodes[0] <=> bb1 jumps to itself
                // TODO(Lucas): support complex loops
                (nodes.len() > 1 || self.bb_graph.contains_edge(nodes[0], nodes[0])).then_some(self.bb_graph[nodes[0]])
            })
            .collect::<HashSet<_>>();

        // Get the node that all paths lead to.
        // if we have
        // bb1
        //   br i1 %0, label %bb2, label %bb3
        // bb2:
        //   br i1 %0, label %bb2, label %bb3
        // bb3:
        //    ret i32 1
        // bb3 will be our return block because there are 2 paths:
        // Path 1: bb1 => bb2(n times) => bb3
        // Path 2: bb1 => bb3
        // We need this because we need to make sure that the return instruction is is the main scope of the
        // function so this instruction is always reached.
        for target_node in self.bb_graph.node_indices() {
            let mut all_paths_lead = true;
            for source_node in self.bb_graph.node_indices() {
                if source_node != target_node && !has_path_connecting(&self.bb_graph, source_node, target_node, None) {
                    all_paths_lead = false;
                    break;
                }
            }
            if all_paths_lead {
                self.return_block = Some(self.bb_graph[target_node]);
            }
        }
    }

    /// If we were in a loop close it. Will also close the scope if we were in an else. Also close
    /// if this is the return basic block.
    pub fn close_scopes(&mut self, bb: &BasicBlock<'ctx>, is_else: &bool, is_loop: &bool) {
        // If we're in an else close it or if we're in the return block close it.
        // TODO(Lucas): Fix that it's 100% wrong as the return block isn't always the last block.
        if self.return_block.is_some_and(|bblock| &bblock != bb) && *is_else
            || self.return_block.is_some_and(|bblock| &bblock == bb)
        {
            self.push_body_line("}".to_string());
        }
        // If we were in a loop, close it
        if *is_loop {
            self.push_body_line("};".to_string());
        }
    }

    /// Create variables outside of the new scope (if/else/loop) so we can still access the value
    /// when we're out of it or at the next iteration.
    pub fn prepare_new_scopes(&mut self, bb: &BasicBlock<'ctx>, is_else: &bool, is_loop: &bool) {
        // If a basic block loops on itself || is this basic block an if clause || is this basic block an
        // else clause
        // TODO(Lucas): fix that it's incorrect for else blocks. This should be declared before the if
        if *is_loop || self.if_blocks.contains_key(bb) || *is_else {
            for instruction in bb.get_instructions() {
                // We'll create mutable variables outside of the loop or the condition because those are new scopes
                // so we'll lose the variables as soon as we escape from there or on each iteration which we don't
                // want because llvm doesn't have smaller scopes than the function itself so it uses variables from
                // other basic blocks (outside of our scope)
                if instruction.get_opcode() != InstructionOpcode::Br
                    && instruction.get_opcode() != InstructionOpcode::Return
                {
                    // Get the variable name, if it's unnamed generate a var{index} string.
                    let res_name = self.get_name(instruction.get_name().unwrap_or_default());
                    // Get the type of the variable because we'll add it to the definition to get more safety.
                    let ty = instruction.get_type().print_to_string().to_string();
                    // i1 are 1 bit integers meaning that they can only be {0, 1} they represent booleans. LLVM can
                    // work with arbitrary sized integers but not cairo so convert it to bool.
                    let val = if ty == "i1" { "false".to_owned() } else { format!("0_{ty}") };
                    self.push_body_line(format!("let mut {} = {};", res_name, val));
                    // Save the variable for later use.
                    let basic_val: BasicValueEnum = instruction.as_any_value_enum().try_into().unwrap();
                    self.variables.insert(basic_val, res_name);
                }
            }
        }
        // If it's an if condition open it with the condition variable.
        if let Some(cond) = self.if_blocks.get(bb) {
            // Here we negate the condition because the return basic block is the first once which is annoying
            // for us as we want it to be the last piece of code in our function.
            // TODO(Lucas): Verify that the return block is always the first operand.
            self.push_body_line(format!("if !{} {{", self.variables.get(cond).unwrap()));
            // If we're at the return block close the previous scope and do nothing as all paths
            // lead here.
        } else if &self.return_block.unwrap() == bb && *is_else {
            self.push_body_line("}".to_string())
            // If it's an else condition clause the previous if and open the else.
        } else if *is_else {
            self.push_body_line("} else {".to_string());
        }
        // If it's a loop open it.
        if *is_loop {
            self.push_body_line("loop {".to_string());
        }
    }
}
