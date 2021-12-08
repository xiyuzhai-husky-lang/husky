use std::collections::HashMap;

use crate::*;

#[derive(Clone, Copy, Debug)]
pub struct SourceFilePosition {
    pub file_id: SourceFileId,
    pub offset: TextSize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SourceFileRange {
    pub file_id: SourceFileId,
    pub range: TextRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceFileType {
    Main,
    Lib,
    Module,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SourceFileId(u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceFileContent {
    OnDisk(Arc<String>),
    Live(Arc<String>),
    Deleted,
}

#[derive(Debug, Default)]
pub struct SourceFileInterner {
    next_id: u32,
    map: HashMap<VirtualPath, SourceFileId>,
}
impl SourceFileInterner {
    pub(crate) fn get_existing_file_id(&self, path: &VirtualPath) -> Option<SourceFileId> {
        self.map.get(&path).map(|id| *id)
    }
    fn next_file_id(&mut self) -> SourceFileId {
        let id = SourceFileId(self.next_id);
        self.next_id += 1;
        id
    }
    pub(crate) fn issue_file_id(&mut self, path: VirtualPath) -> SourceFileId {
        let id = self.next_file_id();
        self.map.insert(path, id);
        id
    }
    pub(crate) fn get_or_issue_file_id(&mut self, path: VirtualPath) -> SourceFileId {
        match self.map.get(&path) {
            Some(id) => *id,
            None => self.issue_file_id(path),
        }
    }
}
