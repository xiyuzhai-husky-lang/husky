use husky_compile_time::*;
use husky_debugger::*;
use mnist_classifier::__init__::link_entity_with_compiled;

#[tokio::main]
async fn main() {
    HuskyDebugger::new(|compile_time| init_compile_time(compile_time))
        .serve("localhost:51617")
        .await
        .expect("")
}

fn init_compile_time(compile_time: &mut HuskyCompileTime) {
    let husky_dir: std::path::PathBuf = std::env::var("HUSKY_DIR").unwrap().into();
    let code_snapshot_dir = husky_dir.join(".compiled/crates/mnist_classifier/snapshot");
    compile_time.load_package(&code_snapshot_dir);
    link_entity_with_compiled(compile_time)
}
