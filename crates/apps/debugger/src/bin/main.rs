use std::env;

use husky_lang_debugger::*;

#[tokio::main]
async fn main() {
    for argument in env::args() {
        println!("{argument}");
    }
    let flags = flags::HuskyLangDebuggerCommand::from_env().expect("invalid arguments");
    if flags.compile {
        todo!()
    } else {
        let mode: Mode = flags.mode.into();
        mode.run(flags.dir.into()).await
    }
}
