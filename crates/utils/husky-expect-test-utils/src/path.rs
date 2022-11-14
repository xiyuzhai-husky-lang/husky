use husky_path_utils::{Path, PathBuf};

pub(crate) fn collect_test_paths(relative_folder_path: &str) -> Vec<PathBuf> {
    let dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let tests_dir = dir.join("tests").join(relative_folder_path);
    assert!(tests_dir.exists());
    assert!(tests_dir.is_dir());
    let mut test_paths: Vec<PathBuf> = vec![];
    for entry in std::fs::read_dir(tests_dir).unwrap() {
        let entry = entry.unwrap();
        let subpath = entry.path();
        if subpath.is_dir() {
            todo!()
        } else {
            let file_stem = subpath.file_name().unwrap().to_str().unwrap();
            let splits: Vec<_> = file_stem.split(".").collect();
            assert_eq!(splits[1], "test");
            // ad hoc
            if splits[2] == "md" {
                continue;
            }
            assert_eq!(splits[2], "json");
            assert_eq!(splits.len(), 3);
            test_paths.push(subpath)
        }
    }
    test_paths
}
