#!/bin/bash

# Check if a file is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <file_name>"
    exit 1
fi

# Get the Rust file from the first argument
rust_file="examples/$1/$1.rs"
out_file="examples/$1/$1.ll"

# Check if the file exists
if [ ! -f "$rust_file" ]; then
    echo "File not found: $rust_file"
    exit 1
fi


# Compile the Rust file to LLVM IR
rustc --crate-type=lib -C opt-level=3 --emit=llvm-ir  "$rust_file" -o $out_file

echo "LLVM IR generated and saved to $out_file"
