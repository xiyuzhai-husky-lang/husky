use std::path::PathBuf;

use crate::PathBufItd;

pub struct HuskyFilePath {
    path: PathBufItd,
    source_kind: HuskyFileSourceKind,
    content_kind: HuskyFileContentKind,
}

pub enum HuskyFileSourceKind {
    Library,
    Published,
    User,
}

pub enum HuskyFileContentKind {
    SourceProgram,
    Toml,
}
