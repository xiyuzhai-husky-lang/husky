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
    todo!()
}
