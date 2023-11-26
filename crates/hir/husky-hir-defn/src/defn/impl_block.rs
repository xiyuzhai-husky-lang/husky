pub mod trai_for_ty_impl_block;
pub mod ty_impl_block;

pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
#[salsa::debug_with_db(db = HirDefnDb, jar = HirDefnJar)]
pub enum ImplBlockHirDefn {
    Type(TypeImplBlockHirDefn),
    TraitForType(TraitForTypeImplBlockHirDefn),
}

impl ImplBlockHirDefn {
    pub fn path(self, db: &dyn HirDefnDb) -> ImplBlockPath {
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

    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        // ad hoc
        match self {
            ImplBlockHirDefn::Type(hir_defn) => hir_defn.dependencies(db),
            ImplBlockHirDefn::TraitForType(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        match self {
            ImplBlockHirDefn::Type(hir_defn) => hir_defn.version_stamp(db),
            ImplBlockHirDefn::TraitForType(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for ImplBlockPath {
    type HirDefn = ImplBlockHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        match self {
            ImplBlockPath::TypeImplBlock(path) => path.hir_defn(db).map(Into::into),
            ImplBlockPath::TraitForTypeImplBlock(path) => path.hir_defn(db).map(Into::into),
        }
    }
}
