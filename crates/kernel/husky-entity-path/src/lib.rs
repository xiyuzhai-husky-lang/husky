#![feature(trait_upcasting)]
#![allow(incomplete_features)]
mod ancestry;
mod db;
mod error;
mod menu;
mod path;
#[cfg(test)]
mod tests;
mod utils;

pub use self::ancestry::*;
pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::path::*;

use either::*;
use husky_entity_taxonomy::*;
use husky_vfs::*;
use husky_word::Ident;
use salsa::{DbWithJar, DebugWithDb};
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntityPathDb)]
pub struct EntityPathJar(
    TypePath,
    prelude_ty_path,
    TraitPath,
    FugitivePath,
    TypeItemPath,
    TraitItemPath,
    TraitForTypeItemPath,
    TypeVariantPath,
    TypeImplBlockPath,
    TraitForTypeImplBlockPath,
    IllFormedImplBlockPath,
    entity_path_menu,
);

impl EntityPath {
    pub fn ident(self, db: &dyn EntityPathDb) -> Ident {
        match self {
            EntityPath::Module(path) => path.ident(db),
            EntityPath::ModuleItem(path) => path.ident(db),
            EntityPath::AssociatedItem(path) => path.ident(db),
            EntityPath::TypeVariant(path) => path.ident(db),
            EntityPath::ImplBlock(_) => todo!(),
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
            EntityPath::AssociatedItem(path) => path.module_path(db),
            EntityPath::TypeVariant(path) => path.module_path(db),
            EntityPath::ImplBlock(_) => todo!(),
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
            EntityPath::TypeVariant(_) => EntityKind::TypeVariant,
            EntityPath::ImplBlock(_) => todo!(),
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
            EntityPath::TypeVariant(path) => path.display_with_db_fmt(f, db, level),
            EntityPath::ImplBlock(_) => todo!(),
        }
    }
}

impl From<FugitivePath> for EntityPath {
    fn from(v: FugitivePath) -> Self {
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
