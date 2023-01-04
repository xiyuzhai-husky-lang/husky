use std::path::PathBuf;
use thiserror::Error;

fn get_home_dir() -> Result<PathBuf, FsSpecsError> {
    let home_dir = dirs::home_dir().ok_or(FsSpecsError::HomeDirError)?;
    Ok(home_dir)
}

pub fn library_path() -> Result<PathBuf, FsSpecsError> {
    Ok(get_home_dir()?.join(".huskyup/toolchains/nightly/lib/rustlib/src/husky/library"))
}

pub fn corgi_install_path() -> Result<PathBuf, FsSpecsError> {
    Ok(get_home_dir()?.join(".corgi"))
}

pub fn huskyup_install_path() -> Result<PathBuf, FsSpecsError> {
    Ok(get_home_dir()?.join(".huskyup"))
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum FsSpecsError {
    #[error("Could not determine home directory")]
    HomeDirError,
    #[error("Failed to canonicalize path: {error_str}")]
    CanonicalizeError {
        error_kind: std::io::ErrorKind,
        error_str: String,
    },
}

pub type FsSpecsResult<T> = Result<T, FsSpecsError>;
