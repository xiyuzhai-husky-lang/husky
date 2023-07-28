mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb)]
#[enum_class::from_variants]
pub enum ImplBlockHirDecl {
    Type(TypeImplBlockHirDecl),
    TraitForType(TraitForTypeImplBlockHirDecl),
}

impl HasHirDecl for ImplBlockPath {
    type HirDecl = ImplBlockHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Self::HirDecl {
        todo!()
    }
}

impl ImplBlockHirDecl {
    pub fn path(self, db: &dyn HirDeclDb) -> ImplBlockPath {
        match self {
            ImplBlockHirDecl::Type(decl) => decl.path(db).into(),
            ImplBlockHirDecl::TraitForType(_) => todo!(),
        }
    }

    pub fn template_parameters<'a>(
        self,
        _db: &'a dyn HirDeclDb,
    ) -> &'a [EtherealTemplateParameter] {
        todo!()
    }

    pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> HirExprRegion {
        match self {
            ImplBlockHirDecl::Type(decl) => decl.hir_expr_region(db).into(),
            ImplBlockHirDecl::TraitForType(decl) => decl.hir_expr_region(db).into(),
        }
    }
}
