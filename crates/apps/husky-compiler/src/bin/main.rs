use clap::Parser;
use husky_compiler::CompilerInstance;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "husky-compiler")]
#[clap(author = "Xiyu Zhai <dirac12345@gmail.com>")]
pub struct HuskyCompilerCli {
    #[clap(short, long, value_parser)]
    verbose: bool,
    #[clap(value_parser)]
    dir: PathBuf,
}

fn main() {
    let cli = HuskyCompilerCli::parse();
    CompilerInstance::new("".into(), cli.verbose, cli.dir).compile_all();
}
