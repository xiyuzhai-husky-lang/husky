use crate::*;
use std::path::PathBuf;

#[salsa::interned]
pub struct VdFilePath {
    #[return_ref]
    data: PathBuf,
}
