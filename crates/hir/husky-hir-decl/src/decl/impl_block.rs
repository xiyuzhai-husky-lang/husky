mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum ImplBlockHirDecl {
    Type(TypeImplBlockHirDecl),
    TraitForType(TraitForTypeImplBlockHirDecl),
}

impl HasHirDecl for ImplBlockPath {
    type HirDecl = ImplBlockHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        match self {
            ImplBlockPath::TypeImplBlock(path) => path.hir_decl(db).map(Into::into),
            ImplBlockPath::TraitForTypeImplBlock(path) => path.hir_decl(db).map(Into::into),
        }
    }
}

impl ImplBlockHirDecl {
    pub fn path(self, db: &::salsa::Db) -> ImplBlockPath {
        match self {
            ImplBlockHirDecl::Type(decl) => decl.path(db).into(),
            ImplBlockHirDecl::TraitForType(_) => todo!(),
        }
    }

    pub fn hir_eager_expr_region(self, db: &::salsa::Db) -> HirEagerExprRegion {
        match self {
            ImplBlockHirDecl::Type(slf) => slf.hir_eager_expr_region(db),
            ImplBlockHirDecl::TraitForType(slf) => slf.hir_eager_expr_region(db),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a HirTemplateParameters {
        match self {
            ImplBlockHirDecl::Type(slf) => slf.template_parameters(db),
            ImplBlockHirDecl::TraitForType(slf) => slf.template_parameters(db),
        }
    }
}
