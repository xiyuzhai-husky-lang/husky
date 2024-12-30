use eterned::db::EternerDb;
use husky_path_utils::search::find_files;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[test]
fn visored_tactic_standard_linear_elaborator_works() {
    use husky_case_utils::{Case, ToCase};
    use husky_path_utils::HuskyLangDevPaths;

    fn t(src_root: &Path, src_file_paths: Vec<PathBuf>, lean4_dir: &Path, expect_files_dir: &Path) {
        let db = &EternerDb::default();
        for src_file_path in src_file_paths {
            use expect_test::expect_file;
            use relative_path::PathExt;

            let relative_path = src_file_path
                .relative_to(src_root)
                .unwrap()
                .to_case(Case::Pascal)
                .with_extension("lean");
            use husky_print_utils::p;
            p!(
                src_file_path.relative_to(src_root).unwrap(),
                relative_path,
                relative_path.to_logical_path(lean4_dir).exists()
            );
            todo!();
            let lean4_code: String = todo!();
            expect_file!(relative_path.to_logical_path(lean4_dir)).assert_eq(&lean4_code);
        }
    }

    let husky_lang_dev_paths = HuskyLangDevPaths::new();
    let specs_dir = husky_lang_dev_paths.specs_dir();
    // Collect all .tex files from the directory
    let src_root = &PathBuf::from("latex/main");
    let tex_files = find_files(src_root, |p| {
        p.extension().map_or(false, |ext| ext == "tex")
    })
    .unwrap();
    let lean4_dir = Path::new("lean4/mathlib4-tests/Mathlib4Tests");
    t(
        src_root,
        tex_files,
        lean4_dir,
        Path::new("../expect-files/visored-pipeline-works"),
    );
}
