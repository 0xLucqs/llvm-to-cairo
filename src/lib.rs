use std::path::Path;

use builder::{CairoBuilder, CairoFunctions};
use inkwell::context::Context;
use inkwell::memory_buffer::MemoryBuffer;

pub mod builder;

pub fn compile(path: &str) -> CairoFunctions {
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
    builder.functions
}

#[cfg(test)]
mod tests {
    use builder::function::{CairoFunctionBody, CairoFunctionSignature, CairoParameter};

    use super::*;

    #[test]
    fn it_compiles() {
        println!("Cairo code:\n{}", compile("examples/increment/increment.ll"));
    }

    #[test]
    fn test_add() {
        let expected_name = "add".to_owned();
        let expected_return_type = "i64".to_owned();
        let expected_params = vec![
            CairoParameter::new("left".to_owned(), "i64".to_owned()),
            CairoParameter::new("right".to_owned(), "i64".to_owned()),
        ];
        let code = compile("examples/add/add.ll");

        // Check number of functions generated
        assert_eq!(code.count_functions(), 1, "Add function should generate exactly 1 function");
        let function = code.functions().first().unwrap();
        // Check function signature
        assert_eq!(
            function.signature,
            CairoFunctionSignature::new(expected_name, expected_params, expected_return_type)
        );

        // Check function body
        assert_eq!(
            function.body,
            CairoFunctionBody::new(vec!["let _0 = right + left;".to_owned(), "return _0;".to_owned()])
        );
    }

    #[test]
    fn test_increment() {
        let expected_name = "increment".to_owned();
        let expected_return_type = "i128".to_owned();
        let expected_params = vec![CairoParameter::new("left".to_owned(), "i128".to_owned())];
        let code = compile("examples/increment/increment.ll");

        // Check number of functions generated
        assert_eq!(code.count_functions(), 1, "Add function should generate exactly 1 function");
        let function = code.functions().first().unwrap();
        // Check function signature
        assert_eq!(
            function.signature,
            CairoFunctionSignature::new(expected_name, expected_params, expected_return_type)
        );

        // Check function body
        assert_eq!(
            function.body,
            CairoFunctionBody::new(vec![
                "let _0 = left + 170141183460469231731687303715884105727_i128;".to_owned(),
                "return _0;".to_owned()
            ])
        );
    }
}
