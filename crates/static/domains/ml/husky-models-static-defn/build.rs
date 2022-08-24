use husky_rust_code_repr::{registration::NonPrimitiveTypeRegistration, BuildCodeGenStart};
use husky_write_utils::w;
use std::env;
use std::fmt::Write;
use std::path::PathBuf;

static FILENAME: &str = &"husky_ml_models";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let path: PathBuf = "src/__rust_code_gen__.rs".into();
    husky_io_utils::diff_write(&path, &gen_rust_code().unwrap());
}

pub static NONPRIMITIVE_TYPES: &'static [&'static str] = &["NaiveI32Internal"];

pub fn gen_rust_code() -> Result<String, std::fmt::Error> {
    let mut code = String::new();
    w!(code; BuildCodeGenStart);
    w!(code; r#"
use crate::{*, naive::*};
use husky_vm::*;

"#);
    for ty in NONPRIMITIVE_TYPES {
        w!(code; NonPrimitiveTypeRegistration { ty })
    }
    Ok(code)
}
