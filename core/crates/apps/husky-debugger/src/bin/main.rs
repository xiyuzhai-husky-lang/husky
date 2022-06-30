use husky_debugger::*;

#[tokio::main]
async fn main() {
    let flags = flags::HuskyDebuggerCommand::from_env().expect("invalid arguments");
    if flags.compile {
        todo!()
    } else {
        let mode: Mode = flags.mode.into();
        mode.run(flags.dir.into()).await
    }
}
