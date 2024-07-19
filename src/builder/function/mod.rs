use std::collections::{HashMap, HashSet};
use std::ffi::CStr;
use std::fmt::Display;

use inkwell::basic_block::BasicBlock;
use inkwell::values::{BasicValueEnum, InstructionOpcode};
use inkwell::IntPredicate;
use petgraph::graph::{DiGraph, NodeIndex};

pub mod binary;
pub mod branch;
pub mod extend;
pub mod phi;
pub mod preprocessing;
pub mod types;

#[derive(Default, Clone, Debug)]
pub struct CairoFunctionBuilder<'ctx> {
    pub(crate) bb_loop: HashSet<BasicBlock<'ctx>>,
    pub(crate) variables: HashMap<BasicValueEnum<'ctx>, String>,
    pub(crate) bb_graph: DiGraph<BasicBlock<'ctx>, ()>,
    pub(crate) node_id_from_name: HashMap<BasicBlock<'ctx>, NodeIndex<u32>>,
    pub(crate) function: CairoFunction,
    pub(crate) phis_bblock: HashSet<BasicBlock<'ctx>>,
    pub(crate) bblock_variables: HashMap<BasicBlock<'ctx>, HashMap<BasicValueEnum<'ctx>, String>>,
    pub(crate) if_blocks: HashMap<BasicBlock<'ctx>, BasicValueEnum<'ctx>>,
    pub(crate) else_blocks: HashSet<BasicBlock<'ctx>>,
    pub(crate) return_block: Option<BasicBlock<'ctx>>,
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

    pub fn push_body_line(&mut self, line: String) {
        self.function.body.push_line(line)
    }
    pub fn get_name(&self, name: &CStr) -> String {
        (!name.is_empty())
            .then(|| name.to_str().expect("Variable name for binary op should be uft-8").replace('.', "_"))
            .unwrap_or_else(|| format!("var{}", self.variables.keys().count()))
    }

    /// Set all the basic block booleans to the correct value. This should be used at the end of a
    /// basic block before jump to know from which basic block we're coming from at runtime.
    pub fn set_basic_block_booleans(&mut self, bb: &BasicBlock<'ctx>) {
        // If we're not in the last basic block set all the booleans to the right value to know what basic
        // block we were in so we can process the phi instruction can work correctly
        if self.return_block.is_some_and(|bblock| bblock != *bb) || self.return_block.is_none() {
            for bblock in self.phis_bblock.iter() {
                // If we were in this basic block
                if self.get_name(bblock.get_name()) == self.get_name(bb.get_name()) {
                    let code_line = format!("is_from_{} = true;", self.get_name(bblock.get_name()),);
                    self.function.body.push_line(code_line);
                } else {
                    // if we were not in this basic block
                    let code_line = format!("is_from_{} = false;", self.get_name(bblock.get_name()),);
                    self.function.body.push_line(code_line);
                }
            }
        }
    }
    /// Process a basic block and convert it to cairo. It will call itself recursively through the
    /// [CairoFunctionBuilder::process_branch] function.
    pub fn process_basic_block(&mut self, bb: &BasicBlock<'ctx>) {
        // Boolean that let's us know if we need to wrap our basic block translation with a loop { bbcode };
        let is_loop = self.bb_loop.contains(bb);
        // Is this block the else clause of an if/else
        let is_else = self.else_blocks.contains(bb);
        // TODO(Lucas): in preprocess function declare all the variables that will be in subscopes and then
        // stop worrying about it + only use is_subscope
        let _is_subscope = is_loop || is_else || self.if_blocks.contains_key(bb);

        // Prepare for loops/if/else
        self.prepare_new_scopes(bb, &is_else, &is_loop);

        // Iterate over each instruction of the basic block. 1 instruction == 1 LLVM code line
        for instruction in bb.get_instructions() {
            // Get the opcode of the instruction
            let code_line = match instruction.get_opcode() {
                InstructionOpcode::Add => self.process_binary_int_op(&instruction, "+", bb),
                InstructionOpcode::Sub => self.process_binary_int_op(&instruction, "-", bb),
                InstructionOpcode::Return => self.process_return(&instruction),
                InstructionOpcode::ICmp => {
                    // we just matched on ICmp so it will never fail
                    match instruction.get_icmp_predicate().unwrap() {
                        IntPredicate::EQ => self.process_binary_int_op(&instruction, "==", bb),
                        IntPredicate::NE => self.process_binary_int_op(&instruction, "!=", bb),
                        IntPredicate::ULT => self.process_binary_int_op(&instruction, "<", bb),
                        _ => "".to_owned(),
                    }
                }
                InstructionOpcode::Br => self.process_branch(&instruction, bb, &is_loop, &is_else),
                InstructionOpcode::ZExt => self.process_zext(&instruction, &is_loop),
                InstructionOpcode::Phi => self.process_phi(&instruction, bb),
                _ => "".to_owned(),
            };
            self.push_body_line(code_line);
            if is_loop && instruction.get_opcode() == InstructionOpcode::Br {
                self.close_scopes(bb, &is_else, &is_loop);
            }
            // Add the line to the function body
        }
    }
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
