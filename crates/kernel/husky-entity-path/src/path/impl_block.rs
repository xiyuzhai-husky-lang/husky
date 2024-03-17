pub mod trai_for_ty_impl_block;
pub mod ty_impl_block;

use self::{
    trai_for_ty_impl_block::{TraitForTypeImplBlockPath, TraitForTypeImplBlockPathData},
    ty_impl_block::{TypeImplBlockPath, TypeImplBlockPathData},
};
use super::*;
use vec_like::VecPairMap;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ImplBlockPath {
    TypeImplBlock(TypeImplBlockPath),
    TraitForTypeImplBlock(TraitForTypeImplBlockPath),
}

#[test]
fn impl_block_path_size_works() {
    assert_eq!(
        std::mem::size_of::<ImplBlockPath>(),
        std::mem::size_of::<[u32; 2]>()
    )
}

impl std::ops::Deref for ImplBlockPath {
    type Target = ItemPathId;

    fn deref(&self) -> &Self::Target {
        unsafe { &std::mem::transmute::<_, &(u32, ItemPathId)>(self).1 }
    }
}

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ImplBlockPathData {
    TypeImplBlock(TypeImplBlockPathData),
    TraitForTypeImplBlock(TraitForTypeImplBlockPathData),
}

impl ImplBlockPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> ImplBlockPath {
        match self {
            ImplBlockPathData::TypeImplBlock(slf) => slf.item_path(id).into(),
            ImplBlockPathData::TraitForTypeImplBlock(slf) => slf.item_path(id).into(),
        }
    }

    pub fn module_path(self, _db: &::salsa::Db) -> ModulePath {
        match self {
            ImplBlockPathData::TypeImplBlock(data) => data.module_path(),
            ImplBlockPathData::TraitForTypeImplBlock(data) => data.module_path(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub enum TypeSketch {
    DeriveAny,
    // { ty_kind: Option<TypeKind> }
    Path(TypePath),
}

impl TypeSketch {
    fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        match self {
            TypeSketch::DeriveAny => f.write_str("#derive _"),
            TypeSketch::Path(ty_path) => ty_path.show_aux(f, db),
        }
    }
}

#[derive(Default)]
pub struct ImplBlockRegistry {
    next_disambiguitors: VecPairMap<(ModulePath, Result<ImplBlockKind, ()>), u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub enum ImplBlockKind {
    Type {
        ty_path: TypePath,
    },
    TraitForType {
        ty_sketch: TypeSketch,
        trai_path: TraitPath,
    },
}

impl ImplBlockRegistry {
    fn issue_disambiguitor(
        &mut self,
        module_path: ModulePath,
        impl_block_kind: ImplBlockKind,
    ) -> u8 {
        let next_disambiguitor = self
            .next_disambiguitors
            .get_value_mut_or_insert_default((module_path, Ok(impl_block_kind)));
        let new_disambiguitor = *next_disambiguitor;
        *next_disambiguitor += 1;
        new_disambiguitor
    }

    pub fn issue_ill_formed_disambiguitor(&mut self, module_path: ModulePath) -> u8 {
        let next_disambiguitor = self
            .next_disambiguitors
            .get_value_mut_or_insert_default((module_path, Err(())));
        let new_disambiguitor = *next_disambiguitor;
        *next_disambiguitor += 1;
        new_disambiguitor
    }
}
