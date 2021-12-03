use std::{sync::Arc, time::Instant};

use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::{Mutex, RwLock};
use rustc_hash::FxHashMap;

use ide::{Analysis, AnalysisHost, Cancellable, Change, FileID};

use crate::{
    line_index::{LineCollection, LineEndingType},
    server::handle::Handle,
    source_root_config::SourceRootConfig,
};

#[derive(Default)]
pub(crate) struct ServerFileSystem {
    pub(crate) vfs: vfs::Vfs,
    pub(crate) line_endings: FxHashMap<FileID, LineEndingType>,
    pub(crate) source_root_config: SourceRootConfig,
}

impl ServerFileSystem {
    pub(crate) fn pipe_changes(&mut self) -> Option<Change> {
        let mut fs_changes = Vec::new();
        // A file was added or deleted
        let mut has_structure_changes = false;

        let mut change = Change::new();
        let changed_files = self.vfs.take_changes();
        if changed_files.is_empty() {
            return None;
        }

        for file in changed_files {
            if let Some(path) = self.vfs.file_path(file.file_id).as_path() {
                let path = path.to_path_buf();
                fs_changes.push((path, file.change_kind));
                if file.is_created_or_deleted() {
                    has_structure_changes = true;
                }
            }

            let text = match get_text(&file, &self.vfs) {
                Some(text) => {
                    let (text, line_endings) = LineEndingType::normalize(text);
                    self.line_endings.insert(file.file_id, line_endings);
                    Some(Arc::new(text))
                }
                None => None,
            };

            change.change_file(file.file_id, text);
        }
        if has_structure_changes {
            let roots = self.source_root_config.partition(&self.vfs);
            change.set_roots(roots);
        }
        return Some(change);

        fn get_text(file: &vfs::ChangedFile, vfs: &vfs::Vfs) -> Option<String> {
            if file.exists() {
                let bytes = vfs.file_contents(file.file_id).to_vec();
                String::from_utf8(bytes).ok()
            } else {
                None
            }
        }
    }
}
