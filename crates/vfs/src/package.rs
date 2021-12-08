use std::collections::HashMap;

use crate::{error::VfsError, *};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct PackageId(u32);

#[derive(Debug, Default)]
pub struct PackageInterner {
    next_id: u32,
    map: HashMap<VirtualPath, PackageId>,
}
impl PackageInterner {
    // self
    pub(crate) fn get_existing_package_id(&self, path: &VirtualPath) -> Option<PackageId> {
        self.map.get(&path).map(|id| *id)
    }
    pub(crate) fn get_existing_package_id_from_source_file_path(
        &self,
        path: &VirtualPath,
    ) -> Option<PackageId> {
        match path {
            VirtualPath::AbsPathBuf(path) => {
                for ancestor in path.ancestors() {
                    match self.get_existing_package_id(&VirtualPath::from(
                        ancestor.with_file_name("main.hsk"),
                    )) {
                        Some(id) => return Some(id),
                        None => (),
                    }
                }
                None
            }
            VirtualPath::NonResidentPath(_) => todo!(),
        }
    }
    // mut self
    pub(crate) fn issue_package_id(&mut self, path: VirtualPath) -> PackageId {
        let id = self.next_package_id();
        self.map.insert(path, id);
        id
    }
    pub(crate) fn get_or_issue_package_id(&mut self, path: VirtualPath) -> PackageId {
        match self.map.get(&path) {
            Some(id) => *id,
            None => self.issue_package_id(path),
        }
    }
    pub(crate) fn get_or_issue_package_id_from_source_file_path(
        &mut self,
        path: VirtualPath,
    ) -> Result<PackageId> {
        match self.get_existing_package_id_from_source_file_path(&path) {
            Some(id) => Ok(id),
            None => match path {
                VirtualPath::AbsPathBuf(path) => {
                    for ancestor in path.ancestors() {
                        let main_path = ancestor.with_file_name("main.hsk");
                        if main_path.is_file() {
                            return Ok(self.issue_package_id(VirtualPath::AbsPathBuf(main_path)));
                        }
                    }
                    Err(Box::new(VfsError::NoSuchPackage))
                }
                VirtualPath::NonResidentPath(_) => todo!(),
            },
        }
    }

    fn next_package_id(&mut self) -> PackageId {
        let id = PackageId(self.next_id);
        self.next_id += 1;
        id
    }
}
