use husky_tracer::*;

#[tokio::main]
async fn main() {
    let flags = flags::HuskyTracerCommand::from_env().expect("invalid arguments");
    if flags.compile {
        todo!()
    } else {
        let mode: Mode = flags.mode.into();
        mode.run(flags.dir.into()).await
    }
}
