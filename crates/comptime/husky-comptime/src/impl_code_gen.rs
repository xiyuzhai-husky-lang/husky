use std::path::Path;

use crate::*;
use husky_rust_code_gen::cargo_toml_content;

impl Comptime {
    pub fn cargo_toml_content(&self, target_entrance: FilePtr, husky_dir: &Path) -> String {
        cargo_toml_content(self, target_entrance, husky_dir)
    }
}
