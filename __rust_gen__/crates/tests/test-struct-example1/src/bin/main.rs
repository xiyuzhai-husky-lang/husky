use __husky_root::__main_utils::*;
use husky_compile_time::*;
use husky_debugger::*;
use test_struct_example1::__init__::LINKAGES;

#[tokio::main]
async fn main() {
    let code_snapshot_dir =
        "crates/tests/test-struct-example1/snapshot/test-struct-example1".into();
    HuskyDebugger::new(
        HuskyDebuggerConfig {
            package_dir: code_snapshot_dir,
            opt_sample_id: Some(__SampleId(23)),
            verbose: false,
            warn_missing_linkage: true,
        },
        LINKAGES,
    )
    .serve("localhost:51617")
    .await
    .expect("")
}

#[test]
fn serve_on_error() {
    let code_snapshot_dir = "snapshot/test-struct-example1".into();
    let sample_id = __SampleId(23);
    match tokio_test::block_on(
        HuskyDebugger::new(
            HuskyDebuggerConfig {
                package_dir: code_snapshot_dir,
                opt_sample_id: Some(sample_id),
                verbose: false,
                warn_missing_linkage: true,
            },
            LINKAGES,
        )
        .serve_on_error("localhost:51617", sample_id),
    ) {
        __TestResult::Success => (),
        __TestResult::Failure => panic!(),
    }
}
