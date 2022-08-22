use husky_rust_code_repr::{registration::*, *};
use husky_write_utils::w;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, fs::File};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    write_rust_code("src/__rust_code_gen__.rs").unwrap();
}

pub fn write_rust_code(rust_path: &str) -> std::io::Result<()> {
    let mut f = File::create(rust_path).expect(&format!("rust path {rust_path} doesn't exist"));
    w!(f; BuildCodeGenStart);
    for ty in PRIMITIVE_TYPES {
        w!(f; RootPrimitiveTypeRegistration { ty })
    }
    for ty in NONPRIMITIVE_BUILTIN_TYPES {
        w!(f; NonPrimitiveTypeRegistration { ty })
    }
    Ok(())
}

pub static PRIMITIVE_TYPES: &'static [&'static str] =
    &["void", "bool", "i32", "i64", "b32", "b64", "f32", "f64"];

pub static NONPRIMITIVE_BUILTIN_TYPES: &'static [&'static str] =
    &["__VirtualFunction", "__VirtualEnum"];
