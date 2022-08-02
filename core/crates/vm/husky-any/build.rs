use husky_rust_code_repr::{registration::NonPrimitiveTypeRegistration, BuildCodeGenStart};
use husky_write_utils::w;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs::File};

static FILENAME: &str = &"husky_any";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let husky_dir = "/home/xiyuzhai/Documents/husky";
    let rust_code_path = format!(
        "{}/core/crates/vm/husky-any/src/__rust_code_gen__.rs",
        husky_dir
    );
    write_rust_code(&rust_code_path).unwrap();
}

pub static NONPRIMITIVE_TYPES: &'static [&'static str] =
    &["VirtualStruct", "VirtualVec", "VirtualCyclicSlice"];

pub fn write_rust_code(rust_code_path: &str) -> std::io::Result<()> {
    let mut f = File::create(rust_code_path)
        .expect(&format!("rust code path {rust_code_path} doesn't exist"));
    w!(f; BuildCodeGenStart);
    //     w!(f; r#"
    // use crate::cv::mnist::BinaryImage28;
    // use crate::cv::mnist::BinaryGrid28;
    // "#);
    for ty in NONPRIMITIVE_TYPES {
        w!(f; NonPrimitiveTypeRegistration { ty })
    }
    Ok(())
}
