//! Conversion lsp_types types to husky-lang-server specific ones.
use common::*;

pub(crate) fn abs_path(url: &lsp_types::Url) -> Result<AbsPathBuf> {
    let path = url.to_file_path().map_err(|()| "url is not a file")?;
    Ok(AbsPathBuf::try_from(path).unwrap())
}

pub(crate) fn vfs_path(url: &lsp_types::Url) -> Result<vfs::VirtualPath> {
    abs_path(url).map(vfs::VirtualPath::from)
}
