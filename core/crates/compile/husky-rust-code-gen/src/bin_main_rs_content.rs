use crate::*;
use std::path::Path;
use word::snake_to_dash;

pub(crate) fn rust_bin_main_rs_content(
    db: &dyn RustCodeGenQueryGroup,
    main_file: FilePtr,
) -> Arc<String> {
    let package = db.package(main_file).unwrap();
    let package_ident = package.ident.as_str();
    let dashed_package_ident = snake_to_dash(package.ident.as_str());
    Arc::new(format!(
        r#"use husky_debugger::*;
use __husky_root::__main_utils::*;
use {package_ident}::__init__::LINKAGES;
use husky_compile_time::*;

#[tokio::main]
async fn main() {{
    let code_snapshot_dir =
        "crates/{dashed_package_ident}/snapshot/{dashed_package_ident}".into();
    HuskyDebugger::new(
        HuskyDebuggerConfig {{
            package_dir: code_snapshot_dir,
            opt_sample_id: Some(SampleId(23)),
            verbose: false,
            warn_missing_linkage: true,
        }},
        LINKAGES,
    )
    .serve("localhost:51617")
    .await
    .expect("")
}}
"#
    ))
}
