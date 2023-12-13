use cargo::{core::Workspace, util::command_prelude::CompileMode};
use husky_print_utils::p;

pub fn compile_workspace(manifest_path: &std::path::Path) {
    assert!(manifest_path.is_absolute());
    let config = cargo::Config::default().expect("what the hell");
    let workspace = Workspace::new(manifest_path, &config).expect("what the hell");
    let mut compile_opts =
        cargo::ops::CompileOptions::new(&config, CompileMode::Build).expect("what the hell");
    compile_opts.spec = cargo::ops::Packages::Default;
    match cargo::ops::compile(&workspace, &compile_opts) {
        Ok(compilation) => {
            // p!(compilation
            //     .cdylibs
            //     .iter()
            //     .map(|cdylib| &cdylib.path)
            //     .collect::<Vec<_>>());
            // todo!()
        }
        Err(error) => {
            p!(manifest_path, error);
            todo!()
        }
    }
}
