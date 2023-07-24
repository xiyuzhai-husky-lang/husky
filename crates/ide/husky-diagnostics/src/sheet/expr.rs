use super::*;
use husky_syn_defn::HasDefns;
use husky_syn_expr::{
    ExprError, OriginalExprError, OriginalPrincipalEntityPathExprError, PrincipalEntityPathExpr,
    PrincipalEntityPathExprError, StmtError, SynExpr, SynExprRegion, SynStmt,
};
use salsa::DebugWithDb;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct ExprDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn expr_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> ExprDiagnosticSheet {
    let mut sheet_collector = ModuleDiagnosticsCollector::new(db, module_path);
    if let (Ok(ranged_token_sheet), Ok(defns)) =
        (db.ranged_token_sheet(module_path), module_path.defns(db))
    {
        let _token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for defn in defns.iter().copied() {
            let decl = defn.decl(db);
            if let Some(syn_expr_region) = decl.syn_expr_region(db) {
                sheet_collector.collect_expr_diagnostics(syn_expr_region);
            }
            if let Some(syn_expr_region) = defn.syn_expr_region(db) {
                sheet_collector.collect_expr_diagnostics(syn_expr_region);
            }
        }
    }
    ExprDiagnosticSheet::new(db, sheet_collector.finish())
}

impl<'a> ModuleDiagnosticsCollector<'a> {
    fn collect_expr_diagnostics(&mut self, syn_expr_region: SynExprRegion) {
        let expr_region_data = syn_expr_region.data(self.db());
        for expr in expr_region_data.expr_arena().data() {
            match expr {
                SynExpr::Err(ExprError::Original(e)) => self.visit_atom(e),
                _ => (),
            }
        }
        for stmt in expr_region_data.stmt_arena().data() {
            match stmt {
                SynStmt::Err(e) => self.visit_atom(e),
                _ => (),
            }
        }
        for item_path_expr in expr_region_data.principal_item_path_expr_arena().data() {
            match item_path_expr {
                PrincipalEntityPathExpr::Root { .. } => (),
                PrincipalEntityPathExpr::Subitem {
                    ident_token, path, ..
                } => {
                    match ident_token {
                        Err(PrincipalEntityPathExprError::Original(e)) => self.visit_atom(e),
                        _ => (),
                    }
                    match path {
                        Err(PrincipalEntityPathExprError::Original(e)) => self.visit_atom(e),
                        _ => (),
                    }
                }
            }
        }
    }
}

