use super::*;
use husky_decl::*;
use husky_syn_expr::ExprError;

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
    if let (Ok(ranged_token_sheet), Ok(node_decl_sheet)) = (
        db.ranged_token_sheet(module_path),
        db.node_decl_sheet(module_path),
    ) {
        for (_, node_decl) in node_decl_sheet.decls(db).iter().copied() {
            for error in node_decl.errors(db) {
                if let NodeDeclError::Original(error) = error {
                    collector.visit_atom(error)
                }
            }
        }
    }
    DeclDiagnosticSheet::new(db, collector.finish())
}

impl Diagnose for OriginalNodeDeclError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, ctx: &Self::Context<'_>) -> String {
        match self {
            OriginalNodeDeclError::Expr(e) => e.message(ctx),
            OriginalNodeDeclError::ExpectedOutputType(_) => {
                format!("Syntax Error: expect output type")
            }
            OriginalNodeDeclError::ExpectedCurry(_) => {
                format!("Syntax Error: expect `->`",)
            }
            OriginalNodeDeclError::ExpectedEolColon(_e) => {
                format!("Syntax Error: expect end-of-line colon",)
            }
            OriginalNodeDeclError::ExpectedRightCurlyBrace(_) => {
                format!("Syntax Error: expect `}}`",)
            }
            OriginalNodeDeclError::ExpectedRightAngleBracketForImplicitParameterDeclList {
                ..
            } => {
                format!("Syntax Error: expect `>` for implicit parameter declaration list",)
            }
            OriginalNodeDeclError::ExpectedParameterDeclList(_) => {
                format!("Syntax Error: ExpectParameterDeclList",)
            }
            OriginalNodeDeclError::ExpectedImplicitParameterDecl(_) => {
                format!("Syntax Error: expect implicit parameter declaration",)
            }
            OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(_) => {
                format!("Syntax Error: expected `)` in parameter list",)
            }
            OriginalNodeDeclError::ExpectedRightParenthesisInTupleStructFieldTypeList(_) => {
                format!("Syntax Error: expected `)` in tuple struct field type list",)
            }
            OriginalNodeDeclError::ExpectedVariableType(_) => {
                format!("Syntax Error: ExpectVariableType",)
            }
            OriginalNodeDeclError::ExpectEqTokenForVariable(_) => {
                format!("Syntax Error: ExpectEqTokenForVariable",)
            }
            OriginalNodeDeclError::ExpectedLeftCurlyBraceOrLeftParenthesisOrSemicolonForStruct(
                _,
            ) => {
                format!("Syntax Error: expected `{{` `(` or `;` for struct",)
            }
            OriginalNodeDeclError::ExpectedEqForAssociatedType(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        match self {
            OriginalNodeDeclError::Expr(error) => error.range(ctx),
            OriginalNodeDeclError::ExpectedOutputType(token_stream_state)
            | OriginalNodeDeclError::ExpectedCurry(token_stream_state)
            | OriginalNodeDeclError::ExpectedEolColon(token_stream_state)
            | OriginalNodeDeclError::ExpectedRightCurlyBrace(token_stream_state)
            | OriginalNodeDeclError::ExpectedRightAngleBracketForImplicitParameterDeclList {
                token_stream_state,
                ..
            }
            | OriginalNodeDeclError::ExpectedParameterDeclList(token_stream_state)
            | OriginalNodeDeclError::ExpectedImplicitParameterDecl(token_stream_state)
            | OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(token_stream_state)
            | OriginalNodeDeclError::ExpectedRightParenthesisInTupleStructFieldTypeList(
                token_stream_state,
            )
            | OriginalNodeDeclError::ExpectedVariableType(token_stream_state)
            | OriginalNodeDeclError::ExpectEqTokenForVariable(token_stream_state)
            | OriginalNodeDeclError::ExpectedLeftCurlyBraceOrLeftParenthesisOrSemicolonForStruct(
                token_stream_state,
            ) => ctx.token_stream_state_text_range(*token_stream_state),
            OriginalNodeDeclError::ExpectedEqForAssociatedType(_) => todo!(),
        }
    }
}
