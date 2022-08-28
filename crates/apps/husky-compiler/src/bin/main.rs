use clap::Parser;
use husky_compiler::CompilerInstance;
use relative_path::RelativePathBuf;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "husky-compiler")]
#[clap(author = "Xiyu Zhai <dirac12345@gmail.com>")]
pub struct HuskyCompilerCli {
    #[clap(value_parser)]
    dir: PathBuf,
}

fn main() {
    let cli = HuskyCompilerCli::parse();
    CompilerInstance::new(RelativePathBuf::from_path(cli.dir).unwrap()).compile_all();
}