impl Diagnose for OriginalExprError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, _db: &Self::Context<'_>) -> String {
        match self {
            OriginalExprError::MismatchingBracket { .. } => {
                format!("Syntax Error: mismatching bracket")
            }
            OriginalExprError::ExpectedRightAngleBracket { .. } => {
                format!("Syntax Error: expect `>`")
            }
            OriginalExprError::ExpectedRightCurlyBrace(_) => format!("Syntax Error: expect `}}`"),
            OriginalExprError::ExpectedIdent(_) => format!("Syntax Error: expect identifier"),
            OriginalExprError::ExpectedColon(_) => format!("Syntax Error: expect `:`"),
            OriginalExprError::ExpectedRightParenthesis(_) => format!("Syntax Error: expect `)`"),
            OriginalExprError::NoMatchingBra { .. } => format!("Syntax Error: no matching bracket"),
            OriginalExprError::ExpectedIdentAfterDot { .. } => {
                format!("Syntax Error: expect identifier after dot")
            }
            OriginalExprError::NoLeftOperandForBinaryOperator { .. } => {
                format!("Syntax Error: no left operand for binary operator")
            }
            OriginalExprError::NoRightOperandForBinaryOperator { .. } => {
                format!("Syntax Error: no right operand for binary operator")
            }
            OriginalExprError::NoOperandForPrefixOperator { .. } => {
                format!("Syntax Error:no operand for prefix operator")
            }
            OriginalExprError::ExpectedItemBeforeComma { .. } => {
                format!("Syntax Error: expect item before `,`")
            }
            OriginalExprError::ExpectedItemBeforeBe { .. } => {
                format!("Syntax Error: expect item before `be`")
            }
            OriginalExprError::ExpectedLetVariableDecls(_) => {
                format!("Syntax Error: expect variable pattern")
            }
            OriginalExprError::ExpectedBeVariablesPattern(_) => {
                format!("Syntax Error: expected pattern expression after `be`")
            }
            OriginalExprError::ExpectedAssign(_) => format!("Syntax Error: expect `=`"),
            OriginalExprError::ExpectedInitialValue(_) => {
                format!("Syntax Error: expect initial value")
            }
            OriginalExprError::UnexpectedKeyword(_) => format!("Syntax Error: unexpected keyword"),
            OriginalExprError::ExpectedResult(_) => format!("Syntax Error: expect result"),
            OriginalExprError::ExpectedCondition(_) => format!("Syntax Error: expect condition"),
            OriginalExprError::ExpectedForExpr(_) => format!("Syntax Error: expect for expr"),
            OriginalExprError::ExpectedBePattern(_) => format!("Syntax Error: expect be pattern"),
            OriginalExprError::ExpectedParameterPattern(_) => {
                format!("Syntax Error: expect paramter pattern")
            }
            OriginalExprError::ExpectedEolColon(_) => {
                format!("Syntax Error: expect `:` at end of line")
            }
            OriginalExprError::ExpectedIdentAfterModifier(_) => {
                format!("Syntax Error: expect identifier after `mut`")
            }
            OriginalExprError::ExpectedConstantImplicitParameterType(_) => {
                format!("Syntax Error: expected constant implicit parameter type")
            }
            OriginalExprError::ExpectedBlock(_) => format!("Syntax Error: expect block"),
            OriginalExprError::UnterminatedList { .. } => {
                format!("Syntax Error: unterminated list")
            }
            OriginalExprError::UnterminatedFunctionCallKeyedArgumentList { bra_token_idx } => {
                format!("Syntax Error: unterminated function call keyed argument list")
            }
            OriginalExprError::UnterminatedMethodCallKeyedArgumentList { bra_token_idx } => {
                format!("Syntax Error: unterminated method call keyed argument list")
            }
            OriginalExprError::UnexpectedSheba(_) => format!("Syntax Error: unexpected `$`"),
            OriginalExprError::UnrecognizedIdent {
                token_idx: _,
                ident: _,
            } => {
                format!("Syntax Error: unrecognized identifier")
            }
            OriginalExprError::UnresolvedSubitem {
                token_idx: _,
                ident: _,
            } => {
                format!("Syntax Error: unresolved subitem")
            }
            OriginalExprError::ExpectedLetVariablesType(_) => {
                format!("Syntax Error: expected let variables type")
            }
            OriginalExprError::ExpectedFieldType(_) => {
                format!("Syntax Error: expected field type")
            }
            OriginalExprError::ExpectedParameterType(_) => {
                format!("Syntax Error: expected parameter type")
            }
            OriginalExprError::SelfTypeNotAllowed(_) => {
                format!("Syntax Error: SelfTypeNotAllowed")
            }
            OriginalExprError::SelfValueNotAllowed(_) => {
                format!("Syntax Error: SelfValueNotAllowed")
            }
            OriginalExprError::ExpectedIdentAfterDot { .. } => {
                format!("Syntax Error: ExpectedIdentAfterDot")
            }
            OriginalExprError::ExpectedExprBeforeDot { .. } => {
                format!("Syntax Error: ExpectedExprBeforeDot")
            }
            OriginalExprError::HtmlTodo(_) => {
                format!("Syntax Error: HtmlTodo")
            }
            OriginalExprError::ExpectedValueForFieldBindInitialization(_) => {
                format!("Syntax Error: ExpectedValueForFieldBindInitialization")
            }
            OriginalExprError::ExpectedFunctionIdentAfterOpeningHtmlBra(_) => {
                format!("Syntax Error: ExpectedFunctionIdentAfterOpeningHtmlBra")
            }
            OriginalExprError::UnexpectedLeftCurlyBrace(_) => {
                format!("Syntax Error: UnexpectedLeftCurlyBrace")
            }
            OriginalExprError::ExpectedTraits(_) => todo!(),
            OriginalExprError::ExpectedTypeAfterLightArrow { light_arrow_token } => todo!(),
            OriginalExprError::ExpectedExplicitParameterDefaultValue(_) => todo!(),
            OriginalExprError::ExpectedTypeTermForAssociatedType(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        let token_idx_range = self.token_idx_range();
        ctx.token_idx_range_text_range(token_idx_range)
    }
}

impl Diagnose for OriginalPrincipalEntityPathExprError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, ctx: &SheetDiagnosticsContext) -> String {
        match self {
            OriginalPrincipalEntityPathExprError::EntityTree {
                token_idx: _,
                error,
            } => {
                format!("item tree error {:?}", error.debug(ctx.db()))
            }
            OriginalPrincipalEntityPathExprError::ExpectIdentAfterScopeResolution(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        match self {
            OriginalPrincipalEntityPathExprError::EntityTree {
                token_idx: _,
                error: _,
            } => DiagnosticSeverity::Error,
            OriginalPrincipalEntityPathExprError::ExpectIdentAfterScopeResolution(_) => {
                DiagnosticSeverity::Error
            }
        }
    }

    fn range(&self, ctx: &SheetDiagnosticsContext) -> TextRange {
        match self {
            OriginalPrincipalEntityPathExprError::EntityTree {
                token_idx,
                error: _,
            } => ctx.token_idx_text_range(*token_idx),
            OriginalPrincipalEntityPathExprError::ExpectIdentAfterScopeResolution(_) => todo!(),
        }
    }
}

impl Diagnose for StmtError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, _ctx: &SheetDiagnosticsContext) -> String {
        todo!()
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(&self, _ctx: &SheetDiagnosticsContext) -> TextRange {
        todo!()
    }
}
