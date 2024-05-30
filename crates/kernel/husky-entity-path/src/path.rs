pub mod assoc_item;
pub mod attr;
pub mod impl_block;
pub mod major_item;
pub mod script;
pub mod submodule;
pub mod ty_variant;
mod utils;

use self::assoc_item::*;
use self::impl_block::*;
use self::major_item::{form::MajorFormPath, ty::TypePath, *};
use self::script::{ScriptItemPath, ScriptItemPathData};
use self::submodule::*;
use self::ty_variant::*;
use self::utils::item_debug_fmt_with_db;
use self::{attr::*, trai::TraitPath};
use crate::*;
use enum_class::Room32;
use husky_vfs::script::Script;

#[salsa::interned(override_debug)]
pub struct ItemPathId {
    pub data: ItemPathData,
}

impl ItemPathId {
    pub fn item_path(self, db: &::salsa::Db) -> ItemPath {
        self.data(db).item_path(self)
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.data(db).module_path(db)
    }

    pub fn crate_path(self, db: &::salsa::Db) -> CratePath {
        self.module_path(db).crate_path(db)
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }

    pub fn ident(self, db: &::salsa::Db) -> Option<Ident> {
        self.data(db).ident(db)
    }

    pub fn item_kind(self, db: &::salsa::Db) -> EntityKind {
        self.data(db).entity_kind(db)
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum ItemPathData {
    SubmoduleItem(SubmoduleItemPathData),
    MajorItem(MajorItemPathData),
    AssocItem(AssocItemPathData),
    TypeVariant(TypeVariantPathData),
    ImplBlock(ImplBlockPathData),
    Attr(AttrItemPathData),
    Script(ScriptItemPathData),
}

impl ItemPathData {
    #[inline(always)]
    fn item_path(self, id: ItemPathId) -> ItemPath {
        match self {
            ItemPathData::SubmoduleItem(slf) => slf.item_path(id).into(),
            ItemPathData::MajorItem(slf) => slf.item_path(id).into(),
            ItemPathData::AssocItem(slf) => slf.item_path(id).into(),
            ItemPathData::TypeVariant(slf) => slf.item_path(id).into(),
            ItemPathData::ImplBlock(slf) => slf.item_path(id).into(),
            ItemPathData::Attr(slf) => slf.item_path(id).into(),
            ItemPathData::Script(_) => todo!(),
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            ItemPathData::SubmoduleItem(slf) => slf.module_path(db),
            ItemPathData::MajorItem(slf) => slf.module_path(db),
            ItemPathData::AssocItem(slf) => slf.module_path(db),
            ItemPathData::TypeVariant(slf) => slf.module_path(db),
            ItemPathData::ImplBlock(slf) => slf.module_path(db),
            ItemPathData::Attr(slf) => slf.module_path(db),
            ItemPathData::Script(slf) => *slf.module_path(db),
        }
    }

    pub fn ident(self, db: &::salsa::Db) -> Option<Ident> {
        match self {
            ItemPathData::SubmoduleItem(slf) => Some(slf.ident(db)),
            ItemPathData::MajorItem(slf) => Some(slf.ident()),
            ItemPathData::AssocItem(slf) => Some(slf.ident(db)),
            ItemPathData::TypeVariant(slf) => Some(slf.ident),
            ItemPathData::ImplBlock(_) => None,
            ItemPathData::Attr(slf) => Some(slf.ident),
            ItemPathData::Script(_) => todo!(),
        }
    }

    pub fn entity_kind(self, db: &::salsa::Db) -> EntityKind {
        match self {
            ItemPathData::SubmoduleItem(_) => EntityKind::Module,
            ItemPathData::MajorItem(slf) => slf.entity_kind(db),
            ItemPathData::AssocItem(slf) => slf.entity_kind(db),
            ItemPathData::TypeVariant(_slf) => EntityKind::TypeVariant,
            ItemPathData::ImplBlock(_slf) => EntityKind::ImplBlock,
            ItemPathData::Attr(_slf) => EntityKind::Attr,
            ItemPathData::Script(_slf) => EntityKind::Script,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum EntityPath {
    Module(ModulePath),
    MajorItem(MajorItemPath),
    AssocItem(AssocItemPath),
    TypeVariant(Room32, TypeVariantPath),
    ImplBlock(ImplBlockPath),
    Attr(Room32, AttrItemPath),
}

impl EntityPath {
    pub fn ident(self, db: &::salsa::Db) -> Option<Ident> {
        match self {
            EntityPath::Module(path) => Some(path.ident(db)),
            EntityPath::MajorItem(path) => Some(path.ident(db)),
            EntityPath::AssocItem(path) => path.ident(db),
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

    pub fn crate_path(self, db: &::salsa::Db) -> CratePath {
        match self {
            EntityPath::Module(path) => path.crate_path(db),
            EntityPath::MajorItem(path) => path.crate_path(db),
            EntityPath::AssocItem(path) => path.crate_path(db),
            EntityPath::TypeVariant(_, path) => path.crate_path(db),
            EntityPath::ImplBlock(path) => path.crate_path(db),
            EntityPath::Attr(_, path) => path.crate_path(db),
        }
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }

    pub fn item_kind(self, db: &::salsa::Db) -> EntityKind {
        match self {
            EntityPath::Module(_path) => EntityKind::Module,
            EntityPath::MajorItem(path) => path.item_kind(db),
            EntityPath::AssocItem(path) => path.item_kind(db),
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
            EntityPath::AssocItem(_) | EntityPath::TypeVariant(_, _) | EntityPath::ImplBlock(_) => {
                None
            }
            EntityPath::Attr(_, _) => todo!(),
        }
    }
}

impl From<MajorFormPath> for EntityPath {
    fn from(v: MajorFormPath) -> Self {
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
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum IdentifiableEntityPath {
    Module(ModulePath),
    MajorItem(MajorItemPath),
    AssocItem(AssocItemPath),
    TypeVariant(TypeVariantPath),
}

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ItemPath {
    Submodule(Room32, SubmoduleItemPath),
    MajorItem(MajorItemPath),
    AssocItem(AssocItemPath),
    TypeVariant(Room32, TypeVariantPath),
    ImplBlock(ImplBlockPath),
    Attr(Room32, AttrItemPath),
    Script(Room32, ScriptItemPath),
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

    pub fn major(self, db: &::salsa::Db) -> Option<MajorEntityPath> {
        match self {
            ItemPath::Submodule(_, path) => Some(path.self_module_path(db).into()),
            ItemPath::MajorItem(path) => Some(path.into()),
            ItemPath::AssocItem(_)
            | ItemPath::TypeVariant(_, _)
            | ItemPath::ImplBlock(_)
            | ItemPath::Attr(_, _)
            | ItemPath::Script(_, _) => None,
        }
    }
}

impl salsa::DisplayWithDb for ItemPath {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        match self {
            ItemPath::Submodule(_, path) => path.display_fmt_with_db(f, db),
            ItemPath::MajorItem(path) => path.display_fmt_with_db(f, db),
            ItemPath::AssocItem(path) => path.display_fmt_with_db(f, db),
            ItemPath::TypeVariant(_, path) => path.display_fmt_with_db(f, db),
            ItemPath::ImplBlock(_path) => todo!(),
            ItemPath::Attr(_, _) => todo!(),
            ItemPath::Script(_, _) => todo!(),
        }
    }
}

impl From<MajorFormPath> for ItemPath {
    fn from(v: MajorFormPath) -> Self {
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
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum PatternPath {
    Type(TypePath),
    TypeVariant(TypeVariantPath),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum MajorEntityPath {
    Module(ModulePath),
    MajorItem(MajorItemPath),
}

impl From<MajorFormPath> for MajorEntityPath {
    fn from(v: MajorFormPath) -> Self {
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
    pub fn crate_path(self, db: &::salsa::Db) -> CratePath {
        match self {
            MajorEntityPath::Module(path) => path.crate_path(db),
            MajorEntityPath::MajorItem(path) => path.crate_path(db),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
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

    pub fn item_kind(self, db: &::salsa::Db) -> EntityKind {
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
        match path {
            PrincipalEntityPath::Module(path) => path.into(),
            PrincipalEntityPath::MajorItem(path) => path.into(),
            PrincipalEntityPath::TypeVariant(path) => path.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum PrincipalItemPath {
    Submodule(SubmoduleItemPath),
    Major(MajorItemPath),
    TypeVariant(TypeVariantPath),
}

impl From<PrincipalItemPath> for ItemPath {
    fn from(path: PrincipalItemPath) -> Self {
        match path {
            PrincipalItemPath::Submodule(path) => path.into(),
            PrincipalItemPath::Major(path) => path.into(),
            PrincipalItemPath::TypeVariant(path) => path.into(),
        }
    }
}

impl From<MajorFormPath> for PrincipalItemPath {
    fn from(path: MajorFormPath) -> Self {
        PrincipalItemPath::Major(path.into())
    }
}

impl From<TypePath> for PrincipalItemPath {
    fn from(path: TypePath) -> Self {
        PrincipalItemPath::Major(path.into())
    }
}
