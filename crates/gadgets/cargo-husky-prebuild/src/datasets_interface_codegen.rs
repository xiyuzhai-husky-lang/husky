use husky_rust_code_repr::{registration::NonPrimitiveTypeRegistration, BuildCodeGenStart};
use husky_write_utils::w;
use std::fmt::Write;
use std::path::PathBuf;

pub(crate) fn write_datasets_interface_codegen() {
    let rust_code_path: PathBuf =
        "crates/static/domains/ml/husky-datasets-interface/src/__rust_code_gen__.rs".into();
    husky_io_utils::diff_write(&rust_code_path, &gen_rust_code().unwrap(), true);
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
