use std::path::Path;

use inkwell::{context::Context, memory_buffer::MemoryBuffer};

pub fn compile(path: &str) {
    // Initialize LLVM context
    let context = Context::create();
    // Parse the LLVM IR
    let module = context
        .create_module_from_ir(
            MemoryBuffer::create_from_file(Path::new(path)).expect("Failed to load llvm file"),
        )
        .expect("Failed to parse LLVM IR");
    println!("Compiling LLVM IR {}", module.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compiles() {
        compile("examples/add/add.ll");
    }
}
