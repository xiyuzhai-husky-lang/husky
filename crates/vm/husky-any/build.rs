// use husky_rust_code_repr::{registration::NonPrimitiveTypeRegistration, BuildCodeGenStart};
// use husky_write_utils::w;
// use std::fmt::Write;
// use std::path::PathBuf;

fn main() {
    // println!("cargo:rerun-if-changed=build.rs");
    // let path: PathBuf = "src/__rust_code_gen__.rs".into();
    // husky_io_utils::diff_write(&path, &gen_rust_code().unwrap(), true);
}

// pub static NONPRIMITIVE_TYPES: &'static [&'static str] = &[
//     "DeprecatedVirtualStruct",
//     "DeprecatedVirtualVec",
//     "DeprecatedVirtualCyclicSlice",
//     "VisualData",
// ];

// pub fn gen_rust_code() -> Result<String, std::fmt::Error> {
//     let mut code = String::new();
//     w!(code; BuildCodeGenStart);
//     w!(code; r#"
//     use husky_trace_protocol::VisualData;
//     "#);
//     for ty in NONPRIMITIVE_TYPES {
//         w!(code; NonPrimitiveTypeRegistration { ty })
//     }
//     Ok(code)
// }
