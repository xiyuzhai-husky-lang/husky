use std::{sync::Arc, time::Instant};

use lsp_types::{SemanticTokens, Url};
use parking_lot::{Mutex, RwLock};
use rustc_hash::FxHashMap;

use ide::{Analysis, AnalysisHost, Cancellable, Change, FileID};

use crate::{
    config::ServerConfig,
    from_proto,
    fs::ServerFileSystem,
    line_index::{LineEndings, LineIndex},
    Result,
};

/// An immutable snapshot of the world's state at a point in time.
pub(crate) struct ServerSnapshot {
    pub(crate) config: Arc<ServerConfig>,
    vfs: Arc<RwLock<ServerFileSystem>>,
    pub(crate) analysis: Analysis,
    pub(crate) check_fixes: CheckFixes,
    live_docs: LiveDocs,
    pub(crate) semantic_tokens_cache: Arc<Mutex<FxHashMap<Url, SemanticTokens>>>,
}

impl ServerSnapshot {
    pub(crate) fn url_to_file_id(&self, url: &Url) -> Result<FileID> {
        url_to_file_id(&self.vfs.read().vfs, url)
    }

    pub(crate) fn file_line_index(&self, file_id: FileID) -> Cancellable<LineIndex> {
        let endings = self.vfs.read().line_endings[&file_id];
        let index = self.analysis.file_line_index(file_id)?;
        let res = LineIndex { index, endings };
        Ok(res)
    }
}

pub(crate) fn url_to_file_id(vfs: &vfs::Vfs, url: &Url) -> Result<FileID> {
    let path = from_proto::vfs_path(url)?;
    let res = vfs
        .file_id(&path)
        .ok_or_else(|| format!("file not found: {}", path))?;
    Ok(res)
}
