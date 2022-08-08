use std::env;
use std::path::PathBuf;

use husky_vm_interface_code_gen::gen_vm_interface_code;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    gen_vm_interface_code();
}
