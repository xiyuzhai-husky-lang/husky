use crate::*;

impl CompilerInstance {
    pub(crate) fn transcribe_package_in_rust(&self, package_dir: &Path) {
        todo!()
        //         let mut comptime = HuskyComptime::new(ComptimeConfig {
        //             __resolve_root_defn,
        //             linkage_table: LinkageTableConfig {
        //                 warn_missing_linkage: false,
        //             },
        //             package_dir: package_dir.to_path_buf(),
        //         });
        //         let target_entrance = comptime.unique_main_file();
        //         let all_diagnostics = comptime.all_diagnostics();
        //         if all_diagnostics.len() > 0 {
        //             panic!("{:#?}", all_diagnostics);
        //         }
        //         let package = comptime.package(target_entrance).unwrap();
        //         let rust_dir = self.getx_rust_gen_cache_dir(&package);
        //         let husky_code_snapshot_dir = self.getx_husky_code_snapshot_dir(&package);
        //         let src_dir = getx_child_dir(&rust_dir, "src");

        //         self.save_husky_code_snapshot(
        //             &comptime,
        //             &husky_code_snapshot_dir.join("main.hsy"),
        //             target_entrance,
        //         );

        //         let cargo_config_path = getx_child_dir(&rust_dir, ".cargo").join("config.toml");
        //         diff_write(
        //             &cargo_config_path,
        //             r#"
        //             # Add the contents of this file to `config.toml` to enable "fast build" configuration. Please read the notes below.
        // # NOTE: For maximum performance, build using a nightly compiler
        // # If you are using rust stable, remove the "-Zshare-generics=y" below.
        // [target.x86_64-unknown-linux-gnu]
        // linker = "clang"
        // rustflags = ["-Awarnings", "-Clink-arg=-fuse-ld=lld", "-Zshare-generics=y"]

        // # NOTE: you must manually install https://github.com/michaeleisel/zld on mac. you can easily do this with the "brew" package manager:
        // # `brew install michaeleisel/zld/zld`
        // [target.x86_64-apple-darwin]
        // rustflags = [
        // #    "-C",
        // #    "link-arg=-fuse-ld=/usr/local/bin/zld",
        //     "-Zshare-generics=y"
        // ]

        // [target.aarch64-apple-darwin]
        // rustflags = [
        // #    "-C",
        // #    "link-arg=-fuse-ld=/opt/homebrew/bin/zld",
        //     "-Zshare-generics=y"
        // ]

        // [target.x86_64-pc-windows-msvc]
        // linker = "rust-lld.exe"
        // rustflags = ["-Zshare-generics=n"]

        // # Optional: Uncommenting the following improves compile times, but reduces the amount of debug info to 'line number tables only'
        // # In most cases the gains are negligible, but if you are on macos and have slow compile times you should see significant gains.
        // [profile.dev]
        // debug = 1
        // "#,
        //             self.rust_codegen_cache_diff_write_verbose(),
        //         );

        //         let husky_dir: RelativePathBuf = "./".into();
        //         let husky_dir = RelativePathBuf::from_path(package_dir.join("__rust_gen__"))
        //             .unwrap()
        //             .relative(&husky_dir);

        //         // Cargo.toml
        //         diff_write(
        //             &rust_dir.join("Cargo.toml"),
        //             &comptime.cargo_toml_content(target_entrance, &husky_dir.to_path("")),
        //             self.rust_codegen_cache_diff_write_verbose(),
        //         );

        //         // lib.rs
        //         diff_write(
        //             &src_dir.join("lib.rs"),
        //             &comptime.rust_lib_rs_content(target_entrance),
        //             self.rust_codegen_cache_diff_write_verbose(),
        //         );

        //         // __init__.rs
        //         diff_write(
        //             &src_dir.join("__init__.rs"),
        //             &comptime.rust_init_rs_content(target_entrance),
        //             self.rust_codegen_cache_diff_write_verbose(),
        //         );

        //         // __init__.rs
        //         diff_write(
        //             &src_dir.join("__registration__.rs"),
        //             &comptime.rust_registration_rs_content(target_entrance),
        //             self.rust_codegen_cache_diff_write_verbose(),
        //         );

        //         for module in package.subentities.iter() {
        //             let module_name = module.ident.as_str();
        //             self.compile_maybe_module(
        //                 &comptime,
        //                 src_dir.join(format!("{module_name}.rs")),
        //                 &husky_code_snapshot_dir.join(format!("{module_name}.hsy")),
        //                 module,
        //             )
        //         }
    }

    // fn compile_maybe_module(
    //     &self,
    //     comptime: &HuskyComptime,
    //     rust_code_path: PathBuf,
    //     husky_code_snapshot_path: &Path,
    //     module: &EntityDefn,
    // ) {
    //     match module.variant {
    //         EntityDefnVariant::Module { .. } => (),
    //         _ => return,
    //     }
    //     diff_write(
    //         &rust_code_path,
    //         &comptime.rust_mod_rs_content(module.base_route),
    //         self.rust_codegen_cache_diff_write_verbose(),
    //     );
    //     self.save_husky_code_snapshot(comptime, husky_code_snapshot_path, module.file);
    //     let module_rust_code_gen_dir = rust_code_path.with_extension("");
    //     let module_husky_code_snapshot_dir = husky_code_snapshot_path.with_extension("");
    //     mkdir(&module_rust_code_gen_dir);
    //     mkdir(&module_husky_code_snapshot_dir);
    //     for subentity in module.subentities.iter() {
    //         let subentity_name = subentity.ident.as_str();
    //         self.compile_maybe_module(
    //             comptime,
    //             module_rust_code_gen_dir.join(format!("{subentity_name}.rs")),
    //             &module_husky_code_snapshot_dir.join(format!("{subentity_name}.hsy")),
    //             subentity,
    //         )
    //     }
    // }

    fn save_husky_code_snapshot(
        &self,
        comptime: &AnalysisHost,
        husky_code_snapshot_path: &Path,
        target_entrance: SourcePath,
    ) {
        todo!()
        // diff_write(
        //     husky_code_snapshot_path,
        //     comptime.file_content(target_entrance).to_str().unwrap(),
        //     self.rust_codegen_cache_diff_write_verbose(),
        // );
    }
}
