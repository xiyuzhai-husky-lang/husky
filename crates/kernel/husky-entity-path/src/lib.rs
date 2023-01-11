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
use salsa::DbWithJar;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = EntityPathDb)]
pub struct EntityPathJar(
    TypePath,
    TraitPath,
    FormPath,
    TypeItemPath,
    TraitItemPath,
    TypeAsTraitItemPath,
    VariantPath,
    entity_path_menu,
);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
            EntityPath::ModuleItem(_) => todo!(),
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::Variant(_) => todo!(),
        }
    }

    pub fn module_item_path(self) -> Option<ModuleItemPath> {
        match self {
            EntityPath::ModuleItem(module_item_path) => Some(module_item_path),
            _ => None,
        }
    }

    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        match self {
            EntityPath::Module(path) => path,
            EntityPath::ModuleItem(path) => path.module_path(db),
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::Variant(_) => todo!(),
        }
    }

    pub fn entity_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        match self {
            EntityPath::Module(path) => todo!(),
            EntityPath::ModuleItem(path) => path.entity_kind(db),
            EntityPath::AssociatedItem(path) => path.entity_kind(db),
            EntityPath::Variant(_) => todo!(),
        }
    }
}

impl<Db> salsa::DebugWithDb<Db> for EntityPath
where
    Db: EntityPathDb + ?Sized,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<EntityPathJar>>::as_jar_db(db);
        if include_all_fields {
            match self {
                EntityPath::Module(path) => f
                    .debug_tuple("Module")
                    .field(&path.debug_with(db, include_all_fields))
                    .finish(),
                EntityPath::ModuleItem(path) => f
                    .debug_tuple(" ModuleItem")
                    .field(&path.debug_with(db, include_all_fields))
                    .finish(),
                EntityPath::AssociatedItem(path) => f
                    .debug_tuple("AssociatedItem")
                    .field(&path.debug_with(db, include_all_fields))
                    .finish(),
                EntityPath::Variant(path) => f
                    .debug_tuple("EnumVariant")
                    .field(&path.debug_with(db, include_all_fields))
                    .finish(),
            }
        } else {
            match self {
                EntityPath::Module(module_path) => module_path.fmt(f, db, false),
                EntityPath::ModuleItem(module_item_path) => module_item_path.fmt(f, db, false),
                EntityPath::AssociatedItem(associated_item_path) => {
                    associated_item_path.fmt(f, db, false)
                }
                EntityPath::Variant(enum_variant_path) => enum_variant_path.fmt(f, db, false),
            }
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
