use alien_seed::AlienSeed;
use clap::Parser;
use eterned::db::EternerDb;
use glob::glob;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use visored_pipeline::{error::VdPipelineResult, runner::VdPipelineRunner};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "visored-config.yaml")]
    config: PathBuf,
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
    let db = &EternerDb::default();
    let cli = Cli::parse();
    let tokio_runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let src_file_paths = cli.expanded_src_file_paths();
    let specs_dir: PathBuf = todo!();
    let lean4_dir = todo!();
    let src_root = todo!();
    match run(
        db,
        tokio_runtime,
        &specs_dir,
        lean4_dir,
        cli.config,
        src_file_paths,
        src_root,
    ) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run(
    db: &EternerDb,
    tokio_runtime: Arc<tokio::runtime::Runtime>,
    // TODO: replace with preloaded specs???
    specs_dir: &Path,
    lean4_dir: &Path,
    config_path: PathBuf,
    src_file_paths: Vec<PathBuf>,
    src_root: &Path,
) -> VdPipelineResult<()> {
    let mut runner = VdPipelineRunner::new(
        db,
        tokio_runtime,
        specs_dir,
        lean4_dir,
        config_path,
        src_file_paths,
        src_root,
    )?;
    runner.run_all_single_threaded(AlienSeed::new(0))?;
    Ok(())
}
