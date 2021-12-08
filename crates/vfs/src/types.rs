use common::*;

use paths::AbsPathBuf;

use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct FilePosition {
    pub file_id: FileId,
    pub offset: TextSize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FileRange {
    pub file_id: FileId,
    pub range: TextRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    PackageRoot,
    Main,
    Module,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VirtualPath {
    AbsPathBuf(AbsPathBuf),
    NonResidentPath(String),
}

impl VirtualPath {
    pub fn as_path(&self) -> Result<&std::path::Path> {
        todo!()
    }
}

impl From<AbsPathBuf> for VirtualPath {
    fn from(v: AbsPathBuf) -> Self {
        VirtualPath::AbsPathBuf(v.normalize())
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FileId(u32);
impl FileId {
    pub(crate) fn new(raw: u32) -> FileId {
        FileId(raw)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileContent {
    OnDisk(Arc<String>),
    Live(Arc<String>),
    Deleted,
}
#[derive(Debug, Default)]
pub struct FileInterner {
    next_id: u32,
    map: std::collections::HashMap<VirtualPath, FileId>,
}
impl FileInterner {
    pub(crate) fn get_existing_file_id(&self, path: &VirtualPath) -> Option<FileId> {
        self.map.get(&path).map(|id| *id)
    }
    fn next_file_id(&mut self) -> FileId {
        let id = FileId::new(self.next_id);
        self.next_id += 1;
        id
    }
    pub(crate) fn issue_file_id(&mut self, path: VirtualPath) -> FileId {
        let id = self.next_file_id();
        self.map.insert(path, id);
        id
    }
    pub(crate) fn get_or_issue_file_id(&mut self, path: VirtualPath) -> FileId {
        match self.map.get(&path) {
            Some(id) => *id,
            None => self.issue_file_id(path),
        }
    }
}
