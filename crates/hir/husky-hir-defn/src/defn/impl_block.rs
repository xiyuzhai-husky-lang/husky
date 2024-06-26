pub mod trai_for_ty_impl_block;
pub mod ty_impl_block;

use self::trai_for_ty_impl_block::*;
use self::ty_impl_block::*;
use super::*;
use husky_entity_path::path::impl_block::ImplBlockPath;
use husky_hir_decl::decl::ImplBlockHirDecl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
pub enum ImplBlockHirDefn {
    Type(TypeImplBlockHirDefn),
    TraitForType(TraitForTypeImplBlockHirDefn),
}

impl ImplBlockHirDefn {
    pub fn path(self, db: &::salsa::Db) -> ImplBlockPath {
        match self {
            ImplBlockHirDefn::Type(hir_defn) => hir_defn.path(db).into(),
            ImplBlockHirDefn::TraitForType(hir_defn) => hir_defn.path(db).into(),
        }
    }

    pub fn hir_decl(self) -> ImplBlockHirDecl {
        match self {
            ImplBlockHirDefn::Type(hir_defn) => hir_defn.hir_decl().into(),
            ImplBlockHirDefn::TraitForType(hir_defn) => hir_defn.hir_decl().into(),
        }
    }

    pub(super) fn deps(self, db: &::salsa::Db) -> HirDefnDeps {
        // ad hoc
        match self {
            ImplBlockHirDefn::Type(hir_defn) => hir_defn.deps(db),
            ImplBlockHirDefn::TraitForType(hir_defn) => hir_defn.deps(db),
        }
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            ImplBlockHirDefn::Type(hir_defn) => hir_defn.version_stamp(db),
            ImplBlockHirDefn::TraitForType(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for ImplBlockPath {
    type HirDefn = ImplBlockHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        match self {
            ImplBlockPath::TypeImplBlock(path) => path.hir_defn(db).map(Into::into),
            ImplBlockPath::TraitForTypeImplBlock(path) => path.hir_defn(db).map(Into::into),
        }
    }
}
