use crate::runner::VdPipelineRunner;
use alien_seed::AlienSeed;
use eterned::db::EternerDb;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[test]
fn visored_pipeline_works() {
    fn t(
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        parent_dir: &Path,
        config_path: PathBuf,
        src_file_paths: Vec<PathBuf>,
        expect_files_dir: &Path,
    ) {
        let db = &EternerDb::default();
        let mut runner =
            VdPipelineRunner::new(db, tokio_runtime, config_path, src_file_paths).unwrap();
        let seed = AlienSeed::new(0);
        runner.run_all_multi_threaded(seed).unwrap();
        let latex_files = runner.export_result_latex_files(parent_dir).unwrap();
        for latex_file in latex_files {
            use expect_test::expect_file;

            expect_file!(latex_file.relative_path.to_logical_path(expect_files_dir))
                .assert_eq(&latex_file.latex_content);
        }
    }

    let tokio_runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());
    // Collect all .tex files from the directory
    let dir_path = &PathBuf::from("tests-data/visored-pipeline-works");
    let config_path = PathBuf::from("tests-data/visored-pipeline-works/config.yaml");
    let tex_files: Vec<PathBuf> = std::fs::read_dir(&dir_path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "tex" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    assert!(!tex_files.is_empty());

    t(
        tokio_runtime,
        dir_path,
        config_path,
        tex_files,
        Path::new("../expect-files/visored-pipeline-works"),
    );
}
