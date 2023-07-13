mod ill_formed;
mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::ill_formed::*;
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
    IllFormed(IllFormedImplBlockNodeDecl),
}

impl ImplBlockNodeDecl {
    pub fn node_path(self, db: &dyn DeclDb) -> ImplBlockNodePath {
        match self {
            ImplBlockNodeDecl::Type(decl) => decl.node_path(db).into(),
            ImplBlockNodeDecl::TraitForType(decl) => decl.node_path(db).into(),
            ImplBlockNodeDecl::IllFormed(decl) => decl.node_path(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            ImplBlockNodeDecl::Type(decl) => decl.ast_idx(db),
            ImplBlockNodeDecl::TraitForType(decl) => decl.ast_idx(db),
            ImplBlockNodeDecl::IllFormed(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, _db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        todo!()
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            ImplBlockNodeDecl::Type(node_decl) => node_decl.expr_region(db),
            ImplBlockNodeDecl::TraitForType(node_decl) => node_decl.expr_region(db),
            ImplBlockNodeDecl::IllFormed(node_decl) => node_decl.expr_region(db),
        }
    }

    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        match self {
            ImplBlockNodeDecl::Type(node_decl) => node_decl.errors(db),
            ImplBlockNodeDecl::TraitForType(node_decl) => node_decl.errors(db),
            ImplBlockNodeDecl::IllFormed(node_decl) => node_decl.errors(db),
        }
    }
}

impl HasNodeDecl for ImplBlockNodePath {
    type NodeDecl = ImplBlockNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        match self {
            ImplBlockNodePath::TypeImplBlock(node_path) => node_path.node_decl(db).into(),
            ImplBlockNodePath::TraitForTypeImplBlock(node_path) => node_path.node_decl(db).into(),
            ImplBlockNodePath::IllFormedImplBlock(node_path) => node_path.node_decl(db).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum ImplBlockDecl {
    Type(TypeImplBlockDecl),
    TraitForType(TraitForTypeImplBlockDecl),
}

impl ImplBlockDecl {
    pub fn path(self, db: &dyn DeclDb) -> ImplBlockPath {
        match self {
            ImplBlockDecl::Type(decl) => decl.path(db).into(),
            ImplBlockDecl::TraitForType(_) => todo!(),
        }
    }

    pub fn implicit_parameters<'a>(self, _db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        todo!()
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            ImplBlockDecl::Type(decl) => decl.expr_region(db),
            ImplBlockDecl::TraitForType(decl) => decl.expr_region(db),
        }
    }
}

impl HasDecl for ImplBlockPath {
    type Decl = ImplBlockDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        match self {
            ImplBlockPath::TypeImplBlock(path) => path.decl(db).map(Into::into),
            ImplBlockPath::TraitForTypeImplBlock(path) => path.decl(db).map(Into::into),
        }
    }
}

pub(crate) fn impl_block_decl(
    db: &dyn DeclDb,
    impl_block: ImplBlockPath,
) -> DeclResult<ImplBlockDecl> {
    todo!()
    // match impl_block {
    //     ImplBlockNode::TypeImplBlock(impl_block) => impl_block.decl(db).map(Into::into),
    //     ImplBlockNode::TraitForTypeImplBlock(impl_block) => impl_block.decl(db).map(Into::into),
    //     ImplBlockNode::IllFormedImplBlock(_) => Err(&DeclError::Derived(DerivedDeclError::ImplErr)),
    // }
}
