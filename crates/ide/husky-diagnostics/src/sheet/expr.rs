use husky_expr::{
    EntityPathExpr, EntityPathExprError, Expr, ExprError, ExprRegion, OriginalEntityPathExprError,
    OriginalExprError, Stmt, StmtError,
};

use salsa::DebugWithDb;

use super::*;

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
    let mut sheet_collector = SheetDiagnosticsCollector::new(db, module_path);
    if let (Ok(ranged_token_sheet), Ok(defn_sheet)) = (
        db.ranged_token_sheet(module_path),
        db.collect_defns(module_path),
    ) {
        let _token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for (_, defn) in defn_sheet.defns() {
            if let Ok(defn) = defn {
                let decl = defn.decl(db);
                sheet_collector.collect_expr_diagnostics(decl.expr_region(db));
                if let Some(expr_region) = defn.expr_region(db) {
                    sheet_collector.collect_expr_diagnostics(expr_region);
                }
            }
        }
    }
    ExprDiagnosticSheet::new(db, sheet_collector.finish())
}

impl<'a> SheetDiagnosticsCollector<'a> {
    fn collect_expr_diagnostics(&mut self, expr_region: ExprRegion) {
        let expr_region_data = expr_region.data(self.db());
        for expr in expr_region_data.expr_arena().data() {
            match expr {
                Expr::Err(ExprError::Original(e)) => self.visit_atom(e),
                _ => (),
            }
        }
        for stmt in expr_region_data.stmt_arena().data() {
            match stmt {
                Stmt::Err(e) => self.visit_atom(e),
                _ => (),
            }
        }
        for entity_path_expr in expr_region_data.entity_path_expr_arena().data() {
            match entity_path_expr {
                EntityPathExpr::Root { .. } => (),
                EntityPathExpr::Subentity {
                    ident_token, path, ..
                } => {
                    match ident_token {
                        Err(EntityPathExprError::Original(e)) => self.visit_atom(e),
                        _ => (),
                    }
                    match path {
                        Err(EntityPathExprError::Original(e)) => self.visit_atom(e),
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
            OriginalExprError::ExpectedLetVariablesPattern(_) => {
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
            OriginalExprError::UnexpectedSheba(_) => format!("Syntax Error: unexpected `$`"),
            OriginalExprError::UnrecognizedIdent {
                token_idx: _,
                ident: _,
            } => {
                format!("Syntax Error: unrecognized identifier")
            }
            OriginalExprError::UnresolvedSubentity {
                token_idx: _,
                ident: _,
            } => {
                format!("Syntax Error: unresolved subentity")
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

impl Diagnose for OriginalEntityPathExprError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, ctx: &SheetDiagnosticsContext) -> String {
        match self {
            OriginalEntityPathExprError::EntityTree {
                token_idx: _,
                error,
            } => {
                format!("entity tree error {:?}", error.debug(ctx.db()))
            }
            OriginalEntityPathExprError::ExpectIdentAfterScopeResolution(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        match self {
            OriginalEntityPathExprError::EntityTree {
                token_idx: _,
                error: _,
            } => DiagnosticSeverity::Error,
            OriginalEntityPathExprError::ExpectIdentAfterScopeResolution(_) => {
                DiagnosticSeverity::Error
            }
        }
    }

    fn range(&self, ctx: &SheetDiagnosticsContext) -> TextRange {
        match self {
            OriginalEntityPathExprError::EntityTree {
                token_idx,
                error: _,
            } => ctx.token_idx_text_range(*token_idx),
            OriginalEntityPathExprError::ExpectIdentAfterScopeResolution(_) => todo!(),
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
