use husky_lang_compiler::compile_all;
use std::path::PathBuf;

xflags::xflags! {
    cmd husky-lang-compiler-command
        required dir: PathBuf
    {}
}

fn main() {
    let flags = HuskyLangCompilerCommand::from_env().expect("invalid arguments");
    compile_all(flags.dir);
}
