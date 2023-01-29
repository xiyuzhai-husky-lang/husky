mod ty_as_trai_impl_block;
mod ty_impl_block;

pub use ty_as_trai_impl_block::*;
pub use ty_impl_block::*;

use super::*;
use husky_entity_tree::{ImplBlock, ImplBlockId};
use husky_token::ImplToken;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
pub enum ImplBlockDecl {
    TypeImplBlock(TypeImplBlockDecl),
    TypeAsTraitImplBlock(TypeAsTraitImplBlockDecl),
}

impl From<TypeAsTraitImplBlockDecl> for ImplBlockDecl {
    fn from(v: TypeAsTraitImplBlockDecl) -> Self {
        Self::TypeAsTraitImplBlock(v)
    }
}

impl From<TypeImplBlockDecl> for ImplBlockDecl {
    fn from(v: TypeImplBlockDecl) -> Self {
        Self::TypeImplBlock(v)
    }
}

impl ImplBlockDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            ImplBlockDecl::TypeImplBlock(decl) => decl.ast_idx(db),
            ImplBlockDecl::TypeAsTraitImplBlock(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        todo!()
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            ImplBlockDecl::TypeImplBlock(decl) => decl.expr_region(db),
            ImplBlockDecl::TypeAsTraitImplBlock(decl) => decl.expr_region(db),
        }
    }
}
