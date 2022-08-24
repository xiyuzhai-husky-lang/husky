use husky_rust_code_repr::{registration::NonPrimitiveTypeRegistration, BuildCodeGenStart};
use husky_write_utils::w;
use std::env;
use std::fmt::Write;
use std::path::PathBuf;

static FILENAME: &str = "husky_ml_datasets";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let rust_code_path: PathBuf = "src/__rust_code_gen__.rs".into();
    husky_io_utils::diff_write(&rust_code_path, &gen_rust_code().unwrap());
}

pub static NONPRIMITIVE_TYPES: &'static [&'static str] =
    &["MnistLabel", "BinaryImage28", "BinaryGrid28", "Dataset"];

pub fn gen_rust_code() -> Result<String, std::fmt::Error> {
    let mut code = String::new();
    w!(code; BuildCodeGenStart);
    w!(code; r#"
use crate::cv::mnist::*;
"#);
    for ty in NONPRIMITIVE_TYPES {
        w!(code; NonPrimitiveTypeRegistration { ty })
    }
    Ok(code)
}
