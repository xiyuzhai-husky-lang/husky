use super::*;

#[salsa::debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SubmoduleItemPath(ItemPathId);

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SubmoduleItemPathData {
    pub submodule_path: SubmodulePath,
}

impl SubmoduleItemPath {
    pub fn new(submodule_path: SubmodulePath, db: &::salsa::Db) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::SubmoduleItem(SubmoduleItemPathData { submodule_path }),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> SubmoduleItemPathData {
        match self.0.data(db) {
            ItemPathData::SubmoduleItem(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db).submodule_path.ident(db)
    }

    pub fn self_module_path(self, db: &::salsa::Db) -> ModulePath {
        self.data(db).submodule_path.inner()
    }

    pub fn submodule_path(self, db: &::salsa::Db) -> SubmodulePath {
        self.data(db).submodule_path
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.data(db).submodule_path.module_path(db)
    }
}

impl SubmoduleItemPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> SubmoduleItemPath {
        SubmoduleItemPath(id)
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.submodule_path.module_path(db)
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.submodule_path.ident(db)
    }
}

impl salsa::DisplayWithDb for SubmoduleItemPath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.data(db).submodule_path.display_with_db_fmt(f, db)
    }
}
