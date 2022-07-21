use __husky_root::__main_utils::*;
use husky_compile_time::*;
use husky_debugger::*;
use test_lazy_control_flow::__init__::LINKAGES;

#[tokio::main]
async fn main() {
    let code_snapshot_dir = "crates/test-lazy-control-flow/snapshot/test-lazy-control-flow".into();
    HuskyDebugger::new(
        HuskyDebuggerConfig {
            package_dir: code_snapshot_dir,
            opt_sample_id: Some(SampleId(23)),
            verbose: false,
            warn_missing_linkage: true,
        },
        LINKAGES,
    )
    .serve("localhost:51617")
    .await
    .expect("")
}
