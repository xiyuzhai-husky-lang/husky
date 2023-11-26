mod associated_item;
mod attr;
mod impl_block;
mod major_item;
mod ty_variant;

use enum_class::Room32;
use salsa::Database;

pub use self::associated_item::*;
pub use self::attr::*;
pub use self::impl_block::*;
pub use self::major_item::*;
pub use self::ty_variant::*;

use crate::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct ItemPathId {
    pub data: ItemPathData,
}

impl ItemPathId {
    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        self.data(db).module_path(db)
    }

    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        todo!()
    }

    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        todo!()
    }

    pub fn ident(self, db: &dyn EntityPathDb) -> Option<Ident> {
        todo!()
        // match self {
        //     ItemPath::Submodule(path) => Some(path.ident(db)),
        //     ItemPath::MajorItem(path) => Some(path.ident(db)),
        //     ItemPath::AssociatedItem(path) => Some(path.ident(db)),
        //     ItemPath::TypeVariant(path) => Some(path.ident(db)),
        //     ItemPath::ImplBlock(_) => None,
        //     ItemPath::Attr(_) => None,
        // }
    }

    pub fn item_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        todo!()
        // EntityKind::AssociatedItem {
        //     associated_item_kind: match self {
        //         AssociatedItemPath::TypeItem(path) => {
        //             AssociatedItemKind::TypeItem(path.item_kind(db))
        //         }

        //         AssociatedItemPath::TraitItem(path) => {
        //             AssociatedItemKind::TraitItem(path.item_kind(db))
        //         }
        //         AssociatedItemPath::TraitForTypeItem(path) => {
        //             AssociatedItemKind::TraitForTypeItem(path.item_kind(db))
        //         }
        //     },
        // }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum ItemPathData {
    Submodule(SubmodulePath),
    MajorItem(MajorItemPathData),
    AssociatedItem(AssociatedItemPathData),
    TypeVariant(TypeVariantPathData),
    ImplBlock(ImplBlockPathData),
    Attr(AttrItemPathData),
}

impl ItemPathData {
    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        match self {
            ItemPathData::Submodule(data) => data.parent(db),
            ItemPathData::MajorItem(data) => data.module_path(db),
            ItemPathData::AssociatedItem(data) => data.module_path(db),
            ItemPathData::TypeVariant(data) => data.module_path(db),
            ItemPathData::ImplBlock(data) => data.module_path(db),
            ItemPathData::Attr(data) => data.module_path(db),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EntityPathDb, jar = EntityPathJar)]
#[enum_class::from_variants]
pub enum EntityPath {
    Module(ModulePath),
    MajorItem(MajorItemPath),
    AssociatedItem(AssociatedItemPath),
    TypeVariant(Room32, TypeVariantPath),
    ImplBlock(ImplBlockPath),
    Attr(Room32, AttrItemPath),
}

impl EntityPath {
    pub fn ident(self, db: &dyn EntityPathDb) -> Option<Ident> {
        match self {
            EntityPath::Module(path) => Some(path.ident(db)),
            EntityPath::MajorItem(path) => Some(path.ident(db)),
            EntityPath::AssociatedItem(path) => path.ident(db),
            EntityPath::TypeVariant(_, path) => Some(path.ident(db)),
            EntityPath::ImplBlock(_) => None,
            EntityPath::Attr(_, _) => None,
        }
    }

    pub fn module_item_path(self) -> Option<MajorItemPath> {
        match self {
            EntityPath::MajorItem(module_item_path) => Some(module_item_path),
            _ => None,
        }
    }

    pub fn ty_path(self) -> Option<TypePath> {
        self.module_item_path()?.ty_path()
    }

    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        match self {
            EntityPath::Module(path) => path.crate_path(db),
            EntityPath::MajorItem(path) => path.crate_path(db),
            EntityPath::AssociatedItem(path) => path.crate_path(db),
            EntityPath::TypeVariant(_, path) => path.crate_path(db),
            EntityPath::ImplBlock(path) => path.crate_path(db),
            EntityPath::Attr(_, path) => path.crate_path(db),
        }
    }

    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }

    pub fn item_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        match self {
            EntityPath::Module(_path) => EntityKind::Module,
            EntityPath::MajorItem(path) => path.item_kind(db),
            EntityPath::AssociatedItem(path) => path.item_kind(db),
            EntityPath::TypeVariant(_, _) => EntityKind::TypeVariant,
            EntityPath::ImplBlock(_) => EntityKind::ImplBlock,
            EntityPath::Attr(_, _) => todo!(),
        }
    }

    #[inline(always)]
    pub fn major(self) -> Option<MajorEntityPath> {
        match self {
            EntityPath::Module(path) => Some(path.into()),
            EntityPath::MajorItem(path) => Some(path.into()),
            EntityPath::AssociatedItem(_)
            | EntityPath::TypeVariant(_, _)
            | EntityPath::ImplBlock(_) => None,
            EntityPath::Attr(_, _) => todo!(),
        }
    }
}

impl From<FugitivePath> for EntityPath {
    fn from(v: FugitivePath) -> Self {
        EntityPath::MajorItem(v.into())
    }
}

impl From<TypePath> for EntityPath {
    fn from(v: TypePath) -> Self {
        EntityPath::MajorItem(v.into())
    }
}

impl From<TraitPath> for EntityPath {
    fn from(v: TraitPath) -> Self {
        EntityPath::MajorItem(v.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EntityPathDb, jar = EntityPathJar)]
#[enum_class::from_variants]
pub enum IdentifiableEntityPath {
    Module(ModulePath),
    MajorItem(MajorItemPath),
    AssociatedItem(AssociatedItemPath),
    TypeVariant(TypeVariantPath),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EntityPathDb, jar = EntityPathJar)]
