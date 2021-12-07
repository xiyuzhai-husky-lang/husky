use std::sync::Arc;

use parking_lot::RwLock;

use ide::{Cancellable, FileID, IdeDatabaseSnapshot};

use crate::{config::ServerConfig, convert::from_lsp_types, line_index::LineCollection, Result};

/// An immutable snapshot of the world's state at a point in time.
pub(crate) struct ServerSnapshot {
    pub(crate) config: Arc<ServerConfig>,
    pub(crate) vfs: Arc<RwLock<vfs::Vfs>>,
    pub(crate) analysis: IdeDatabaseSnapshot,
}

impl std::panic::UnwindSafe for ServerSnapshot {}

impl ServerSnapshot {
    pub(crate) fn url_to_file_id(&self, url: &lsp_types::Url) -> Result<FileID> {
        url_to_file_id(&self.vfs.read(), url)
    }

    pub(crate) fn file_line_collection(&self, file_id: FileID) -> Cancellable<LineCollection> {
        todo!()
        // let endings = self.vfs.read().line_endings[&file_id];
        // let index = self.analysis.file_line_index(file_id)?;
        // let res = LineCollection {
        //     begins: index,
        //     ending_type: endings,
        //     encoding: self.config.offset_encoding(),
        // };
        // Ok(res)
    }
}

pub(crate) fn url_to_file_id(vfs: &vfs::Vfs, url: &lsp_types::Url) -> Result<FileID> {
    let path = from_lsp_types::vfs_path(url)?;
    let res = vfs
        .get_file_id(&path)
        .ok_or_else(|| format!("file not found: {}", path))?;
    Ok(res)
}
