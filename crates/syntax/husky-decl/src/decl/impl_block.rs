mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use trai_for_ty_impl_block::*;
pub use ty_impl_block::*;

use super::*;

use husky_token::ImplToken;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum ImplBlockDecl {
    Type(TypeImplBlockDecl),
    TraitForType(TraitForTypeImplBlockDecl),
}

impl ImplBlockDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            ImplBlockDecl::Type(decl) => decl.ast_idx(db),
            ImplBlockDecl::TraitForType(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(
        self,
        _db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        todo!()
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            ImplBlockDecl::Type(decl) => decl.expr_region(db),
            ImplBlockDecl::TraitForType(decl) => decl.expr_region(db),
        }
    }
}
