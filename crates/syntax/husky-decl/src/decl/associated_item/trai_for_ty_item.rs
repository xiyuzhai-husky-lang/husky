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
pub enum TraitForTypeItemDecl {
    AssociatedFn(TraitForTypeAssociatedFnDecl),
    MethodFn(TraitForTypeMethodFnDecl),
    AssociatedType(TraitForTypeAssociatedTypeDecl),
    AssociatedVal(TraitForTypeAssociatedValDecl),
}

impl TraitForTypeItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TraitForTypeItemDecl::AssociatedFn(decl) => decl.ast_idx(db),
            TraitForTypeItemDecl::MethodFn(decl) => decl.ast_idx(db),
            TraitForTypeItemDecl::AssociatedType(decl) => decl.ast_idx(db),
            TraitForTypeItemDecl::AssociatedVal(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, _db: &'a dyn DeclDb) -> &'a [ImplicitParameterDecl] {
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

    pub fn path(self, db: &dyn DeclDb) -> Option<TraitForTypeItemPath> {
        match self {
            TraitForTypeItemDecl::AssociatedFn(_) => todo!(),
            TraitForTypeItemDecl::MethodFn(decl) => decl.path(db),
            TraitForTypeItemDecl::AssociatedType(_) => todo!(),
            TraitForTypeItemDecl::AssociatedVal(_) => todo!(),
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
    ) -> DeclResult<TraitForTypeMethodFnDecl> {
        let Ok(impl_decl) = associated_item.impl_block(self.db()).decl(
            self.db()
        ) else {
            return Err(
                DerivedDeclError::UnableToParseImplDeclForTyAsTraitMethodFnDecl.into()
            )
        };
        let mut parser = self.expr_parser(
            DeclRegionPath::AssociatedItem(associated_item.id(self.db())),
            Some(impl_decl.expr_region(self.db())),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let implicit_parameter_decl_list = ctx.parse()?;
        let path = match associated_item.path(self.db()) {
            Some(AssociatedItemPath::TraitForTypeItem(path)) => Some(path),
            None => None,
            _ => unreachable!(),
        };
        let parameter_decl_list =
            ctx.parse_expected(OriginalDeclExprError::ExpectParameterDeclList)?;

        let curry_token = ctx.parse()?;
        let return_ty = if curry_token.is_some() {
            Some(ctx.parse_expected(OriginalDeclExprError::ExpectOutputType)?)
        } else {
            None
        };
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon)?;
        Ok(TraitForTypeMethodFnDecl::new(
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
