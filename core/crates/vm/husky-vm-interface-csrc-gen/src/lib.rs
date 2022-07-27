mod build;
mod c_header;
mod rust_codegen;

use std::io::prelude::*;
use std::process::Command;
use std::{fs::File, path::Path};

use build::build_c;
use c_header::write_header;
use c_source::write_source;

pub static PRIMITIVE_TYPES: &'static [&'static str] =
    &["void", "bool", "i32", "i64", "b32", "b64", "f32", "f64"];

pub static NONPRIMITIVE_BUILTIN_TYPES: &'static [&'static str] = &[
    "BinaryImage28",
    "BinaryGrid28",
    "Dataset",
    "VirtualVec",
    "VirtualCyclicSlice",
    "VirtualStruct",
];

pub fn gen_vm_interface_csrc() {
    let husky_dir = std::env::var("HUSKY_DIR").expect("HUSKY_DIR not set");
    let c_codegen_dir = format!("{}/core/csrc-gen", husky_dir);
    let build_dir = format!("{}/core/build", husky_dir);
    let lib_dir = format!("{}/core/build/lib", husky_dir);
    let c_header_path = format!("{}/husky_vm_interface.h", c_codegen_dir);
    let c_source_path = format!("{}/husky_vm_interface.c", c_codegen_dir);
    write_header(&c_header_path).unwrap();
    write_source(&c_source_path).unwrap();
    build_c(&c_codegen_dir, &build_dir, &lib_dir);
}

mod c_source;

#[test]
fn try_gen_vm_interface_csrc() {
    // this only works on my comuter
    std::env::set_var("HUSKY_DIR", "/home/xiyuzhai/Documents/husky");
    std::env::set_var("RUST_BACKTRACE", "0");
    gen_vm_interface_csrc()
}
