use crate::*;
use husky_coword::Coword;
use rustc_hash::FxHashMap;
use visored_vfs::path::VdFilePath;

#[salsa::interned(constructor = new_inner)]
pub struct VdModulePath {
    data: VdModulePathData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VdModulePathData {
    Root(VdFilePath),
    Division {
        parent: VdModulePath,
        division_data: VdModulePathDivisionData,
        disambiguator: u32,
    },
    Paragraph {
        parent: VdModulePath,
        disambiguator: u32,
    },
    Environment {
        parent: VdModulePath,
        name: String,
        disambiguator: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdModulePathDivisionData {
    Section(Coword),
    Subsection(Coword),
    Subsubsection(Coword),
}

impl VdModulePath {
    pub fn new_root(db: &::salsa::Db, file_path: VdFilePath) -> Self {
        Self::new_inner(db, VdModulePathData::Root(file_path))
    }

    fn new_child_from_tag_and_disambiguator(
        parent: VdModulePath,
        tag: VdModulePathTag,
        disambiguator: u32,
        db: &::salsa::Db,
    ) -> Self {
        let data = match tag {
            VdModulePathTag::Division(division_data) => VdModulePathData::Division {
                parent,
                division_data,
                disambiguator,
            },
            VdModulePathTag::Paragraph => VdModulePathData::Paragraph {
                parent,
                disambiguator,
            },
            VdModulePathTag::Environment => VdModulePathData::Environment {
                parent,
                name: String::new(), // You might want to pass this as a parameter
                disambiguator,
            },
        };
        Self::new_inner(db, data)
    }
}

/// Data without parent and disambiguator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdModulePathTag {
    Division(VdModulePathDivisionData),
    Paragraph,
    Environment,
}

pub struct VdModulePathRegistry {
    parent: VdModulePath,
    pub(crate) map: FxHashMap<VdModulePathTag, u32>,
}

impl VdModulePathRegistry {
    fn issue_new_child(&mut self, tag: VdModulePathTag, db: &::salsa::Db) -> VdModulePath {
        let disambiguator = match self.map.get_mut(&tag) {
            None => {
                self.map.insert(tag, 0);
                0
            }
            Some(existing_disambiguator) => {
                let new_disambiguator = *existing_disambiguator;
                *existing_disambiguator += 1;
                new_disambiguator
            }
        };
        VdModulePath::new_child_from_tag_and_disambiguator(self.parent, tag, disambiguator, db)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_issue_new_child() {
        // Create a mock Salsa database

        let db = DB::default();

        let file_path = VdFilePath::new(&db, PathBuf::from("test.txt"));

        // Create a root path
        let root = VdModulePath::new_inner(&db, VdModulePathData::Root(file_path));

        // Create a registry with the root path
        let mut registry = VdModulePathRegistry {
            parent: root,
            map: FxHashMap::default(),
        };

        // Test paragraph paths
        let paragraph_tag = VdModulePathTag::Paragraph;

        // First paragraph should have disambiguator 0
        let path1 = registry.issue_new_child(paragraph_tag, &db);
        if let VdModulePathData::Paragraph { disambiguator, .. } = path1.data(&db) {
            assert_eq!(disambiguator, 0);
        } else {
            panic!("Expected Paragraph");
        }

        // Second paragraph should have disambiguator 1
        let path2 = registry.issue_new_child(paragraph_tag, &db);
        if let VdModulePathData::Paragraph { disambiguator, .. } = path2.data(&db) {
            assert_eq!(disambiguator, 1);
        } else {
            panic!("Expected Paragraph");
        }
    }
}
