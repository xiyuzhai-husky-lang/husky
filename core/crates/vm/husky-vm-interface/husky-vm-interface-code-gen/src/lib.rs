mod rust_code;

use rust_code::write_rust_code;
use std::io::prelude::*;
use std::process::Command;
use std::{fs::File, path::Path};

pub static PRIMITIVE_TYPES: &'static [&'static str] =
    &["void", "bool", "i32", "i64", "b32", "b64", "f32", "f64"];

pub static NONPRIMITIVE_BUILTIN_TYPES: &'static [&'static str] =
    &["__VirtualFunction", "__VirtualEnum"];

pub fn gen_vm_interface_code() {
    write_rust_code("src/__rust_code_gen__.rs").unwrap();
}
