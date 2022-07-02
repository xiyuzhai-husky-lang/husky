mod flags;

use __husky_root::__root_defn;
use husky_compile_dir::{get_or_create_child_dir, get_rust_dir, mkdir};
use husky_compile_time::*;
use husky_entity_semantics::{EntityDefn, EntityDefnVariant};
use husky_file::FilePtr;
use io_utils::diff_write;
use path_utils::collect_all_package_dirs;
use print_utils::*;
use std::path::{Path, PathBuf};

pub fn compile_all(dir: PathBuf) {
    let pack_dirs = collect_all_package_dirs(dir);
    for pack_dir in pack_dirs {
        compile_package(pack_dir);
    }
}

pub fn compile_package(package_dir: PathBuf) {
    let mut compile_time = HuskyCompileTime::new(__root_defn);
    compile_time.load_package(&package_dir);
    let main_file = compile_time.unique_main_file();
    p!(package_dir);
    let package = compile_time.package(main_file).unwrap();
    let rust_dir = get_rust_dir(&package);
    let code_snapshot_dir = get_or_create_child_dir(&rust_dir, "snapshot");
    let src_dir = get_or_create_child_dir(&rust_dir, "src");
    let bin_dir = get_or_create_child_dir(&src_dir, "bin");

    save_code_snapshot(&compile_time, &code_snapshot_dir, main_file);

    // Cargo.toml
    diff_write(
        &rust_dir.join("Cargo.toml"),
        &compile_time.cargo_toml_content(main_file),
    );

    // lib.rs
    diff_write(
        &src_dir.join("lib.rs"),
        &compile_time.rust_lib_rs_content(main_file),
    );

    // __init__.rs
    diff_write(
        &src_dir.join("__init__.rs"),
        &compile_time.rust_init_rs_content(main_file),
    );

    for module in package.subentities.iter() {
        let module_name = module.ident.as_str();
        compile_maybe_module(
            &compile_time,
            src_dir.join(format!("{module_name}.rs")),
            module,
        )
    }

    // bin/main.rs
    diff_write(
        &bin_dir.join("main.rs"),
        &compile_time.rust_bin_main_rs_content(main_file),
    );
}

fn compile_maybe_module(compile_time: &HuskyCompileTime, path: PathBuf, module: &EntityDefn) {
    match module.variant {
        EntityDefnVariant::Module { .. } => (),
        _ => return,
    }
    diff_write(&path, &compile_time.rust_mod_rs_content(module.base_route));
    let dir = path.with_extension("");
    for submodule in module.subentities.iter() {
        let submodule_name = submodule.ident.as_str();
        compile_maybe_module(
            compile_time,
            dir.join(format!("{submodule_name}.rs")),
            submodule,
        )
    }
}

fn save_code_snapshot(compile_time: &HuskyCompileTime, snapshot_dir: &Path, main_file: FilePtr) {
    diff_write(
        &snapshot_dir.join("main.hsk"),
        compile_time.file_content(main_file).to_str().unwrap(),
    );
    emsg_once!("save snapshot of other files")
}
