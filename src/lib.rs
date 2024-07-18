use std::path::Path;

use builder::CairoBuilder;
use inkwell::context::Context;
use inkwell::memory_buffer::MemoryBuffer;

pub mod builder;

pub fn compile(path: &str) {
    // Initialize LLVM context
    let context = Context::create();
    // Parse the LLVM IR
    let module = context
        .create_module_from_ir(MemoryBuffer::create_from_file(Path::new(path)).expect("Failed to load llvm file"))
        .expect("Failed to parse LLVM IR");

    // Create a cairo builder that will hold all the translated code.
    let mut builder = CairoBuilder::default();
    // For each function on the llvm file translate it to cairo. Append the code to our file.
    module.get_functions().for_each(|func| {
        let translated_func = builder.translate_function(&func);
        builder.functions.push_function(translated_func);
    });
    // println!("Compiling LLVM IR {}", module.to_string());
    println!("Cairo code:\n{}", builder.functions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compiles() {
        compile("examples/add/add.ll");
    }
}
