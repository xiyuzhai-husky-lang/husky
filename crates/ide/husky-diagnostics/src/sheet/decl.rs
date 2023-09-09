use super::*;
use husky_syn_decl::*;
use husky_syn_expr::SynExprError;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct DeclDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn decl_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> DeclDiagnosticSheet {
    let mut collector = ModuleDiagnosticsCollector::new(db, module_path);
    if let (Ok(ranged_token_sheet), Ok(syn_node_decl_sheet)) = (
        db.ranged_token_sheet(module_path),
        db.syn_node_decl_sheet(module_path),
    ) {
        for (_, syn_node_decl) in syn_node_decl_sheet.decls(db).iter().copied() {
            for error in syn_node_decl.node_decl_errors(db) {
                if let SynNodeDeclError::Original(error) = error {
                    collector.visit_atom(error)
                }
            }
        }
    }
    DeclDiagnosticSheet::new(db, collector.finish())
}

impl Diagnose for OriginalSynNodeDeclError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, ctx: &Self::Context<'_>) -> String {
        match self {
            OriginalSynNodeDeclError::Expr(e) => e.message(ctx),
            OriginalSynNodeDeclError::ExpectedOutputType(_) => {
                format!("Syntax Error: expect output type")
            }
            OriginalSynNodeDeclError::ExpectedCurry(_) => {
                format!("Syntax Error: expect `->`",)
            }
            OriginalSynNodeDeclError::ExpectedEolColon(_e) => {
                format!("Syntax Error: expect end-of-line colon",)
            }
            OriginalSynNodeDeclError::ExpectedRightCurlyBrace(_) => {
                format!("Syntax Error: expect `}}`",)
            }
            OriginalSynNodeDeclError::ExpectedRightAngleBracketForImplicitParameterDeclList {
                ..
            } => {
                format!("Syntax Error: expect `>` for implicit parameter declaration list",)
            }
            OriginalSynNodeDeclError::ExpectedParameterDeclList(_) => {
                format!("Syntax Error: ExpectParameterDeclList",)
            }
            OriginalSynNodeDeclError::ExpectedImplicitParameterDecl(_) => {
                format!("Syntax Error: expect implicit parameter declaration",)
            }
            OriginalSynNodeDeclError::ExpectedRightParenthesisInParameterList(_) => {
                format!("Syntax Error: expected `)` in parameter list",)
            }
            OriginalSynNodeDeclError::ExpectedRightParenthesisInTupleStructFieldTypeList(_) => {
                format!("Syntax Error: expected `)` in tuple struct field type list",)
            }
            OriginalSynNodeDeclError::ExpectedVariableType(_) => {
                format!("Syntax Error: ExpectVariableType",)
            }
            OriginalSynNodeDeclError::ExpectEqTokenForVariable(_) => {
                format!("Syntax Error: ExpectEqTokenForVariable",)
            }
            OriginalSynNodeDeclError::ExpectedLeftCurlyBraceOrLeftParenthesisOrSemicolonForStruct(
                _,
            ) => {
                format!("Syntax Error: expected `{{` `(` or `;` for struct",)
            }
            OriginalSynNodeDeclError::ExpectedEqForAssociatedType(_) => todo!(),
            OriginalSynNodeDeclError::ExpectLeftBracketInDerive(_) => todo!(),
            OriginalSynNodeDeclError::ExpectRightBracketInDerive(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        match self {
            OriginalSynNodeDeclError::Expr(error) => error.range(ctx),
            OriginalSynNodeDeclError::ExpectedOutputType(token_stream_state)
            | OriginalSynNodeDeclError::ExpectedCurry(token_stream_state)
            | OriginalSynNodeDeclError::ExpectedEolColon(token_stream_state)
            | OriginalSynNodeDeclError::ExpectedRightCurlyBrace(token_stream_state)
            | OriginalSynNodeDeclError::ExpectedRightAngleBracketForImplicitParameterDeclList {
                token_stream_state,
                ..
            }
            | OriginalSynNodeDeclError::ExpectedParameterDeclList(token_stream_state)
            | OriginalSynNodeDeclError::ExpectedImplicitParameterDecl(token_stream_state)
            | OriginalSynNodeDeclError::ExpectedRightParenthesisInParameterList(token_stream_state)
            | OriginalSynNodeDeclError::ExpectedRightParenthesisInTupleStructFieldTypeList(
                token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedVariableType(token_stream_state)
            | OriginalSynNodeDeclError::ExpectEqTokenForVariable(token_stream_state)
            | OriginalSynNodeDeclError::ExpectedLeftCurlyBraceOrLeftParenthesisOrSemicolonForStruct(
                token_stream_state,
            ) => ctx.token_stream_state_text_range(*token_stream_state),
            OriginalSynNodeDeclError::ExpectedEqForAssociatedType(_) => todo!(),
            OriginalSynNodeDeclError::ExpectLeftBracketInDerive(_) => todo!(),
            OriginalSynNodeDeclError::ExpectRightBracketInDerive(_) => todo!(),
        }
    }
}