#[enum_class::from_variants]
pub enum ItemPath {
    Submodule(Room32, SubmodulePath),
    MajorItem(MajorItemPath),
    AssociatedItem(AssociatedItemPath),
    TypeVariant(Room32, TypeVariantPath),
    ImplBlock(ImplBlockPath),
    Attr(Room32, AttrItemPath),
}

#[test]
fn item_path_size_works() {
    assert_eq!(
        std::mem::size_of::<ItemPath>(),
        std::mem::size_of::<[u32; 3]>()
    )
}

impl std::ops::Deref for ItemPath {
    type Target = ItemPathId;

    fn deref(&self) -> &Self::Target {
        unsafe { &std::mem::transmute::<_, &((u32, u32), ItemPathId)>(self).1 }
    }
}

impl ItemPath {
    pub fn module_item_path(self) -> Option<MajorItemPath> {
        match self {
            ItemPath::MajorItem(module_item_path) => Some(module_item_path),
            _ => None,
        }
    }

    pub fn ty_path(self) -> Option<TypePath> {
        self.module_item_path()?.ty_path()
    }

    #[inline(always)]
    pub fn major(self) -> Option<MajorEntityPath> {
        match self {
            ItemPath::Submodule(_, path) => Some(path.inner().into()),
            ItemPath::MajorItem(path) => Some(path.into()),
            ItemPath::AssociatedItem(_)
            | ItemPath::TypeVariant(_, _)
            | ItemPath::ImplBlock(_)
            | ItemPath::Attr(_, _) => None,
        }
    }
}

impl salsa::DisplayWithDb for ItemPath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn Database,
    ) -> std::fmt::Result {
        match self {
            ItemPath::Submodule(_, path) => path.display_with_db_fmt(f, db),
            ItemPath::MajorItem(path) => path.display_with_db_fmt(f, db),
            ItemPath::AssociatedItem(path) => path.display_with_db_fmt(f, db),
            ItemPath::TypeVariant(_, path) => path.display_with_db_fmt(f, db),
            ItemPath::ImplBlock(path) => todo!(),
            ItemPath::Attr(_, _) => todo!(),
        }
    }
}

impl From<FugitivePath> for ItemPath {
    fn from(v: FugitivePath) -> Self {
        ItemPath::MajorItem(v.into())
    }
}

impl From<TypePath> for ItemPath {
    fn from(v: TypePath) -> Self {
        ItemPath::MajorItem(v.into())
    }
}

impl From<TraitPath> for ItemPath {
    fn from(v: TraitPath) -> Self {
        ItemPath::MajorItem(v.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EntityPathDb, jar = EntityPathJar)]
#[enum_class::from_variants]
pub enum PatternPath {
    Type(TypePath),
    TypeVariant(TypeVariantPath),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EntityPathDb, jar = EntityPathJar)]
#[enum_class::from_variants]
pub enum MajorEntityPath {
    Module(ModulePath),
    MajorItem(MajorItemPath),
}

impl From<FugitivePath> for MajorEntityPath {
    fn from(v: FugitivePath) -> Self {
        MajorEntityPath::MajorItem(v.into())
    }
}

impl From<TypePath> for MajorEntityPath {
    fn from(v: TypePath) -> Self {
        MajorEntityPath::MajorItem(v.into())
    }
}

impl From<TraitPath> for MajorEntityPath {
    fn from(v: TraitPath) -> Self {
        MajorEntityPath::MajorItem(v.into())
    }
}

impl MajorEntityPath {
    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        match self {
            MajorEntityPath::Module(path) => path.crate_path(db),
            MajorEntityPath::MajorItem(path) => path.crate_path(db),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EntityPathDb, jar = EntityPathJar)]
#[enum_class::from_variants]
pub enum PrincipalEntityPath {
    Module(ModulePath),
    MajorItem(MajorItemPath),
    TypeVariant(TypeVariantPath),
}

impl PrincipalEntityPath {
    #[inline(always)]
    pub fn major(self) -> Option<MajorEntityPath> {
        match self {
            PrincipalEntityPath::Module(path) => Some(path.into()),
            PrincipalEntityPath::MajorItem(path) => Some(path.into()),
            PrincipalEntityPath::TypeVariant(_) => None,
        }
    }

    pub fn item_kind(self, db: &dyn EntityPathDb) -> EntityKind {
        match self {
            PrincipalEntityPath::Module(_path) => EntityKind::Module,
            PrincipalEntityPath::MajorItem(path) => path.item_kind(db),
            PrincipalEntityPath::TypeVariant(_) => EntityKind::TypeVariant,
        }
    }
}

impl From<MajorEntityPath> for PrincipalEntityPath {
    fn from(path: MajorEntityPath) -> Self {
        match path {
            MajorEntityPath::Module(_path) => todo!(),
            // path.into(),
            MajorEntityPath::MajorItem(path) => path.into(),
        }
    }
}

impl From<PrincipalEntityPath> for EntityPath {
    fn from(path: PrincipalEntityPath) -> Self {
        todo!()
        // match path {
        //     PrincipalEntityPath::Module(path) => path.into(),
        //     PrincipalEntityPath::MajorItem(path) => path.into(),
        //     PrincipalEntityPath::TypeVariant(path) => path.into(),
        // }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EntityPathDb, jar = EntityPathJar)]
#[enum_class::from_variants]
pub enum PrincipalItemPath {
    Submodule(SubmodulePath),
    MajorItem(MajorItemPath),
    TypeVariant(TypeVariantPath),
}
