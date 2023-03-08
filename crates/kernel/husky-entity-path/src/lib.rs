#![feature(trait_upcasting)]
#![allow(incomplete_features)]
mod ancestry;
mod associated_item;
mod db;
mod error;
mod menu;
mod module_item;
#[cfg(test)]
mod tests;
mod utils;
mod variant;

pub use ancestry::*;
pub use associated_item::*;
pub use db::*;
pub use error::*;
pub use menu::*;
pub use module_item::*;
pub use variant::*;

use husky_entity_taxonomy::*;
use husky_vfs::*;
use husky_word::Identifier;
use salsa::{DbWithJar, DebugWithDb};
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntityPathDb)]
pub struct EntityPathJar(
    TypePath,
    prelude_ty_path,
    TraitPath,
    FormPath,
    TypeItemPath,
    TraitItemPath,
    TypeAsTraitItemPath,
    VariantPath,
    entity_path_menu,
);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
pub enum EntityPath {
    Module(ModulePath),
    ModuleItem(ModuleItemPath),
    AssociatedItem(AssociatedItemPath),
    Variant(VariantPath),
}

impl From<ModulePath> for EntityPath {
    fn from(v: ModulePath) -> Self {
        Self::Module(v)
    }
}

impl From<ModuleItemPath> for EntityPath {
    fn from(v: ModuleItemPath) -> Self {
        Self::ModuleItem(v)
    }
}

impl EntityPath {
    pub fn ident(self, db: &dyn EntityPathDb) -> VfsResult<Identifier> {
        match self {
            EntityPath::Module(path) => path.ident(db),
            EntityPath::ModuleItem(path) => Ok(path.ident(db)),
            EntityPath::AssociatedItem(path) => Ok(path.ident(db)),
            EntityPath::Variant(path) => Ok(path.ident(db)),
        }
    }

    pub fn module_item_path(self) -> Option<ModuleItemPath> {
        match self {
            EntityPath::ModuleItem(module_item_path) => Some(module_item_path),
            _ => None,
        }
    }

    pub fn ty_path(self) -> Option<TypePath> {
        self.module_item_path()?.ty_path()
    }

    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        match self {
            EntityPath::Module(path) => path,
            EntityPath::ModuleItem(path) => path.module_path(db),
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::Variant(_) => todo!(),
        }
    }

    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        self.module_path(db).crate_path(db)
    }

    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }

    pub fn entity_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        match self {
            EntityPath::Module(_path) => EntityKind::Module,
            EntityPath::ModuleItem(path) => path.entity_kind(db),
            EntityPath::AssociatedItem(path) => path.entity_kind(db),
            EntityPath::Variant(_) => todo!(),
        }
    }
}

impl<Db> salsa::DisplayWithDb<Db> for EntityPath
where
    Db: EntityPathDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<EntityPathJar>>::as_jar_db(db);
        match self {
            EntityPath::Module(path) => path.display_with_db_fmt(f, db, level),
            EntityPath::ModuleItem(path) => path.display_with_db_fmt(f, db, level),
            EntityPath::AssociatedItem(path) => path.display_with_db_fmt(f, db, level),
            EntityPath::Variant(path) => path.display_with_db_fmt(f, db, level),
        }
    }
}

impl From<VariantPath> for EntityPath {
    fn from(v: VariantPath) -> Self {
        Self::Variant(v)
    }
}

impl From<AssociatedItemPath> for EntityPath {
    fn from(v: AssociatedItemPath) -> Self {
        Self::AssociatedItem(v)
    }
}

impl From<FormPath> for EntityPath {
    fn from(v: FormPath) -> Self {
        Self::ModuleItem(v.into())
    }
}

impl From<TypePath> for EntityPath {
    fn from(v: TypePath) -> Self {
        Self::ModuleItem(v.into())
    }
}

impl From<TraitPath> for EntityPath {
    fn from(v: TraitPath) -> Self {
        Self::ModuleItem(v.into())
    }
}
