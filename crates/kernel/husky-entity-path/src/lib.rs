#![feature(trait_upcasting)]
#![allow(incomplete_features)]
mod ancestry;
mod associated_item;
mod db;
mod enum_variant;
mod error;
mod generic_parameter;
mod menu;
mod module_item;
#[cfg(test)]
mod tests;
mod utils;

pub use ancestry::*;
pub use associated_item::*;
pub use db::*;
pub use enum_variant::*;
pub use error::*;
pub use generic_parameter::*;
pub use menu::*;
pub use module_item::*;

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
    GenericParameterPath,
    AssociatedItemPath,
    ModuleItemVariantPath,
    entity_path_menu,
);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EntityPath {
    Module(ModulePath),
    ModuleItem(ModuleItemPath),
    GenericParameter(GenericParameterPath),
    AssociatedItem(AssociatedItemPath),
    EnumVariant(ModuleItemVariantPath),
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
    pub fn ident(self, db: &dyn EntityPathDb) -> Identifier {
        // Xiao Ma
        match self {
            EntityPath::Module(_) => todo!(),
            EntityPath::ModuleItem(_) => todo!(),
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::EnumVariant(_) => todo!(),
            EntityPath::GenericParameter(_) => todo!(),
        }
    }

    pub fn module_item_path(self) -> Option<ModuleItemPath> {
        match self {
            EntityPath::ModuleItem(module_item_path) => Some(module_item_path),
            _ => None,
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
                EntityPath::Module(module_path) => f
                    .debug_tuple("Module")
                    .field(&module_path.debug_with(db, include_all_fields))
                    .finish(),
                EntityPath::ModuleItem(module_item_path) => f
                    .debug_tuple(" ModuleItem")
                    .field(&module_item_path.debug_with(db, include_all_fields))
                    .finish(),
                EntityPath::GenericParameter(generic_parameter_path) => f
                    .debug_tuple("GenericParameter")
                    .field(&generic_parameter_path.debug_with(db, include_all_fields))
                    .finish(),
                EntityPath::AssociatedItem(associated_item_path) => f
                    .debug_tuple("AssociatedItem")
                    .field(&associated_item_path.debug_with(db, include_all_fields))
                    .finish(),
                EntityPath::EnumVariant(enum_variant_path) => f
                    .debug_tuple("EnumVariant")
                    .field(&enum_variant_path.debug_with(db, include_all_fields))
                    .finish(),
            }
        } else {
            match self {
                EntityPath::Module(module_path) => module_path.fmt(f, db, false),
                EntityPath::ModuleItem(module_item_path) => module_item_path.fmt(f, db, false),
                EntityPath::GenericParameter(generic_parameter_path) => {
                    generic_parameter_path.fmt(f, db, false)
                }
                EntityPath::AssociatedItem(associated_item_path) => {
                    associated_item_path.fmt(f, db, false)
                }
                EntityPath::EnumVariant(enum_variant_path) => enum_variant_path.fmt(f, db, false),
            }
        }
    }
}

impl From<ModuleItemVariantPath> for EntityPath {
    fn from(v: ModuleItemVariantPath) -> Self {
        Self::EnumVariant(v)
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
