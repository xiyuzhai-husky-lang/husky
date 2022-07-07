use husky_debugger::*;
use __husky_root::__main_utils::*;
use mnist_classifier::__init__::LINKAGES;
use husky_compile_time::*;

#[tokio::main]
async fn main() {
    let code_snapshot_dir =
        "crates/mnist-classifier/snapshot/mnist-classifier".into();
    HuskyDebugger::new(
        HuskyDebuggerConfig {
            package_dir: code_snapshot_dir,
            opt_sample_id: Some(SampleId(23)),
            verbose: false,
            report_vm: false,
        },
        LINKAGES,
    )
    .serve("localhost:51617")
    .await
    .expect("")
}
