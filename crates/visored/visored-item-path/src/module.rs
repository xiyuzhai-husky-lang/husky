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
    pub fn issue_new_paragraph(&mut self, db: &::salsa::Db) -> VdModulePath {
        self.issue_new_child(VdModulePathTag::Paragraph, db)
    }

    pub fn issue_new_division(
        &mut self,
        division_data: VdModulePathDivisionData,
        db: &::salsa::Db,
    ) -> VdModulePath {
        self.issue_new_child(VdModulePathTag::Division(division_data), db)
    }

    pub fn issue_new_environment(&mut self, db: &::salsa::Db) -> VdModulePath {
        self.issue_new_child(VdModulePathTag::Environment, db)
    }

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

        // Test multiple paragraphs (0 through 9)
        let mut paragraph_paths = Vec::new();
        for expected_disambiguator in 0..10 {
            let path = registry.issue_new_paragraph(&db);
            if let VdModulePathData::Paragraph { disambiguator, .. } = path.data(&db) {
                assert_eq!(disambiguator, expected_disambiguator);
                paragraph_paths.push(path);
            } else {
                panic!("Expected Paragraph");
            }
        }

        // Test multiple divisions (0 through 4)
        let section_coword = Coword::from_ref(&db, "test_section");
        let mut division_paths = Vec::new();
        for expected_disambiguator in 0..5 {
            let path =
                registry.issue_new_division(VdModulePathDivisionData::Section(section_coword), &db);
            if let VdModulePathData::Division { disambiguator, .. } = path.data(&db) {
                assert_eq!(disambiguator, expected_disambiguator);
                division_paths.push(path);
            } else {
                panic!("Expected Division");
            }
        }

        // Test multiple environments (0 through 7)
        let mut env_paths = Vec::new();
        for expected_disambiguator in 0..8 {
            let path = registry.issue_new_environment(&db);
            if let VdModulePathData::Environment { disambiguator, .. } = path.data(&db) {
                assert_eq!(disambiguator, expected_disambiguator);
                env_paths.push(path);
            } else {
                panic!("Expected Environment");
            }
        }

        // Verify final counts in the registry
        assert_eq!(*registry.map.get(&VdModulePathTag::Paragraph).unwrap(), 10);
        assert_eq!(
            *registry
                .map
                .get(&VdModulePathTag::Division(
                    VdModulePathDivisionData::Section(section_coword)
                ))
                .unwrap(),
            5
        );
        assert_eq!(*registry.map.get(&VdModulePathTag::Environment).unwrap(), 8);

        // Verify we can still create new items after many iterations
        let last_paragraph = registry.issue_new_paragraph(&db);
        if let VdModulePathData::Paragraph { disambiguator, .. } = last_paragraph.data(&db) {
            assert_eq!(disambiguator, 10);
        } else {
            panic!("Expected Paragraph");
        }
    }
}
