mod impl_cargo_build;
mod impl_cargo_check;
mod impl_cargo_fmt;
mod impl_clean;
mod impl_dir;
mod impl_sync_code;
mod impl_transcribe_rust;

use husky_compile_dir::{getx_child_dir, mkdir};
use husky_comptime::*;
use husky_entity_semantics::{EntityDefn, EntityDefnVariant};
use husky_file::FilePtr;
use husky_io_utils::diff_write;
use husky_linkage_table::LinkageTableConfig;
use husky_path_utils::collect_all_package_dirs;
use husky_print_utils::*;
use husky_root_static_defn::__resolve_root_defn;
use husky_word::snake_to_dash;
use relative_path::RelativePathBuf;
use std::{
    path::{Path, PathBuf},
    time::Instant,
};

pub struct CompilerInstance {
    dir: RelativePathBuf,
    verbose: bool,
}

impl CompilerInstance {
    pub fn new(verbose: bool, dir: RelativePathBuf) -> Self {
        Self { dir, verbose }
    }

    pub fn compile_all(&self) {
        use husky_print_utils::*;
        let package_dirs = collect_all_package_dirs(&self.dir.to_path(""));
        println!(
            "{GREEN}\x1B[1mCompiling{RESET} {} 🐺 packages:",
            package_dirs.len()
        );
        for package_dir in package_dirs.iter() {
            // transcribe to rust
            println!(
                "   {GREEN}\x1B[1mTranscribing{RESET} 🐺 package `{}` ({})",
                package_dir.file_name().unwrap().to_str().unwrap(),
                package_dir.as_os_str().to_str().unwrap(),
            );
            let now = Instant::now();
            self.transcribe_package_in_rust(&package_dir);
            self.cargo_fmt(&package_dir);
            self.sync_rust_code(&package_dir);
            self.cargo_check(&package_dir);
            println!(
                "    {GREEN}\x1B[1mFinished{RESET} in {:.2} seconds.",
                now.elapsed().as_millis() as f32 / 1000.
            );
        }
        for package_dir in package_dirs {
            println!(
                "   {GREEN}\x1B[1mCompiling{RESET} 🐺 package `{}` ({})",
                package_dir.file_name().unwrap().to_str().unwrap(),
                package_dir.as_os_str().to_str().unwrap(),
            );
            let now = Instant::now();
            self.cargo_build(&package_dir);
            self.clean_rust_gen_cache(&package_dir);
            println!(
                "    {GREEN}\x1B[1mFinished{RESET} in {:.2} seconds.",
                now.elapsed().as_millis() as f32 / 1000.
            );
        }
    }
}
