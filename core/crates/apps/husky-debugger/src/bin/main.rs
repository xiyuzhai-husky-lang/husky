use std::path::PathBuf;

use husky_debugger::*;

#[tokio::main]
async fn main() {
    let flags = HuskyDebuggerFlags::from_env().expect("invalid arguments");
    if flags.compile {
        todo!()
    } else {
        let mode: Mode = flags.mode.into();
        let package_dir: PathBuf = flags.package_dir.unwrap().into();
        mode.run(&package_dir).await
    }
}
