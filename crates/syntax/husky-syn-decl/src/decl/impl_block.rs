mod ill_formed;
mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::ill_formed::*;
pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use super::*;
use husky_token::ImplToken;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum ImplBlockSynNodeDecl {
    Type(TypeImplBlockSynNodeDecl),
    TraitForType(TraitForTypeImplBlockSynNodeDecl),
    IllFormed(IllFormedImplBlockSynNodeDecl),
}

impl ImplBlockSynNodeDecl {
    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> ImplBlockSynNodePath {
        match self {
            ImplBlockSynNodeDecl::Type(decl) => decl.syn_node_path(db).into(),
            ImplBlockSynNodeDecl::TraitForType(decl) => decl.syn_node_path(db).into(),
            ImplBlockSynNodeDecl::IllFormed(decl) => decl.syn_node_path(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn SynDeclDb) -> AstIdx {
        match self {
            ImplBlockSynNodeDecl::Type(decl) => decl.ast_idx(db),
            ImplBlockSynNodeDecl::TraitForType(decl) => decl.ast_idx(db),
            ImplBlockSynNodeDecl::IllFormed(decl) => decl.ast_idx(db),
        }
    }

    pub fn generic_parameters<'a>(self, _db: &'a dyn SynDeclDb) -> &'a [GenericParameterDecl] {
        todo!()
    }

    pub fn expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            ImplBlockSynNodeDecl::Type(syn_node_decl) => syn_node_decl.expr_region(db),
            ImplBlockSynNodeDecl::TraitForType(syn_node_decl) => syn_node_decl.expr_region(db),
            ImplBlockSynNodeDecl::IllFormed(syn_node_decl) => syn_node_decl.expr_region(db),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        match self {
            ImplBlockSynNodeDecl::Type(syn_node_decl) => syn_node_decl.errors(db),
            ImplBlockSynNodeDecl::TraitForType(syn_node_decl) => syn_node_decl.errors(db),
            ImplBlockSynNodeDecl::IllFormed(syn_node_decl) => syn_node_decl.errors(db),
        }
    }
}

impl HasNodeDecl for ImplBlockSynNodePath {
    type NodeDecl = ImplBlockSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        match self {
            ImplBlockSynNodePath::TypeImplBlock(syn_node_path) => {
                syn_node_path.syn_node_decl(db).into()
            }
            ImplBlockSynNodePath::TraitForTypeImplBlock(syn_node_path) => {
                syn_node_path.syn_node_decl(db).into()
            }
            ImplBlockSynNodePath::IllFormedImplBlock(syn_node_path) => {
                syn_node_path.syn_node_decl(db).into()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum ImplBlockSynDecl {
    Type(TypeImplBlockSynDecl),
    TraitForType(TraitForTypeImplBlockSynDecl),
}

impl ImplBlockSynDecl {
    pub fn path(self, db: &dyn SynDeclDb) -> ImplBlockPath {
        match self {
            ImplBlockSynDecl::Type(decl) => decl.path(db).into(),
            ImplBlockSynDecl::TraitForType(_) => todo!(),
        }
    }

    pub fn generic_parameters<'a>(self, _db: &'a dyn SynDeclDb) -> &'a [GenericParameterDecl] {
        todo!()
    }

    pub fn expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            ImplBlockSynDecl::Type(decl) => decl.expr_region(db),
            ImplBlockSynDecl::TraitForType(decl) => decl.expr_region(db),
        }
    }
}

impl HasSynDecl for ImplBlockPath {
    type Decl = ImplBlockSynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        match self {
            ImplBlockPath::TypeImplBlock(path) => path.syn_decl(db).map(Into::into),
            ImplBlockPath::TraitForTypeImplBlock(path) => path.syn_decl(db).map(Into::into),
        }
    }
}

pub(crate) fn impl_block_syn_decl(
    db: &dyn SynDeclDb,
    impl_block: ImplBlockPath,
) -> DeclResult<ImplBlockSynDecl> {
    todo!()
    // match impl_block {
    //     ImplBlockNode::TypeImplBlock(impl_block) => impl_block.decl(db).map(Into::into),
    //     ImplBlockNode::TraitForTypeImplBlock(impl_block) => impl_block.decl(db).map(Into::into),
    //     ImplBlockNode::IllFormedImplBlock(_) => Err(&DeclError::Derived(DerivedDeclError::ImplErr)),
    // }
}
