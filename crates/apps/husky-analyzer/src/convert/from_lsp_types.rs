use crate::*;

use std::{error::Error, fmt::Display, path::PathBuf};

pub(crate) fn path_from_url(url: &lsp_types::Url) -> Result<PathBuf> {
    Ok(url
        .to_file_path()
        .map_err(|()| Box::new(PathConversionError::default()))?)
}

#[derive(Debug, Default)]
pub struct PathConversionError {}
impl Display for PathConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PathConversionError").finish()
    }
}
impl Error for PathConversionError {}

pub trait LspExt {
    type LspType;

    fn from_lsp(t: Self::LspType) -> Self;
    fn into_lsp(self) -> Self::LspType;
}

impl LspExt for PathBuf {
    type LspType = lsp_types::Url;

    fn from_lsp(_t: Self::LspType) -> Self {
        todo!()
    }

    fn into_lsp(self) -> Self::LspType {
        todo!()
    }
}
