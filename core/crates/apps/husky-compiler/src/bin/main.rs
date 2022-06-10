use husky_compiler::compile_all;
use std::path::PathBuf;

xflags::xflags! {
    cmd husky-compiler-command
        required dir: PathBuf
    {}
}

fn main() {
    let flags = HuskyCompilerCommand::from_env().expect("invalid arguments");
    compile_all(flags.dir);
}
