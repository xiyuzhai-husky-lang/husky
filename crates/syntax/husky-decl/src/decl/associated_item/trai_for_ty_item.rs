mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;

use super::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemRawDecl {
    AssociatedFn(TraitForTypeAssociatedFnRawDecl),
    MethodFn(TraitForTypeMethodFnRawDecl),
    AssociatedType(TraitForTypeAssociatedTypeRawDecl),
    AssociatedVal(TraitForTypeAssociatedValRawDecl),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemDecl {
    AssociatedFn(TraitForTypeAssociatedFnDecl),
    MethodFn(TraitForTypeMethodFnDecl),
    AssociatedType(TraitForTypeAssociatedTypeDecl),
    AssociatedVal(TraitForTypeAssociatedValDecl),
}

impl TraitForTypeItemDecl {
    pub fn node_path(self, db: &dyn DeclDb) -> TraitForTypeItemNodePath {
        match self {
            TraitForTypeItemDecl::AssociatedFn(_) => todo!(),
            TraitForTypeItemDecl::MethodFn(decl) => decl.node_path(db),
            TraitForTypeItemDecl::AssociatedType(decl) => decl.node_path(db),
            TraitForTypeItemDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TraitForTypeItemDecl::AssociatedFn(decl) => decl.ast_idx(db),
            TraitForTypeItemDecl::MethodFn(decl) => decl.ast_idx(db),
            TraitForTypeItemDecl::AssociatedType(decl) => decl.ast_idx(db),
            TraitForTypeItemDecl::AssociatedVal(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(
        self,
        _db: &'a dyn DeclDb,
    ) -> &'a [ImplicitParameterDeclPattern] {
        match self {
            TraitForTypeItemDecl::AssociatedFn(_) => todo!(),
            TraitForTypeItemDecl::MethodFn(_) => todo!(),
            TraitForTypeItemDecl::AssociatedType(_) => todo!(),
            TraitForTypeItemDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            TraitForTypeItemDecl::AssociatedFn(decl) => decl.expr_region(db),
            TraitForTypeItemDecl::MethodFn(decl) => decl.expr_region(db),
            TraitForTypeItemDecl::AssociatedType(decl) => decl.expr_region(db),
            TraitForTypeItemDecl::AssociatedVal(decl) => decl.expr_region(db),
        }
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_trai_for_ty_item_decl(
        &self,
        trai_item_kind: TraitItemKind,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        node: TraitForTypeItemNode,
        saved_stream_state: TokenStreamState,
    ) -> DeclResult<TraitForTypeItemDecl> {
        Ok(match trai_item_kind {
            TraitItemKind::MethodFn => self
                .parse_trai_for_ty_method_fn_decl(
                    ast_idx,
                    token_group_idx,
                    node,
                    saved_stream_state,
                )?
                .into(),
            TraitItemKind::AssociatedType => self
                .parse_trai_for_ty_associated_ty_decl(
                    ast_idx,
                    token_group_idx,
                    node,
                    saved_stream_state,
                )?
                .into(),
        })
    }
}
