use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum RegionPath {
    Snippet(Toolchain),
    Decl(DeclRegionPath),
    Defn(DefnRegionPath),
}

impl RegionPath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        match self {
            RegionPath::Snippet(_) => todo!(),
            RegionPath::Decl(path) => path.module_path(db),
            RegionPath::Defn(path) => path.module_path(db),
        }
    }

    pub fn toolchain(self, db: &dyn EntityTreeDb) -> Toolchain {
        self.module_path(db).toolchain(db)
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
    Impl(ImplBlockId),
    AssociatedItem(AssociatedItemId),
}

impl DeclRegionPath {
    pub fn defn_region_path(self) -> DefnRegionPath {
        match self {
            DeclRegionPath::Entity(path) => DefnRegionPath::Entity(path),
            DeclRegionPath::Impl(id) => DefnRegionPath::Impl(id),
            DeclRegionPath::AssociatedItem(id) => DefnRegionPath::AssociatedItem(id),
        }
    }
}

impl DeclRegionPath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        match self {
            DeclRegionPath::Entity(path) => path.module_path(db),
            DeclRegionPath::Impl(id) => id.module_path(),
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
            DefnRegionPath::Impl(id) => id.module_path(),
        }
    }
}
