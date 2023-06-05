use husky_token::TokenSheetData;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum RegionPath {
    Snippet(Toolchain),
    Decr(DecrId),
    Decl(DeclRegionPath),
    Defn(DefnRegionPath),
}

impl RegionPath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> Option<ModulePath> {
        match self {
            RegionPath::Snippet(_) => None,
            RegionPath::Decr(id) => Some(id.module_path(db)),
            RegionPath::Decl(path) => Some(path.module_path(db)),
            RegionPath::Defn(path) => Some(path.module_path(db)),
        }
    }

    pub fn toolchain(self, db: &dyn EntityTreeDb) -> Toolchain {
        match self {
            RegionPath::Snippet(_) => todo!(),
            _ => self.module_path(db).unwrap().toolchain(db),
        }
    }

    pub fn token_sheet_data<'a>(self, db: &'a dyn EntityTreeDb) -> VfsResult<&'a TokenSheetData> {
        match self.module_path(db) {
            Some(module_path) => db.token_sheet_data(module_path),
            None => todo!(),
        }
    }
}

impl From<DecrId> for RegionPath {
    fn from(v: DecrId) -> Self {
        Self::Decr(v)
    }
}

impl From<DefnRegionPath> for RegionPath {
    fn from(v: DefnRegionPath) -> Self {
        Self::Defn(v)
    }
}

impl From<DeclRegionPath> for RegionPath {
    fn from(v: DeclRegionPath) -> Self {
        Self::Decl(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum DeclRegionPath {
    Entity(EntityPath),
    ImplBlock(ImplBlockId),
    AssociatedItem(AssociatedItemId),
}

impl DeclRegionPath {
    pub fn defn_region_path(self) -> DefnRegionPath {
        match self {
            DeclRegionPath::Entity(path) => DefnRegionPath::Entity(path),
            DeclRegionPath::ImplBlock(id) => DefnRegionPath::Impl(id),
            DeclRegionPath::AssociatedItem(id) => DefnRegionPath::AssociatedItem(id),
        }
    }
}

impl DeclRegionPath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        match self {
            DeclRegionPath::Entity(path) => path.module_path(db),
            DeclRegionPath::ImplBlock(id) => id.module(),
            DeclRegionPath::AssociatedItem(id) => id.module_path(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum DefnRegionPath {
    Entity(EntityPath),
    Impl(ImplBlockId),
    AssociatedItem(AssociatedItemId),
}

impl DefnRegionPath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        match self {
            DefnRegionPath::Entity(path) => path.module_path(db),
            DefnRegionPath::AssociatedItem(id) => id.module_path(),
            DefnRegionPath::Impl(id) => id.module(),
        }
    }
}
