mod dir;
pub mod flags;

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
use std::path::{Path, PathBuf};

pub struct CompilerInstance {
    packages_dir: PathBuf,
    husky_dir: String,
}

impl CompilerInstance {
    pub fn from_env() -> Self {
        let flags = flags::HuskyCompilerFlags::from_env().expect("invalid arguments");
        Self {
            packages_dir: flags.packages_dir,
            husky_dir: std::env::var("HUSKY_DIR").expect("env not set"),
        }
    }

    pub fn compile_all(&self) {
        let pack_dirs = collect_all_package_dirs(&self.packages_dir);
        for pack_dir in pack_dirs {
            self.compile_package(pack_dir);
        }
    }

    pub fn compile_package(&self, package_dir: PathBuf) {
        let mut compile_time = HuskyCompileTime::new(HuskyCompileTimeConfig {
            __resolve_root_defn,
            linkage_table: LinkageTableConfig {
                warn_missing_linkage: false,
            },
        });
        compile_time.load_package(&package_dir);
        let main_file = compile_time.unique_main_file();
        let package = compile_time.package(main_file).unwrap();
        let rust_dir = self.getx_rust_gen_cache_dir(&package);
        let husky_code_snapshot_dir = self.getx_husky_code_snapshot_dir(&package);
        let src_dir = getx_child_dir(&rust_dir, "src");

        self.save_husky_code_snapshot(
            &compile_time,
            &husky_code_snapshot_dir.join("main.hsk"),
            main_file,
        );

        let cargo_config_path = getx_child_dir(&rust_dir, ".cargo").join("config.toml");
        diff_write(
            &cargo_config_path,
            r#"
            # Add the contents of this file to `config.toml` to enable "fast build" configuration. Please read the notes below.
# NOTE: For maximum performance, build using a nightly compiler
# If you are using rust stable, remove the "-Zshare-generics=y" below.
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-Awarnings", "-Clink-arg=-fuse-ld=lld", "-Zshare-generics=y"]

# NOTE: you must manually install https://github.com/michaeleisel/zld on mac. you can easily do this with the "brew" package manager:
# `brew install michaeleisel/zld/zld`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld", "-Zshare-generics=y"]

[target.aarch64-apple-darwin]
rustflags = [
    "-C",
    "link-arg=-fuse-ld=/opt/homebrew/bin/zld",
    "-Zshare-generics=y"
]

[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
rustflags = ["-Zshare-generics=n"]

# Optional: Uncommenting the following improves compile times, but reduces the amount of debug info to 'line number tables only'
# In most cases the gains are negligible, but if you are on macos and have slow compile times you should see significant gains.
[profile.dev]
debug = 1
"#,
        );

        // Cargo.toml
        diff_write(
            &rust_dir.join("Cargo.toml"),
            &compile_time.cargo_toml_content(main_file, &self.husky_dir),
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
            self.compile_maybe_module(
                &compile_time,
                src_dir.join(format!("{module_name}.rs")),
                &husky_code_snapshot_dir.join(format!("{module_name}.hsk")),
                module,
            )
        }
    }

    fn compile_maybe_module(
        &self,
        compile_time: &HuskyCompileTime,
        rust_code_path: PathBuf,
        husky_code_snapshot_path: &Path,
        module: &EntityDefn,
    ) {
        match module.variant {
            EntityDefnVariant::Module { .. } => (),
            _ => return,
        }
        diff_write(
            &rust_code_path,
            &compile_time.rust_mod_rs_content(module.base_route),
        );
        self.save_husky_code_snapshot(compile_time, husky_code_snapshot_path, module.file);
        let rust_code_dir = rust_code_path.with_extension("");
        let husky_code_snapshot_dir = husky_code_snapshot_path.with_extension("");
        mkdir(&husky_code_snapshot_dir);
        for submodule in module.subentities.iter() {
            let submodule_name = submodule.ident.as_str();
            self.compile_maybe_module(
                compile_time,
                rust_code_dir.join(format!("{submodule_name}.rs")),
                &husky_code_snapshot_dir.join(format!("{submodule_name}.hsk")),
                submodule,
            )
        }
    }

    fn save_husky_code_snapshot(
        &self,
        compile_time: &HuskyCompileTime,
        husky_code_snapshot_path: &Path,
        main_file: FilePtr,
    ) {
        diff_write(
            husky_code_snapshot_path,
            compile_time.file_content(main_file).to_str().unwrap(),
        );
    }
}
