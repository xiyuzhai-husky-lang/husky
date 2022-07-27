use husky_datasets_static_defn_code_gen::gen_ml_datasets_code;
use std::env;
use std::path::PathBuf;

fn main() {
    let c_code_gen_dir = format!(
        "{}/core/crates/static/domains/ml/husky-datasets-static-defn/__c_code_gen__",
        std::env::var("HUSKY_DIR").expect("env not set")
    );
    gen_ml_datasets_code(&c_code_gen_dir);
    // msg_once!("todo")
    // let husky_dir = std::env::var("HUSKY_DIR").expect("HUSKY_DIR is not set");
    // let husky_core_dir = format!("{}/core", husky_dir);

    // // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search={}/lib", husky_core_dir);

    // // Tell cargo to tell rustc to link the husky_vm
    // // shared library.
    // println!("cargo:rustc-link-lib=husky_vm");

    // // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed={}/csrc/husky_vm.c", husky_core_dir);
    // // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed={}/csrc/husky_vm.h", husky_core_dir);

    // // The bindgen::Builder is the main entry point
    // // to bindgen, and lets you build up options for
    // // the resulting bindings.
    // let bindings = bindgen::Builder::default()
    //     // The input header we would like to generate
    //     // bindings for.
    //     .header(format!("{}/csrc/husky_vm.h", husky_core_dir))
    //     // Tell cargo to invalidate the built crate whenever any of the
    //     // included header files changed.
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     // Finish the builder and generate the bindings.
    //     .generate()
    //     // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}
