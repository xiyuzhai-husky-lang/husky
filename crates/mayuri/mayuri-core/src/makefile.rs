use crate::*;
use husky_make_utils::extract::extract_makefile_rules;
use std::path::Path;

#[derive(Debug, Serialize, PartialEq, Eq, Clone, Hash)]
pub struct MayuriMakefileExtracted {
    content: String,
}

impl MayuriMakefileExtracted {
    pub fn new(
        root: impl AsRef<Path>,
        rules: impl IntoIterator<Item = String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let makefile_path = root.as_ref().join("makefile");
        let content = extract_makefile_rules(makefile_path, rules)?;
        Ok(Self { content })
    }
}
