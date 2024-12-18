use cargo::{
    core::{compiler::Compilation, Verbosity, Workspace},
    util::command_prelude::CompileMode,
};

pub fn compile_workspace<R>(
    manifest_path: &std::path::Path,
    f: impl FnOnce(Compilation) -> R,
) -> Result<R, ()> {
    assert!(manifest_path.is_absolute());
    let gctx = cargo::GlobalContext::default().expect("what the hell");
    let workspace = Workspace::new(manifest_path, &gctx).expect("what the hell");
    workspace.gctx().shell().set_verbosity(Verbosity::Quiet);
    let mut compile_opts =
        cargo::ops::CompileOptions::new(&gctx, CompileMode::Build).expect("what the hell");
    compile_opts.spec = cargo::ops::Packages::Default;
    match cargo::ops::compile(&workspace, &compile_opts) {
        Ok(compilation) => Ok(f(compilation)),
        Err(_error) => {
            // p!(manifest_path, error);
            // todo!()
            Err(())
        }
    }
}
