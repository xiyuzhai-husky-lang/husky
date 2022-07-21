use husky_compiler::compile_all;
use husky_compiler::flags::HuskyCompilerFlags;
use std::path::PathBuf;

fn main() {
    let flags = HuskyCompilerFlags::from_env().expect("invalid arguments");
    compile_all(flags.dir);
}
