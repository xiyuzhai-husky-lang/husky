mod build;
mod c_header;
mod rust_code;

use build::build_c;
use c_header::write_c_header;
use c_source::write_c_source;
use rust_code::write_rust_code;
use std::io::prelude::*;
use std::process::Command;
use std::{fs::File, path::Path};

pub static PRIMITIVE_TYPES: &'static [&'static str] =
    &["void", "bool", "i32", "i64", "b32", "b64", "f32", "f64"];

pub static NONPRIMITIVE_BUILTIN_TYPES: &'static [&'static str] = &[];
// &[
//     "BinaryImage28",
//     "BinaryGrid28",
//     "Dataset",
//     "VirtualVec",
//     "VirtualCyclicSlice",
//     "VirtualStruct",
// ];

pub fn gen_vm_interface_csrc(out_dir: &str) {
    let c_header_path = format!("{}/husky_vm_interface.h", out_dir);
    let c_source_path = format!("{}/husky_vm_interface.c", out_dir);
    let husky_dir = std::env::var("HUSKY_DIR").expect("HUSKY_DIR is not set");
    let rust_path = format!(
        "{}/core/crates/vm/husky-vm-interface/src/__rust_code_gen__.rs",
        husky_dir
    );
    write_c_header(&c_header_path).unwrap();
    write_c_source(&c_source_path).unwrap();
    write_rust_code(&rust_path).unwrap();
    build_c(&out_dir);
}

mod c_source;

#[test]
fn try_gen_vm_interface_csrc() {
    std::env::set_var("RUST_BACKTRACE", "0");
    // this only works on my comuter
    std::env::set_var("HUSKY_DIR", "/home/xiyuzhai/Documents/husky");
    gen_vm_interface_csrc(
        "/home/xiyuzhai/Documents/husky/core/crates/vm/husky-vm-interface/__c_code_gen_test__",
    )
}
