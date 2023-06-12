mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use super::*;
use husky_token::ImplToken;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum ImplBlockNodeDecl {
    Type(TypeImplBlockNodeDecl),
    TraitForType(TraitForTypeImplBlockNodeDecl),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum ImplBlockDecl {
    Type(TypeImplBlockDecl),
    TraitForType(TraitForTypeImplBlockDecl),
}

impl HasDecl for ImplBlockNode {
    type Decl = ImplBlockDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        impl_block_decl(db, self)
    }
}

impl ImplBlockDecl {
    pub fn node_path(self, db: &dyn DeclDb) -> ImplBlockNodePath {
        match self {
            ImplBlockDecl::Type(decl) => decl.node_path(db).into(),
            ImplBlockDecl::TraitForType(_) => todo!(),
        }
    }

    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            ImplBlockDecl::Type(decl) => decl.ast_idx(db),
            ImplBlockDecl::TraitForType(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(
        self,
        _db: &'a dyn DeclDb,
    ) -> &'a [ImplicitParameterDeclPattern] {
        todo!()
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            ImplBlockDecl::Type(decl) => decl.expr_region(db),
            ImplBlockDecl::TraitForType(decl) => decl.expr_region(db),
        }
    }
}

pub(crate) fn impl_block_decl(
    db: &dyn DeclDb,
    impl_block: ImplBlockNode,
) -> DeclResultRef<ImplBlockDecl> {
    match impl_block {
        ImplBlockNode::TypeImplBlock(impl_block) => impl_block.decl(db).map(Into::into),
        ImplBlockNode::TraitForTypeImplBlock(impl_block) => impl_block.decl(db).map(Into::into),
        ImplBlockNode::IllFormedImplBlock(_) => Err(&DeclError::Derived(DerivedDeclError::ImplErr)),
    }
}
