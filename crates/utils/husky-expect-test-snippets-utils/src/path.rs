use const_format::formatcp;
use husky_control_flow_utils::require;
use husky_path_utils::{find_regular_files, Path, PathBuf};

const TEST_INPUTS: &'static str = "test-inputs";
const TEST_RESULTS: &'static str = "test-results";
const JSON: &'static str = "json";
const MD: &'static str = "md";

pub(crate) fn collect_test_path_stems(relative_folder_path: &str) -> Vec<PathBuf> {
    let dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let tests_dir = dir.join("tests").join(relative_folder_path);
    if !tests_dir.exists() {
        panic!("expect {tests_dir:?} to exist")
    }
    if !tests_dir.is_dir() {
        panic!("expect {tests_dir:?} is directory")
    }
    let mut test_paths: Vec<PathBuf> = vec![];
    let paths = find_regular_files(&tests_dir);
    for path in paths {
        if is_test_input(&path) {
            test_paths.push(path.with_extension("").with_extension(""))
        }
    }
    test_paths
}

fn is_test_input(path: &Path) -> bool {
    let splits = split_file_name(path);
    splits[1] == TEST_INPUTS && splits[2] == JSON
}

fn split_file_name(path: &Path) -> Vec<&str> {
    let file_stem = path.file_name().unwrap().to_str().unwrap();
    let splitting: Vec<_> = file_stem.split(".").collect();
    if !is_splitting_valid(path, &splitting) {
        panic!("{path:?}'s filename is not valid for testing")
    }
    splitting
}

fn is_splitting_valid(path: &Path, splitting: &[&str]) -> bool {
    require!(splitting.len() == 3);
    // todo: check case of splitting[0] is snake case
    if splitting[1] == TEST_INPUTS {
        require!(splitting[2] == JSON)
    } else if splitting[1] == TEST_RESULTS {
        require!(splitting[2] == JSON || splitting[2] == MD)
    } else {
        return false;
    }
    path.parent()
        .unwrap()
        .join(splitting[0])
        .with_extension("test-inputs.json")
        .exists()
}
use super::*;

impl ExpectInstance {
    pub(crate) fn test_results_markdown_path(&self) -> PathBuf {
        self.test_path_stem()
            .with_extension(formatcp!("{TEST_RESULTS}.{MD}"))
    }

    pub(crate) fn test_inputs_json_path(&self) -> PathBuf {
        self.test_path_stem()
            .with_extension(formatcp!("{TEST_INPUTS}.{JSON}"))
    }

    pub(crate) fn test_results_json_path(&self) -> PathBuf {
        self.test_path_stem()
            .with_extension(formatcp!("{TEST_RESULTS}.{JSON}"))
    }
}
