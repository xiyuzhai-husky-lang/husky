use const_format::formatcp;
use husky_control_flow_utils::require;
use husky_path_utils::{Path, PathBuf};

pub(crate) fn collect_test_path_stems(relative_folder_path: &str) -> Vec<PathBuf> {
    let dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let tests_dir = dir.join("tests").join(relative_folder_path);
    assert!(tests_dir.exists());
    assert!(tests_dir.is_dir());
    let mut test_paths: Vec<PathBuf> = vec![];
    for entry in std::fs::read_dir(tests_dir).unwrap() {
        let entry = entry.unwrap();
        let subpath = entry.path();
        if is_test_input(&subpath) {
            test_paths.push(subpath.with_extension("").with_extension(""))
        } else if subpath.is_dir() {
            todo!()
        }
    }
    test_paths
}

fn is_test_input(path: &Path) -> bool {
    let splits = split_file_name(path);
    splits[1] == "test-inputs" && splits[2] == "json"
}

fn split_file_name(path: &Path) -> Vec<&str> {
    let file_stem = path.file_name().unwrap().to_str().unwrap();
    let splitting: Vec<_> = file_stem.split(".").collect();
    if !is_splitting_valid(&splitting) {
        panic!("{path:?}'s filename is not valid for testing")
    }
    splitting
}

const TEST_INPUTS: &'static str = "test-inputs";
const TEST_RESULTS: &'static str = "test-results";
const JSON: &'static str = "json";
const MD: &'static str = "md";

fn is_splitting_valid(splitting: &[&str]) -> bool {
    require!(splitting.len() == 3);
    // todo: check case of splitting[0] is snake case
    if splitting[1] == TEST_INPUTS {
        require!(splitting[2] == JSON)
    } else if splitting[1] == TEST_RESULTS {
        require!(splitting[2] == JSON || splitting[2] == MD)
    } else {
        return false;
    }
    true
}
use super::*;

impl ExpectInstance {
    pub(crate) fn test_results_markdown_path(&self) -> PathBuf {
        self.test_path_stem()
            .with_extension(formatcp!("{TEST_RESULTS}.{MD}"))
    }
    pub(crate) fn test_results_json_path(&self) -> PathBuf {
        self.test_path_stem()
            .with_extension(formatcp!("{TEST_RESULTS}.{JSON}"))
    }
}
