use super::*;
use husky_ast::Ast;
use husky_entity_syn_tree::{
    helpers::tokra_region::HasDeclTokraRegion, EntitySynTreeDb, EntitySynTreeError,
    ImplBlockIllForm, MajorPathExprError, OriginalEntityTreeError, OriginalMajorPathExprError,
};

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct EntityTreeDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn item_tree_diagnostic_sheet(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> EntityTreeDiagnosticSheet {
    let mut diagnostics = vec![];
    let ctx = SheetDiagnosticsContext::new(db, module_path);
    let item_syn_tree_sheet = db.item_syn_tree_sheet(module_path);
    for e in item_syn_tree_sheet.errors() {
        match e {
            EntitySynTreeError::Original(e) => diagnostics.push(e.to_diagnostic(&ctx)),
            EntitySynTreeError::Derived(_) => (),
        }
    }
    for impl_block_ill_form in item_syn_tree_sheet.all_impl_block_ill_forms(db) {
        diagnostics.push(impl_block_ill_form.to_diagnostic(&ctx))
    }
    // todo
    EntityTreeDiagnosticSheet::new(db, diagnostics)
}

impl Diagnose for OriginalEntityTreeError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, _db: &Self::Context<'_>) -> String {
        match self {
            OriginalEntityTreeError::UnresolvedRootIdent(_) => format!("unresolved identifier"),
            OriginalEntityTreeError::NoVisibleSubitem => format!("NoSubitem"),
            OriginalEntityTreeError::EntitySymbolAlreadyDefined { old: _, new: _ } => {
                format!("EntitySymbolAlreadyDefined")
            }
            OriginalEntityTreeError::ExpectIdentAfterKeyword => {
                format!("ExpectIdentAfterKeyword")
            }
            OriginalEntityTreeError::InvalidTypePath(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        let db = ctx.db();
        match self {
            OriginalEntityTreeError::UnresolvedRootIdent(ident_token) => {
                ctx.token_idx_text_range(ident_token.token_idx())
            }
            OriginalEntityTreeError::NoVisibleSubitem => todo!(),
            OriginalEntityTreeError::EntitySymbolAlreadyDefined { old: _, new } => {
                match ctx.ast_sheet()[new.decl_ast_idx(db)] {
                    Ast::Use {
                        token_group_idx: _,
                        visibility_expr: _,
                        state_after_visibility_expr: _,
                    } => todo!(),
                    Ast::Identifiable { ident_token, .. }
                    | Ast::TypeVariant { ident_token, .. } => {
                        ctx.token_idx_text_range(ident_token.token_idx())
                    }
                    _ => unreachable!(),
                }
            }
            OriginalEntityTreeError::ExpectIdentAfterKeyword => todo!(),
            OriginalEntityTreeError::InvalidTypePath(_) => todo!(),
        }
    }
}

impl Diagnose for ImplBlockIllForm {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, ctx: &Self::Context<'_>) -> String {
        match self {
            ImplBlockIllForm::UnmatchedAngleBras => todo!(),
            ImplBlockIllForm::TokenData(_) => todo!(),
            ImplBlockIllForm::MajorPath(e) => match e {
                MajorPathExprError::Original(e) => match e {
                    OriginalMajorPathExprError::UnrecognizedIdent(ident_token) => {
                        format!(
                            "Syntax Error: unrecognized identifier `{}` for major entity path",
                            ident_token.ident().data(ctx.db())
                        )
                    }
                    OriginalMajorPathExprError::ExpectedName(_) => format!("expected identifier",),
                    OriginalMajorPathExprError::NoSuchSubitem => todo!(),
                },
                MajorPathExprError::Derived(_) => todo!(),
            },
            ImplBlockIllForm::MissingForKeyword => format!("missing `for` keyword"),
            ImplBlockIllForm::ExpectTypePathAfterForKeyword => {
                format!("expect type path after `for` keyword")
            }
            ImplBlockIllForm::ExpectedDeriveIdent(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        match self {
            ImplBlockIllForm::UnmatchedAngleBras => todo!(),
            ImplBlockIllForm::TokenData(_) => todo!(),
            ImplBlockIllForm::MajorPath(e) => match e {
                MajorPathExprError::Original(e) => match e {
                    OriginalMajorPathExprError::UnrecognizedIdent(ident_token) => {
                        ctx.token_idx_text_range(ident_token.token_idx())
                    }
                    OriginalMajorPathExprError::ExpectedName(token_stream_state) => {
                        match token_stream_state.drained() {
                            true => todo!(),
                            false => ctx.token_idx_text_range(token_stream_state.next_token_idx()),
                        }
                    }
                    OriginalMajorPathExprError::NoSuchSubitem => todo!(),
                },
                MajorPathExprError::Derived(_) => todo!(),
            },
            ImplBlockIllForm::MissingForKeyword => todo!(),
            ImplBlockIllForm::ExpectTypePathAfterForKeyword => todo!(),
            ImplBlockIllForm::ExpectedDeriveIdent(_) => todo!(),
        }
    }
}
