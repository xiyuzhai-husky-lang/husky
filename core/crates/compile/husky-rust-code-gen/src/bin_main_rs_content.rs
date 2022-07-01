use std::path::Path;

use husky_compile_dir::get_code_snapshot_dir;

use crate::*;

pub(crate) fn rust_bin_main_rs_content(
    db: &dyn RustCodeGenQueryGroup,
    main_file: FilePtr,
) -> Arc<String> {
    let package = db.package(main_file).unwrap();
    let package_ident = package.ident.as_str();
    Arc::new(format!(
        r#"use husky_debugger::*;
use {package_ident}::__init__::link_entity_with_compiled;
use husky_compile_time::*;

#[tokio::main]
async fn main() {{
    HuskyDebugger::new(|compile_time| init_compile_time(compile_time))
        .serve("localhost:51617")
        .await
        .expect("")
}}

fn init_compile_time(compile_time: &mut HuskyCompileTime) {{
    let husky_dir: std::path::PathBuf = std::env::var("HUSKY_DIR").unwrap().into();
    let code_snapshot_dir = husky_dir.join(".compiled/crates/{package_ident}/snapshot");
    compile_time.load_package(&code_snapshot_dir);
    link_entity_with_compiled(compile_time)
}}
"#
    ))
}
