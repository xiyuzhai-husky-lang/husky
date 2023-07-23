mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TraitItemSynNodeDecl {
    AssociatedFn(TraitAssociatedFnSynNodeDecl),
    MethodFn(TraitMethodFnSynNodeDecl),
    AssociatedType(TraitAssociatedTypeSynNodeDecl),
    AssociatedVal(TraitAssociatedValSynNodeDecl),
}

impl TraitItemSynNodeDecl {
    pub fn syn_node_path(self, _db: &dyn DeclDb) -> TraitItemSynNodePath {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(_) => todo!(),
            TraitItemSynNodeDecl::MethodFn(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedType(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(decl) => decl.ast_idx(db),
            TraitItemSynNodeDecl::MethodFn(decl) => decl.ast_idx(db),
            TraitItemSynNodeDecl::AssociatedType(decl) => decl.ast_idx(db),
            TraitItemSynNodeDecl::AssociatedVal(decl) => decl.ast_idx(db),
        }
    }

    pub fn generic_parameters<'a>(self, _db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(_) => todo!(),
            TraitItemSynNodeDecl::MethodFn(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedType(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn expr_region(self, _db: &dyn DeclDb) -> SynExprRegion {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(_) => todo!(),
            TraitItemSynNodeDecl::MethodFn(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedType(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(_) => todo!(),
            TraitItemSynNodeDecl::MethodFn(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedType(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedVal(_) => todo!(),
        }
    }
}

impl HasNodeDecl for TraitItemSynNodePath {
    type NodeDecl = TraitItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TraitItemSynDecl {
    AssociatedFn(TraitAssociatedFnSynDecl),
    MethodFn(TraitMethodFnSynDecl),
    AssociatedType(TraitAssociatedTypeSynDecl),
    AssociatedVal(TraitAssociatedValSynDecl),
}

impl TraitItemSynDecl {
    pub fn path(self, _db: &dyn DeclDb) -> TraitItemPath {
        match self {
            TraitItemSynDecl::AssociatedFn(_) => todo!(),
            TraitItemSynDecl::MethodFn(_) => todo!(),
            TraitItemSynDecl::AssociatedType(_) => todo!(),
            TraitItemSynDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn generic_parameters<'a>(self, _db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        match self {
            TraitItemSynDecl::AssociatedFn(_) => todo!(),
            TraitItemSynDecl::MethodFn(_) => todo!(),
            TraitItemSynDecl::AssociatedType(_) => todo!(),
            TraitItemSynDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn expr_region(self, _db: &dyn DeclDb) -> SynExprRegion {
        match self {
            TraitItemSynDecl::AssociatedFn(_) => todo!(),
            TraitItemSynDecl::MethodFn(_) => todo!(),
            TraitItemSynDecl::AssociatedType(_) => todo!(),
            TraitItemSynDecl::AssociatedVal(_) => todo!(),
        }
    }
}

impl HasSynDecl for TraitItemPath {
    type Decl = TraitItemSynDecl;

    fn syn_decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        todo!()
    }
}

impl<'a> DeclParser<'a> {}
