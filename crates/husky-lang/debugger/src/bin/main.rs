use husky_lang_compile_time::HuskyLangCompileTime;
use husky_lang_debugger::*;

#[tokio::main]
async fn main() {
    if let Err(e) = Debugger::new(init_compile_time_from_env)
        .serve("localhost:51617")
        .await
    {
        eprintln!("{}", e);
        todo!()
    }
}

fn init_compile_time_from_env(compile_time: &mut HuskyLangCompileTime) {
    let flags = flags::HuskyLangDebuggerCommand::from_env().expect("invalid arguments");
    compile_time.load_package(flags.path)
}
