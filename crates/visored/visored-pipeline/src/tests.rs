use crate::runner::VdPipelineRunner;
use eterned::db::EternerDb;
use std::path::PathBuf;

#[test]
fn visored_pipeline_works() {
    fn t(config_path: PathBuf, src_file_paths: Vec<PathBuf>) {
        let db = &EternerDb::default();
        let mut runner = VdPipelineRunner::new(db, config_path, src_file_paths).unwrap();
        runner.run_all_single_threaded().unwrap();
    }

    // Collect all .tex files from the directory
    let dir_path = PathBuf::from("tests-data/visored-pipeline-works");
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

    t(config_path, tex_files);
}
