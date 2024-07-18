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
        println!("Cairo code:\n{}", compile("examples/fib/fib.ll"));
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

    #[test]
    fn test_fib() {
        let expected_name = "fib".to_owned();
        let expected_return_type = "i32".to_owned();
        let expected_params = vec![
            CairoParameter::new("a".to_owned(), "i32".to_owned()),
            CairoParameter::new("b".to_owned(), "i32".to_owned()),
            CairoParameter::new("n".to_owned(), "i32".to_owned()),
        ];
        let code = compile("examples/fib/fib.ll");

        // Check number of functions generated
        assert_eq!(code.count_functions(), 1, "Add function should generate exactly 1 function");
        let function = code.functions().first().unwrap();
        // Check function signature
        assert_eq!(
            function.signature,
            CairoFunctionSignature::new(expected_name, expected_params, expected_return_type)
        );

        // Check function body
        pretty_assertions::assert_eq!(
            function.body,
            CairoFunctionBody::new(vec![
                "let mut is_from_bb2 = false;".to_owned(),
                "let mut is_from_start = false;".to_owned(),
                "let result = n == 0_i32;".to_owned(),
                "".to_owned(),
                "is_from_bb2 = false;".to_owned(),
                "is_from_start = true;".to_owned(),
                "let mut ntr3 = 0_i32;".to_owned(),
                "let mut btr2 = 0_i32;".to_owned(),
                "let mut atr1 = 0_i32;".to_owned(),
                "let mut _4 = 0_i32;".to_owned(),
                "let mut _5 = 0_i32;".to_owned(),
                "let mut var9 = false;".to_owned(),
                "if !result {".to_owned(),
                "loop {".to_owned(),
                "ntr3 = if is_from_bb2 { _5 } else if is_from_start { n } else { panic!(\"There is a bug in the \
                 compiler please report it\")};"
                    .to_owned(),
                "btr2 = if is_from_bb2 { _4 } else if is_from_start { b } else { panic!(\"There is a bug in the \
                 compiler please report it\")};"
                    .to_owned(),
                "atr1 = if is_from_bb2 { btr2 } else if is_from_start { a } else { panic!(\"There is a bug in the \
                 compiler please report it\")};"
                    .to_owned(),
                "_4 = btr2 + atr1;".to_owned(),
                "_5 = ntr3 + -1_i32;".to_owned(),
                "var9 = _5 == 0_i32;".to_owned(),
                "if var9\n{break;}".to_owned(),
                "is_from_bb2 = true;".to_owned(),
                "is_from_start = false;".to_owned(),
                "};".to_owned(),
                "let mut atrlcssa = 0_i32;".to_owned(),
                "}".to_owned(),
                "let atrlcssa = if is_from_start { a } else if is_from_bb2 { btr2 } else { panic!(\"There is a bug in \
                 the compiler please report it\")};"
                    .to_owned(),
                "return btr2;".to_owned()
            ]),
        );
    }
}
