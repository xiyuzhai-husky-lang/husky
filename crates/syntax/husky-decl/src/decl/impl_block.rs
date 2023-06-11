mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use trai_for_ty_impl_block::*;
pub use ty_impl_block::*;

use super::*;

use husky_token::ImplToken;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum ImplBlockRawDecl {
    Type(TypeImplBlockRawDecl),
    TraitForType(TraitForTypeImplBlockRawDecl),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum ImplBlockDecl {
    Type(TypeImplBlockDecl),
    TraitForType(TraitForTypeImplBlockDecl),
}

impl HasDecl for ImplBlock {
    type Decl = ImplBlockDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        impl_block_decl(db, self)
    }
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
    impl_block: ImplBlock,
) -> DeclResultRef<ImplBlockDecl> {
    match impl_block {
        ImplBlock::Type(impl_block) => impl_block.decl(db).map(Into::into),
        ImplBlock::TraitForType(impl_block) => impl_block.decl(db).map(Into::into),
        ImplBlock::IllFormed(_) => Err(&DeclError::Derived(DerivedDeclError::ImplErr)),
    }
}
