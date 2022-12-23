mod init;
mod update;

use std::panic::{RefUnwindSafe, UnwindSafe};

use crate::*;
use convert_case::*;
use husky_unwind_utils::catch_unwind_with_message;
use thiserror::Error;

pub(crate) struct ExpectInstance {
    test_path_stem: PathBuf,
    entries: Vec<ExpectEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub(crate) struct ExpectEntry {
    pub(crate) input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) output: Option<String>,
}

impl ExpectInstance {
    pub(crate) fn test_path_stem(&self) -> &Path {
        &self.test_path_stem
    }

    pub(crate) fn extract_from_file(test_path_stem: PathBuf) -> DesIoResult<Self> {
        let mut this = ExpectInstance {
            test_path_stem,
            entries: vec![],
        };
        this.init()?;
        Ok(this)
    }

    fn entries<'a>(&'a mut self) -> impl Iterator<Item = &mut ExpectEntry> + 'a {
        self.entries.iter_mut()
    }

    pub(crate) fn verify(&self, f: &impl Fn(&str) -> String) {
        for expect in self.entries.iter() {
            let output = f(&expect.input);
            assert_eq!(expect.output, Some(output))
        }
    }
}

#[derive(Debug, Error)]
pub enum UpdateExpectError {
    #[error("output not accepted")]
    OutputNotAccepted,
    #[error("derived panic due to \"{0}\"")]
    DerivedPanic(String),
}

pub type UpdateExpectResult<T> = Result<T, UpdateExpectError>;
