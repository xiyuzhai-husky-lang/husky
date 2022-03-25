use husky_lang_debugger::*;
use loop_examples::__init__::link_entity_with_compiled;
use compile_time_db::*;

#[tokio::main]
async fn main() {
    Debugger::new(|compile_time| init_compile_time(compile_time))
        .serve("localhost:51617")
        .await
        .expect("")
}

fn init_compile_time(compile_time: &mut HuskyLangCompileTime) {
    compile_time.load_package("/home/xiyuzhai/Documents/husky/rust-gen/loop-examples/snapshot".into());
    link_entity_with_compiled(compile_time)
}
