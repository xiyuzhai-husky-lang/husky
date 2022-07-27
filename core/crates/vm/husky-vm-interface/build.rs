use std::env;
use std::path::PathBuf;

use husky_vm_interface_codegen::gen_vm_interface_csrc;

fn main() {
    let c_codegen_dir = format!(
        "{}/core/crates/vm/husky-vm-interface/__c_codegen__",
        std::env::var("HUSKY_DIR").expect("env not set")
    );
    gen_vm_interface_csrc(&c_codegen_dir);

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", c_codegen_dir);

    // Tell cargo to tell rustc to link the husky_vm
    // shared library.
    println!("cargo:rustc-link-lib=husky_vm_interface");

    // // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!(
    //     "cargo:rerun-if-changed={}/__c_codegen__/husky_vm_interface.c",
    //     husky_core_dir
    // );
    // // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed={}/csrc/husky_vm.h", husky_core_dir);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(format!("{}/husky_vm_interface.h", c_codegen_dir))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
