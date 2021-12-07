use std::sync::Arc;
mod anchored_path;
pub mod file_id_path_table;
mod path_interner;
mod vfs_path;

use std::{fmt, mem};

use crate::path_interner::PathInterner;

pub use crate::{
    anchored_path::{AnchoredPath, AnchoredPathBuf},
    vfs_path::VfsPath,
};
pub use paths::{AbsPath, AbsPathBuf};

/// Handle to a file in [`Vfs`]
///
/// Most functions in husky-lang-server use this when they need to refer to a file.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FileID(pub u32);

/// Changed file in the [`Vfs`].
#[derive(Copy, Clone, Debug)]
pub struct FileChange {
    /// Id of the changed file
    pub file_id: FileID,
    /// Kind of change
    pub change_kind: FileChangeKind,
}

impl FileChange {
    /// Returns `true` if the change is not [`Delete`](ChangeKind::Delete).
    pub fn exists(&self) -> bool {
        self.change_kind != FileChangeKind::Delete
    }

    /// Returns `true` if the change is [`Create`](ChangeKind::Create) or
    /// [`Delete`](ChangeKind::Delete).
    pub fn is_created_or_deleted(&self) -> bool {
        matches!(
            self.change_kind,
            FileChangeKind::Create | FileChangeKind::Delete
        )
    }
}

/// Kind of [file change](ChangedFile).
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum FileChangeKind {
    /// The file was (re-)created
    Create,
    /// The file was modified
    Modify,
    /// The file was deleted
    Delete,
}

/// Storage for all files read by husky-lang-server.
///
/// For more informations see the [crate-level](crate) documentation.
#[derive(Default)]
pub struct Vfs {
    interner: PathInterner,
    data: Vec<Option<file_data::FileData>>,
    changes: Vec<FileChange>,
}

mod file_data;

impl Vfs {
    /// Id of the given path if it exists in the `Vfs` and is not deleted.
    pub fn get_file_id(&self, path: &VfsPath) -> Option<FileID> {
        self.interner
            .get(path)
            .filter(|&it| self.get_file_content_or_none(it).is_some())
    }

    pub fn get_file_path(&self, file_id: FileID) -> VfsPath {
        self.interner.lookup(file_id).clone()
    }

    pub fn get_file_content(&self, file_id: FileID) -> &String {
        self.get_file_content_or_none(file_id).unwrap()
    }

    pub fn iter_non_deleted(&self) -> impl Iterator<Item = (FileID, &VfsPath)> + '_ {
        (0..self.data.len())
            .map(|it| FileID(it as u32))
            .filter(move |&file_id| self.get_file_content_or_none(file_id).is_some())
            .map(move |file_id| {
                let path = self.interner.lookup(file_id);
                (file_id, path)
            })
    }

    pub fn set_file_content_and_is_updated(
        &mut self,
        path: VfsPath,
        content: Option<Vec<u8>>,
    ) -> bool {
        let file_id = self.alloc_or_get_file_id(path);
        let file_data = content.map(|content| file_data::FileData::new(content));
        let change_kind = match (self.get_file_content_or_none(file_id), &file_data) {
            (None, None) => return false,
            (None, Some(_)) => FileChangeKind::Create,
            (Some(_), None) => FileChangeKind::Delete,
            (Some(old), Some(new)) if old == &new.content => return false,
            (Some(_), Some(_)) => FileChangeKind::Modify,
        };

        *self.get_mut(file_id) = file_data;
        self.changes.push(FileChange {
            file_id,
            change_kind,
        });
        true
    }

    pub fn has_changed(&self) -> bool {
        !self.changes.is_empty()
    }

    pub fn drain_changes(&mut self) -> Vec<FileChange> {
        mem::take(&mut self.changes)
        // eprintln!("start transforming");
        // let mut fs_changes = Vec::new();
        // // A file was added or deleted
        // let mut has_structure_changes = false;
        // eprintln!("here");

        // let mut change = DatabaseChange::new();
        // eprintln!("here");
        // if changed_files.is_empty() {
        //     eprintln!("finish transforming");
        //     return None;
        // }
        // eprintln!("here");

        // for file in changed_files {
        //     eprintln!("here89");
        //     if let Some(path) = self.vfs.read().get_file_path(file.file_id).as_path() {
        //         eprintln!("here90");
        //         let path = path.to_path_buf();
        //         fs_changes.push((path, file.change_kind));
        //         if file.is_created_or_deleted() {
        //             has_structure_changes = true;
        //         }
        //     }
        //     eprintln!("here98");

        //     change.change_file(
        //         file.file_id,
        //         Some(Arc::new(
        //             self.vfs.read().get_file_content(file.file_id).clone(),
        //         )),
        //     );
        // }
        // if has_structure_changes {
        //     todo!()
        //     // let roots = self.source_root_config.partition(&self.vfs);
        //     // change.set_roots(roots);
        // }
        // eprintln!("finish transforming");
        // return Some(change);
    }

    fn alloc_or_get_file_id(&mut self, path: VfsPath) -> FileID {
        let file_id = self.interner.intern(path);
        let idx = file_id.0 as usize;
        let len = self.data.len().max(idx + 1);
        self.data.resize_with(len, || None);
        file_id
    }

    /// Returns the content associated with the given `file_id`.
    ///
    /// # Panics
    ///
    /// Panics if no file is associated to that id.
    fn get_file_content_or_none(&self, file_id: FileID) -> Option<&String> {
        self.data[file_id.0 as usize]
            .as_ref()
            .map(|data| &data.content)
    }

    /// Mutably returns the content associated with the given `file_id`.
    ///
    /// # Panics
    ///
    /// Panics if no file is associated to that id.
    fn get_mut(&mut self, file_id: FileID) -> &mut Option<file_data::FileData> {
        &mut self.data[file_id.0 as usize]
    }
}

impl fmt::Debug for Vfs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vfs")
            .field("n_files", &self.data.len())
            .finish()
    }
}
