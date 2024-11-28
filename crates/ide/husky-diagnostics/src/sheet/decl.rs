use super::*;
use husky_syn_decl::{
    error::{OriginalSynNodeDeclError, SynNodeDeclError},
    jar::SynDeclDb,
};

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct DeclDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn decl_diagnostic_sheet(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> DeclDiagnosticSheet {
    let mut collector = ModuleDiagnosticsCollector::new(db, module_path);
    for (_, syn_node_decl) in db
        .syn_node_decl_sheet(module_path)
        .decls(db)
        .iter()
        .copied()
    {
        if let Some(syn_expr_region) = syn_node_decl.syn_expr_region(db) {
            for error in syn_node_decl.errors(db) {
                if let SynNodeDeclError::Original(error) = error {
                    collector
                        .region_collector(syn_expr_region)
                        .visit_atom(error)
                }
            }
        }
    }
    DeclDiagnosticSheet::new(db, collector.finish())
}

impl Diagnose for OriginalSynNodeDeclError {
    type Context<'a> = RegionDiagnosticsContext<'a>;

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
            OriginalSynNodeDeclError::ExpectedRcurlForStruct(_) => {
                format!("Syntax Error: expect `}}`",)
            }
            OriginalSynNodeDeclError::ExpectedRcurlForTypePropsVariant(_) => {
                format!("Syntax Error: expect `}}`",)
            }
            OriginalSynNodeDeclError::ExpectedRightAngleDelimiterForImplicitParameterDeclList {
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
            OriginalSynNodeDeclError::ExpectedValReturnType(_) => {
                format!("Syntax Error: ExpectVariableType",)
            }
            OriginalSynNodeDeclError::ExpectedEqTokenForMemo(_) => {
                format!("Syntax Error: ExpectEqTokenForVariable",)
            }
            OriginalSynNodeDeclError::ExpectedLcurlOrLparOrSemicolonForStruct(_) => {
                format!("Syntax Error: expect `{{`, `(`, or `;`")
            }
            OriginalSynNodeDeclError::ExpectedEqForAssocType(_) => {
                format!("Syntax Error: expect `=` for associated type")
            }
            OriginalSynNodeDeclError::ExpectedLeftDelimiterInDerive(_) => {
                format!("Syntax Error: expect `(` in derive")
            }
            OriginalSynNodeDeclError::ExpectedRightDelimiterInDerive(_) => {
                format!("Syntax Error: expect `)` in derive")
            }
            OriginalSynNodeDeclError::ExpectedColonBeforeValReturnType(_) => {
                format!("Syntax Error: expect `:` before return type")
            }
            OriginalSynNodeDeclError::ExpectedEqTokenForBackpropArgument(_) => {
                format!("Syntax Error: expect `=` for backprop argument")
            }
            OriginalSynNodeDeclError::ExpectedExprForBackpropArgument(_) => {
                format!("Syntax Error: expect expression for backprop argument")
            }
            OriginalSynNodeDeclError::ExpectedColonForTraitMemoizedField(_) => {
                format!("Syntax Error: expect `:` for trait memoized field")
            }
            OriginalSynNodeDeclError::ExpectedColonBeforeStaticReturnType(_) => {
                format!("Syntax Error: expect `:` before static return type")
            }
            OriginalSynNodeDeclError::ExpectedStaticReturnType(_) => {
                format!("Syntax Error: expect static return type")
            }
            OriginalSynNodeDeclError::ExpectedCrateDeclNarrative(_) => {
                format!("Syntax Error: expect crate declaration narrative")
            }
            OriginalSynNodeDeclError::ExpectedEqTokenForLibCrateDefaultConstExcludes(_) => {
                format!("Syntax Error: expect `=` for lib crate default const excludes")
            }
            OriginalSynNodeDeclError::ExpectedEqTokenForTypeAlias(_) => {
                format!("Syntax Error: expect `=` for type alias")
            }
            OriginalSynNodeDeclError::ExpectedEqTokenForStaticMut(_) => {
                format!("Syntax Error: expect `=` for static mut")
            }
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextPositionRange {
        match self {
            OriginalSynNodeDeclError::Expr(error) => error.range(ctx),
            OriginalSynNodeDeclError::ExpectedOutputType(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedCurry(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedEolColon(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedRcurlForStruct(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedRcurlForTypePropsVariant(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedRightAngleDelimiterForImplicitParameterDeclList {
                regional_token_stream_state,
                ..
            }
            | OriginalSynNodeDeclError::ExpectedParameterDeclList(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedImplicitParameterDecl(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedRightParenthesisInParameterList(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedRightParenthesisInTupleStructFieldTypeList(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedValReturnType(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedEqTokenForMemo(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedLcurlOrLparOrSemicolonForStruct(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedEqForAssocType(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedLeftDelimiterInDerive(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedRightDelimiterInDerive(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedColonBeforeValReturnType(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedEqTokenForBackpropArgument(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedExprForBackpropArgument(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedColonForTraitMemoizedField(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedColonBeforeStaticReturnType(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedStaticReturnType(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedCrateDeclNarrative(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedEqTokenForLibCrateDefaultConstExcludes(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedEqTokenForTypeAlias(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedEqTokenForStaticMut(regional_token_stream_state) => {
                ctx.token_stream_state_text_range(*regional_token_stream_state)
            }
        }
    }
}
