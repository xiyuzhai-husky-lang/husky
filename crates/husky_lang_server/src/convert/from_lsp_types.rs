//! Conversion lsp_types types to husky-lang-server specific ones.

use vfs::AbsPathBuf;

use crate::{server_snapshot::ServerSnapshot, Result};

pub(crate) fn abs_path(url: &lsp_types::Url) -> Result<AbsPathBuf> {
    let path = url.to_file_path().map_err(|()| "url is not a file")?;
    Ok(AbsPathBuf::try_from(path).unwrap())
}

pub(crate) fn vfs_path(url: &lsp_types::Url) -> Result<vfs::VfsPath> {
    abs_path(url).map(vfs::VfsPath::from)
}

pub(crate) fn to_file_id(snapshot: &ServerSnapshot, url: &lsp_types::Url) -> Result<vfs::FileID> {
    snapshot.url_to_file_id(url)
}
