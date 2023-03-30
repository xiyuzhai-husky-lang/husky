mod trai_for_ty_item;
mod trai_item;
mod ty_item;

use husky_entity_taxonomy::{AssociatedItemKind, EntityKind, TraitItemKind, TypeItemKind};
pub use trai_for_ty_item::*;
pub use trai_item::*;
pub use ty_item::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum AssociatedItemDecl {
    TypeItem(TypeItemDecl),
    TraitItem(TraitItemDecl),
    TraitForTypeItem(TypeAsTraitItemDecl),
}

impl AssociatedItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.ast_idx(db),
            AssociatedItemDecl::TraitItem(decl) => decl.ast_idx(db),
            AssociatedItemDecl::TraitForTypeItem(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.implicit_parameters(db),
            AssociatedItemDecl::TraitItem(decl) => decl.implicit_parameters(db),
            AssociatedItemDecl::TraitForTypeItem(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.expr_region(db),
            AssociatedItemDecl::TraitItem(decl) => decl.expr_region(db),
            AssociatedItemDecl::TraitForTypeItem(decl) => decl.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> Option<AssociatedItemPath> {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.path(db).map(|path| path.into()),
            AssociatedItemDecl::TraitItem(decl) => Some(decl.path(db).into()),
            AssociatedItemDecl::TraitForTypeItem(decl) => decl.path(db).map(|path| path.into()),
        }
    }
}

#[salsa::tracked(jar = DeclJar,return_ref)]
pub(crate) fn associated_item_decl(
    db: &dyn DeclDb,
    associated_item: AssociatedItem,
) -> DeclResult<AssociatedItemDecl> {
    let parser = DeclParseContext::new(db, associated_item.module_path(db))?;
    parser.parse_associated_item_decl(associated_item)
}

impl<'a> DeclParseContext<'a> {
    fn parse_associated_item_decl(
        &self,
        associated_item: AssociatedItem,
    ) -> DeclResult<AssociatedItemDecl> {
        let ast_idx = associated_item.ast_idx(self.db());
        Ok(match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                body: _,
                accessibility: _,
                entity_kind:
                    EntityKind::AssociatedItem {
                        associated_item_kind,
                    },
                entity_path: _,
                ident_token: _,
                is_generic: _,
                body_kind: _,
                saved_stream_state,
            } => match associated_item_kind {
                AssociatedItemKind::TraitItem(_) => todo!(),
                AssociatedItemKind::TypeItem(ty_item_kind) => {
                    AssociatedItemDecl::TypeItem(match ty_item_kind {
                        TypeItemKind::MethodFn => self
                            .parse_ty_method_decl(
                                ast_idx,
                                token_group_idx,
                                associated_item,
                                saved_stream_state,
                            )?
                            .into(),
                        TypeItemKind::AssociatedFn => self
                            .parse_ty_associated_fn_decl(
                                ast_idx,
                                token_group_idx,
                                associated_item,
                                saved_stream_state,
                            )?
                            .into(),
                        TypeItemKind::Memo => self
                            .parse_ty_memo_decl(
                                ast_idx,
                                token_group_idx,
                                associated_item,
                                saved_stream_state,
                            )?
                            .into(),
                    })
                }
                AssociatedItemKind::TraitForTypeItem(trai_for_ty_item_kind) => {
                    AssociatedItemDecl::TraitForTypeItem(match trai_for_ty_item_kind {
                        TraitItemKind::MethodFn => self
                            .parse_trai_for_ty_method_decl(
                                ast_idx,
                                token_group_idx,
                                associated_item,
                                saved_stream_state,
                            )?
                            .into(),
                        TraitItemKind::AssociatedType => todo!(),
                    })
                }
            },
            _ => unreachable!(),
        })
    }

    fn parse_ty_associated_fn_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        associated_item: AssociatedItem,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeAssociatedFnDecl> {
        let Ok(impl_decl) = self.db().impl_block_decl(associated_item.impl_block(self.db()))
        else { return Err(DerivedDeclError::UnableToParseImplDeclForTyMethodDecl.into()) };
        let mut parser = self.expr_parser(
            DeclRegionPath::AssociatedItem(associated_item.id(self.db())),
            Some(impl_decl.expr_region(self.db())),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx2(None, token_group_idx, saved_stream_state);
        let implicit_parameter_decl_list = ctx.parse();
        let path = match associated_item.path(self.db()) {
            Some(AssociatedItemPath::TypeItem(path)) => Some(path),
            None => None,
            _ => unreachable!(),
        };
        let parameter_decl_list =
            ctx.parse_expected(OriginalDeclExprError::ExpectParameterDeclList);
        let curry_token = ctx.parse_expected(OriginalDeclExprError::ExpectCurry);
        let return_ty = ctx.parse_expected(OriginalDeclExprError::ExpectOutputType);
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon);
        Ok(TypeAssociatedFnDecl::new(
            self.db(),
            associated_item.id(self.db()),
            path,
            ast_idx,
            parser.finish(),
            implicit_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
        ))
    }

    fn parse_ty_memo_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        associated_item: AssociatedItem,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeMemoDecl> {
        let Ok(impl_decl) = self.db().impl_block_decl(associated_item.impl_block(self.db()))
        else { todo!() };
        let mut parser = self.expr_parser(
            DeclRegionPath::AssociatedItem(associated_item.id(self.db())),
            Some(impl_decl.expr_region(self.db())),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx2(None, token_group_idx, saved_stream_state);
        let path = match associated_item.path(self.db()) {
            Some(AssociatedItemPath::TypeItem(path)) => Some(path),
            None => None,
            _ => unreachable!(),
        };
        let curry_token = ctx.parse_expected(OriginalDeclExprError::ExpectCurry);
        let return_ty = ctx.parse_expected(OriginalDeclExprError::ExpectOutputType);
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon);
        Ok(TypeMemoDecl::new(
            self.db(),
            path,
            associated_item,
            ast_idx,
            parser.finish(),
            curry_token,
            return_ty,
            eol_colon,
        )
        .into())
    }

    fn parse_trai_for_ty_method_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        associated_item: AssociatedItem,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeAsTraitMethodDecl> {
        let Ok(impl_decl) = self.db().impl_block_decl(associated_item.impl_block(self.db()))
        else {
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
        let mut ctx = parser.ctx2(None, token_group_idx, saved_stream_state);
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
        Ok(TypeAsTraitMethodDecl::new(
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
