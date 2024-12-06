use clap::Parser;
use glob::glob;
use std::path::PathBuf;
use visored_pipeline::{error::VdPipelineResult, runner::VdPipelineRunner};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "visored-config.yaml")]
    config_path: PathBuf,
    #[arg(short, long)]
    src_files: Vec<String>,
}

impl Cli {
    fn expanded_src_file_paths(&self) -> Vec<PathBuf> {
        self.src_files
            .iter()
            .flat_map(|pattern| {
                glob(pattern)
                    .expect("Failed to read glob pattern")
                    .filter_map(Result::ok)
            })
            .collect()
    }
}

fn main() {
    let cli = Cli::parse();
    let src_file_paths = cli.expanded_src_file_paths();
    match run(cli.config_path, src_file_paths) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run(config_path: PathBuf, src_file_paths: Vec<PathBuf>) -> VdPipelineResult<()> {
    let mut runner = VdPipelineRunner::new(config_path, src_file_paths)?;
    runner.run_all_single_threaded()?;
    Ok(())
}
