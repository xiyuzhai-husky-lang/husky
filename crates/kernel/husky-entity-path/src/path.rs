mod associated_item;
mod impl_block;
mod module_item;
mod variant;

pub use self::associated_item::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::variant::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
#[enum_class::from_variants]
pub enum EntityPath {
    Module(ModulePath),
    ModuleItem(ModuleItemPath),
    AssociatedItem(AssociatedItemPath),
    TypeVariant(TypeVariantPath),
    ImplBlock(ImplBlockPath),
}

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
            EntityPath::ImplBlock(path) => path.module_path(db),
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
            EntityPath::ImplBlock(_) => EntityKind::ImplBlock,
        }
    }

    #[inline(always)]
    pub fn major(self) -> Option<MajorEntityPath> {
        match self {
            EntityPath::Module(path) => Some(path.into()),
            EntityPath::ModuleItem(path) => Some(path.into()),
            EntityPath::AssociatedItem(_)
            | EntityPath::TypeVariant(_)
            | EntityPath::ImplBlock(_) => None,
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
        EntityPath::ModuleItem(v.into())
    }
}

impl From<TypePath> for EntityPath {
    fn from(v: TypePath) -> Self {
        EntityPath::ModuleItem(v.into())
    }
}

impl From<TraitPath> for EntityPath {
    fn from(v: TraitPath) -> Self {
        EntityPath::ModuleItem(v.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
#[enum_class::from_variants]
pub enum MajorEntityPath {
    Module(ModulePath),
    ModuleItem(ModuleItemPath),
}

impl From<FugitivePath> for MajorEntityPath {
    fn from(v: FugitivePath) -> Self {
        MajorEntityPath::ModuleItem(v.into())
    }
}

impl From<TypePath> for MajorEntityPath {
    fn from(v: TypePath) -> Self {
        MajorEntityPath::ModuleItem(v.into())
    }
}

impl From<TraitPath> for MajorEntityPath {
    fn from(v: TraitPath) -> Self {
        MajorEntityPath::ModuleItem(v.into())
    }
}

impl MajorEntityPath {
    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        match self {
            MajorEntityPath::Module(path) => path.crate_path(db),
            MajorEntityPath::ModuleItem(path) => path.crate_path(db),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
#[enum_class::from_variants]
pub enum PrincipalEntityPath {
    Module(ModulePath),
    ModuleItem(ModuleItemPath),
    TypeVariant(TypeVariantPath),
}

impl PrincipalEntityPath {
    #[inline(always)]
    pub fn major(self) -> Option<MajorEntityPath> {
        match self {
            PrincipalEntityPath::Module(path) => Some(path.into()),
            PrincipalEntityPath::ModuleItem(path) => Some(path.into()),
            PrincipalEntityPath::TypeVariant(_) => None,
        }
    }

    pub fn entity_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        match self {
            PrincipalEntityPath::Module(_path) => EntityKind::Module,
            PrincipalEntityPath::ModuleItem(path) => path.entity_kind(db),
            PrincipalEntityPath::TypeVariant(_) => EntityKind::TypeVariant,
        }
    }
}

impl From<MajorEntityPath> for PrincipalEntityPath {
    fn from(path: MajorEntityPath) -> Self {
        match path {
            MajorEntityPath::Module(path) => path.into(),
            MajorEntityPath::ModuleItem(path) => path.into(),
        }
    }
}

impl From<PrincipalEntityPath> for EntityPath {
    fn from(path: PrincipalEntityPath) -> Self {
        match path {
            PrincipalEntityPath::Module(path) => path.into(),
            PrincipalEntityPath::ModuleItem(path) => path.into(),
            PrincipalEntityPath::TypeVariant(path) => path.into(),
        }
    }
}
