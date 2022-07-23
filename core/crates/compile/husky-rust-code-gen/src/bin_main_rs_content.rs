use crate::*;
use std::path::Path;
use word::snake_to_dash;

pub(crate) fn rust_bin_main_rs_content(
    db: &dyn RustCodeGenQueryGroup,
    main_file: FilePtr,
    rel_crate_dir: PathBuf,
) -> Arc<String> {
    let package = db.package(main_file).unwrap();
    let package_ident = package.ident.as_str();
    let dashed_package_ident = snake_to_dash(package.ident.as_str());
    let rel_crate_dir = rel_crate_dir.display();
    Arc::new(format!(
        r#"use __husky::__main_utils::*;
use {package_ident}::__init__::LINKAGES;

#[tokio::main]
async fn main() {{
    let code_snapshot_dir =
        "{rel_crate_dir}/{dashed_package_ident}/snapshot/{dashed_package_ident}".into();
    HuskyDebugger::new(
        HuskyDebuggerConfig {{
            package_dir: code_snapshot_dir,
            opt_sample_id: Some(__SampleId(23)),
            verbose: false,
            warn_missing_linkage: true,
        }},
        LINKAGES,
    )
    .serve("localhost:51617")
    .await
    .expect("")
}}

#[test]
fn serve_on_error() {{
    __serve_on_error_init();
    let code_snapshot_dir =
        "snapshot/{dashed_package_ident}".into();
    let sample_id = __SampleId(23);
    match tokio_test::block_on(
        HuskyDebugger::new(
            HuskyDebuggerConfig {{
                package_dir: code_snapshot_dir,
                opt_sample_id: Some(sample_id),
                verbose: false,
                warn_missing_linkage: true,
            }},
            LINKAGES,
        )
        .serve_on_error("localhost:51617", sample_id),
    ) {{
        __TestResult::Success => (),
        __TestResult::Failure => panic!(),
    }}
}}
"#
    ))
}
