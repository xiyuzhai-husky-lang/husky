mod impl_cargo_build;
mod impl_cargo_fmt;
mod impl_clean;
mod impl_dir;
mod impl_sync_code;
mod impl_transcribe_rust;

use husky_compile_dir::{getx_child_dir, mkdir};
use husky_compile_time::*;
use husky_entity_semantics::{EntityDefn, EntityDefnVariant};
use husky_file::FilePtr;
use husky_linkage_table::LinkageTableConfig;
use husky_print_utils::*;
use husky_root_static_defn::__resolve_root_defn;
use husky_word::snake_to_dash;
use io_utils::diff_write;
use path_utils::collect_all_package_dirs;
use std::{
    path::{Path, PathBuf},
    time::Instant,
};

pub struct CompilerInstance {
    dir: PathBuf,
    husky_dir: String,
    verbose: bool,
}

impl CompilerInstance {
    pub fn new(husky_dir: String, verbose: bool, dir: PathBuf) -> Self {
        // let flags = flags::husky-compilerompilerFlags::from_env().expect("invalid arguments");
        Self {
            dir,
            husky_dir,
            verbose,
        }
    }

    pub fn compile_all(&self) {
        use husky_print_utils::*;
        let package_dirs = collect_all_package_dirs(&self.dir);
        println!(
            "{GREEN}\x1B[1mCompiling{RESET} {} 🐺 packages:",
            package_dirs.len()
        );
        for package_dir in package_dirs {
            // compile via rust
            println!(
                "   {GREEN}\x1B[1mCompiling{RESET} 🐺 package `{}` ({})",
                package_dir.file_name().unwrap().to_str().unwrap(),
                package_dir.as_os_str().to_str().unwrap(),
            );
            let now = Instant::now();
            self.transcribe_package_in_rust(&package_dir);
            self.cargo_fmt(&package_dir);
            self.sync_rust_code(&package_dir);
            self.cargo_build(&package_dir);
            self.clean_rust_gen_cache(&package_dir);
            println!(
                "    {GREEN}\x1B[1mFinished{RESET} in {:.2} seconds.",
                now.elapsed().as_millis() as f32 / 1000.
            );
        }
    }
}
