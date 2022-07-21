use std::path::Path;

use crate::*;
use husky_rust_code_gen::cargo_toml_content;

impl HuskyCompileTime {
    pub fn cargo_toml_content(&self, main_file: FilePtr, rel_husky_dir: &Path) -> String {
        cargo_toml_content(self, main_file, rel_husky_dir)
    }
}
