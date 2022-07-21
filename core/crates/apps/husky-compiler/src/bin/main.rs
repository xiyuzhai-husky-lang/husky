use husky_compiler::flags::HuskyCompilerFlags;
use husky_compiler::CompilerInstance;
use std::path::PathBuf;

fn main() {
    CompilerInstance::from_env().compile_all();
}
