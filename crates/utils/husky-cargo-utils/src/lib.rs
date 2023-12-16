use cargo::{
    core::{compiler::Compilation, Workspace},
    util::command_prelude::CompileMode,
};
use husky_print_utils::p;

pub fn compile_workspace<R>(
    manifest_path: &std::path::Path,
    f: impl FnOnce(Compilation) -> R,
) -> Result<R, ()> {
    assert!(manifest_path.is_absolute());
    let config = cargo::Config::default().expect("what the hell");
    let workspace = Workspace::new(manifest_path, &config).expect("what the hell");
    let mut compile_opts =
        cargo::ops::CompileOptions::new(&config, CompileMode::Build).expect("what the hell");
    compile_opts.spec = cargo::ops::Packages::Default;
    match cargo::ops::compile(&workspace, &compile_opts) {
        Ok(compilation) => Ok(f(compilation)),
        Err(error) => {
            // p!(manifest_path, error);
            // todo!()
            Err(())
        }
    }
}
