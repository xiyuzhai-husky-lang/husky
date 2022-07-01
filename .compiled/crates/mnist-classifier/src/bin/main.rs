use husky_debugger::*;
use mnist_classifier::__init__::link_entity_with_compiled;
use husky_compile_time::*;

#[tokio::main]
async fn main() {
    Debugger::new(|compile_time| init_compile_time(compile_time))
        .serve("localhost:51617")
        .await
        .expect("")
}

fn init_compile_time(compile_time: &mut HuskyCompileTime) {
    compile_time.load_package("/home/xiyuzhai/Documents/husky/.compiled/crates/mnist-classifier/snapshot".into());
    link_entity_with_compiled(compile_time)
}
