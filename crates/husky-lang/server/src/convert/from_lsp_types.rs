//! Conversion lsp_types types to husky-lang-server specific ones.
use std::{error::Error, fmt::Display};

use common::*;

pub(crate) fn path(url: &lsp_types::Url) -> Result<PathBuf> {
    Ok(url
        .to_file_path()
        .map_err(|()| Box::new(PathConversionError::default()))?)
}
#[derive(Debug, Default)]
pub struct PathConversionError {}
impl Display for PathConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathConversionError").finish()
    }
}
impl Error for PathConversionError {}
