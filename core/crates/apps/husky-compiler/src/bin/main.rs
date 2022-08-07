use clap::Parser;
use husky_compiler::CompilerInstance;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "huskyc")]
#[clap(author = "Kevin K. <kbknapp@gmail.com>")]
pub struct HuskyCompilerCli {
    #[clap(short, long, value_parser)]
    recursive: bool,
    #[clap(value_parser)]
    dir: PathBuf,
}

fn main() {
    let cli = HuskyCompilerCli::parse();
    CompilerInstance::new(
        std::env::var("HUSKY_DIR").expect("env not set"),
        cli.dir,
        cli.recursive,
    )
    .compile_all();
}
