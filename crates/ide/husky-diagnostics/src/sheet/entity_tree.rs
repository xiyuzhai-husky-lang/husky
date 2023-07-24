use super::*;
use husky_item_tree::{
    EntityTreeError, IllFormedImplBlockSynNode, ImplBlockIllForm, MajorPathExprError,
    OnceUseRuleState, OriginalEntityTreeError, OriginalMajorPathExprError,
};
use salsa::DebugWithDb;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct EntityTreeDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn item_tree_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> EntityTreeDiagnosticSheet {
    let mut diagnostics = vec![];
    let ctx = SheetDiagnosticsContext::new(db, module_path);
    if let Ok(item_tree_sheet) = db.item_syn_tree_sheet(module_path) {
        for e in item_tree_sheet.errors() {
            match e {
                EntityTreeError::Original(e) => diagnostics.push(e.to_diagnostic(&ctx)),
                EntityTreeError::Derived(_) => (),
            }
        }
        for ill_formed_impl_block in item_tree_sheet.all_ill_formed_impl_block_syn_nodes() {
            diagnostics.push(ill_formed_impl_block.to_diagnostic(&ctx))
        }
    }
    // todo
    EntityTreeDiagnosticSheet::new(db, diagnostics)
}

impl Diagnose for OriginalEntityTreeError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, db: &Self::Context<'_>) -> String {
        match self {
            OriginalEntityTreeError::UnresolvedRootIdent(_) => format!("unresolved identifier"),
            OriginalEntityTreeError::NoVisibleSubitem => format!("NoSubitem"),
            OriginalEntityTreeError::EntitySymbolAlreadyDefined { old, new } => {
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
        match self {
            OriginalEntityTreeError::UnresolvedRootIdent(ident_token) => {
                ctx.token_idx_text_range(ident_token.token_idx())
            }
            OriginalEntityTreeError::NoVisibleSubitem => todo!(),
            OriginalEntityTreeError::EntitySymbolAlreadyDefined { old, new } => {
                ctx.token_idx_text_range(new.ident_token(ctx.db()).token_idx())
            }
            OriginalEntityTreeError::ExpectIdentAfterKeyword => todo!(),
            OriginalEntityTreeError::InvalidTypePath(_) => todo!(),
        }
    }
}

impl Diagnose for IllFormedImplBlockSynNode {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, ctx: &Self::Context<'_>) -> String {
        match self.ill_form(ctx.db()) {
            ImplBlockIllForm::UnmatchedAngleBras => todo!(),
            ImplBlockIllForm::Token(_) => todo!(),
            ImplBlockIllForm::MajorPath(e) => match e {
                MajorPathExprError::Original(e) => match e {
                    OriginalMajorPathExprError::UnrecognizedIdent(ident_token) => {
                        format!(
                            "Syntax Error: unrecognized identifier `{}` for major item path",
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
        match self.ill_form(ctx.db()) {
            ImplBlockIllForm::UnmatchedAngleBras => todo!(),
            ImplBlockIllForm::Token(_) => todo!(),
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
