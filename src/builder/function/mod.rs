use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use inkwell::basic_block::BasicBlock;
use inkwell::values::BasicValueEnum;
use petgraph::graph::{DiGraph, NodeIndex};

pub mod binary;
pub mod branch;
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
