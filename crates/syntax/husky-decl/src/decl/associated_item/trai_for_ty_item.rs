mod assoc_ty;
mod assoc_val;
mod function;
mod method_fn;

pub use assoc_ty::*;
pub use assoc_val::*;
pub use function::*;
pub use method_fn::*;

use super::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemDecl {
    AssociatedFunction(TraitForTypeAssociatedFunctionDecl),
    Method(TraitForTypeMethodDecl),
    AssociatedType(TraitForTypeAssociatedTypeDecl),
    AssociatedValue(TraitForTypeAssociatedValueDecl),
}

impl TraitForTypeItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TraitForTypeItemDecl::AssociatedFunction(decl) => decl.ast_idx(db),
            TraitForTypeItemDecl::Method(decl) => decl.ast_idx(db),
            TraitForTypeItemDecl::AssociatedType(decl) => decl.ast_idx(db),
            TraitForTypeItemDecl::AssociatedValue(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(
        self,
        _db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        match self {
            TraitForTypeItemDecl::AssociatedFunction(_) => todo!(),
            TraitForTypeItemDecl::Method(_) => todo!(),
            TraitForTypeItemDecl::AssociatedType(_) => todo!(),
            TraitForTypeItemDecl::AssociatedValue(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            TraitForTypeItemDecl::AssociatedFunction(decl) => decl.expr_region(db),
            TraitForTypeItemDecl::Method(decl) => decl.expr_region(db),
            TraitForTypeItemDecl::AssociatedType(decl) => decl.expr_region(db),
            TraitForTypeItemDecl::AssociatedValue(decl) => decl.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> Option<TraitForTypeItemPath> {
        match self {
            TraitForTypeItemDecl::AssociatedFunction(_) => todo!(),
            TraitForTypeItemDecl::Method(decl) => decl.path(db),
            TraitForTypeItemDecl::AssociatedType(_) => todo!(),
            TraitForTypeItemDecl::AssociatedValue(_) => todo!(),
        }
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_trai_for_ty_method_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        associated_item: AssociatedItem,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TraitForTypeMethodDecl> {
        let Ok(impl_decl) = associated_item.impl_block(self.db()).decl(
            self.db()
        ) else {
            return Err(
                DerivedDeclError::UnableToParseImplDeclForTyAsTraitMethodDecl.into()
            )
        };
        let mut parser = self.expr_parser(
            DeclRegionPath::AssociatedItem(associated_item.id(self.db())),
            Some(impl_decl.expr_region(self.db())),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let implicit_parameter_decl_list = ctx.parse();
        let path = match associated_item.path(self.db()) {
            Some(AssociatedItemPath::TraitForTypeItem(path)) => Some(path),
            None => None,
            _ => unreachable!(),
        };
        let parameter_decl_list =
            ctx.parse_expected(OriginalDeclExprError::ExpectParameterDeclList);
        let curry_token = ctx.parse_expected(OriginalDeclExprError::ExpectCurry);
        let return_ty = ctx.parse_expected(OriginalDeclExprError::ExpectOutputType);
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon);
        Ok(TraitForTypeMethodDecl::new(
            self.db(),
            path,
            associated_item,
            ast_idx,
            parser.finish(),
            implicit_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
        )
        .into())
    }
}
