use __husky_root::__main_utils::*;
use husky_compile_time::*;
use husky_debugger::*;
use test_proc_branch_in_loop1::__init__::LINKAGES;

#[tokio::main]
async fn main() {
    let code_snapshot_dir =
        "crates/test-proc-branch-in-loop1/snapshot/test-proc-branch-in-loop1".into();
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
