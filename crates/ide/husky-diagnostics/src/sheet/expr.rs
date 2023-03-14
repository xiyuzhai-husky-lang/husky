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
    let mut diagnostics = vec![];
    if let (Ok(ranged_token_sheet), Ok(defn_sheet)) = (
        db.ranged_token_sheet(module_path),
        db.collect_defns(module_path),
    ) {
        let _token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for (_, defn) in defn_sheet.defns() {
            if let Ok(defn) = defn {
                let decl = defn.decl(db);
                collect_expr_diagnostics(db, decl.expr_region(db), &mut diagnostics);
                if let Some(expr_region) = defn.expr_region(db) {
                    collect_expr_diagnostics(db, expr_region, &mut diagnostics);
                }
            }
        }
    }
    ExprDiagnosticSheet::new(db, diagnostics)
}

fn collect_expr_diagnostics(
    db: &dyn DiagnosticsDb,
    expr_region: ExprRegion,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let expr_region_data = expr_region.data(db);
    let ctx: RegionDiagnosticsContext = RegionDiagnosticsContext::new(db, expr_region);
    for expr in expr_region_data.expr_arena().data() {
        match expr {
            Expr::Err(ExprError::Original(error)) => diagnostics.push(error.to_diagnostic(&ctx)),
            _ => (),
        }
    }
    for stmt in expr_region_data.stmt_arena().data() {
        match stmt {
            Stmt::Err(error) => diagnostics.push(error.to_diagnostic(&ctx)),
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
                    Err(EntityPathExprError::Original(error)) => {
                        diagnostics.push(error.to_diagnostic(&ctx))
                    }
                    _ => (),
                }
                match path {
                    Err(EntityPathExprError::Original(error)) => {
                        diagnostics.push(error.to_diagnostic(&ctx))
                    }
                    _ => (),
                }
            }
        }
    }
}

impl Diagnose for OriginalExprError {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, _db: &Self::Context<'_>) -> String {
        match self {
            OriginalExprError::MismatchingBracket { .. } => {
                format!("Syntax Error: mismatching bracket")
            }
            OriginalExprError::ExpectRightAngleBracket { .. } => {
                format!("Syntax Error: expect `>`")
            }
            OriginalExprError::ExpectRightCurlyBrace(_) => format!("Syntax Error: expect `}}`"),
            OriginalExprError::ExpectIdent(_) => format!("Syntax Error: expect identifier"),
            OriginalExprError::ExpectColon(_) => format!("Syntax Error: expect `:`"),
            OriginalExprError::ExpectRightParenthesis(_) => format!("Syntax Error: expect `)`"),
            OriginalExprError::NoMatchingBra { .. } => format!("Syntax Error: no matching bracket"),
            OriginalExprError::ExpectIdentAfterDot { .. } => {
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
            OriginalExprError::ExpectItemBeforeComma { .. } => {
                format!("Syntax Error: expect item before `,`")
            }
            OriginalExprError::ExpectItemBeforeBe { .. } => {
                format!("Syntax Error: expect item before `be`")
            }
            OriginalExprError::ExpectLetVariablePattern(_) => {
                format!("Syntax Error: expect variable pattern")
            }
            OriginalExprError::ExpectAssign(_) => format!("Syntax Error: expect `=`"),
            OriginalExprError::ExpectInitialValue(_) => {
                format!("Syntax Error: expect initial value")
            }
            OriginalExprError::UnexpectedKeyword(_) => format!("Syntax Error: unexpected keyword"),
            OriginalExprError::ExpectResult(_) => format!("Syntax Error: expect result"),
            OriginalExprError::ExpectCondition(_) => format!("Syntax Error: expect condition"),
            OriginalExprError::ExpectForExpr(_) => format!("Syntax Error: expect for expr"),
            OriginalExprError::ExpectBePattern(_) => format!("Syntax Error: expect be pattern"),
            OriginalExprError::ExpectParameterPattern(_) => {
                format!("Syntax Error: expect paramter pattern")
            }
            OriginalExprError::UnterminatedList { .. } => {
                format!("Syntax Error: unterminated list")
            }
            OriginalExprError::ExpectEolColon(_) => {
                format!("Syntax Error: expect `:` at end of line")
            }
            OriginalExprError::ExpectIdentAfterMut(_) => {
                format!("Syntax Error: expect identifier after `mut`")
            }
            OriginalExprError::ExpectBlock(_) => format!("Syntax Error: expect block"),
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
            OriginalExprError::ExpectedPatternExprAfterBe(_) => {
                format!("Syntax Error: expected pattern expression after `be`")
            }
            OriginalExprError::ExpectParameterType(_) => {
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
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        // todo: duplicated
        // see ExprRangeCalculator::calc_expr_range in crate `husky-expr`
        match self {
            OriginalExprError::MismatchingBracket {
                ket_token_idx: token_idx,
                ..
            }
            | OriginalExprError::ExpectRightAngleBracket {
                langle_token_idx: token_idx,
            }
            | OriginalExprError::ExpectRightCurlyBrace(token_idx)
            | OriginalExprError::ExpectIdent(token_idx)
            | OriginalExprError::ExpectColon(token_idx)
            | OriginalExprError::ExpectRightParenthesis(token_idx)
            | OriginalExprError::NoMatchingBra {
                ket_token_idx: token_idx,
                ..
            }
            | OriginalExprError::ExpectIdentAfterDot(token_idx)
            | OriginalExprError::NoLeftOperandForBinaryOperator {
                binary_token_idx: token_idx,
            }
            | OriginalExprError::NoRightOperandForBinaryOperator {
                punctuation_token_idx: token_idx,
                ..
            }
            | OriginalExprError::NoOperandForPrefixOperator {
                prefix_token_idx: token_idx,
                ..
            }
            | OriginalExprError::ExpectItemBeforeComma {
                comma_token_idx: token_idx,
            }
            | OriginalExprError::ExpectItemBeforeBe {
                be_token_idx: token_idx,
            }
            | OriginalExprError::ExpectLetVariablePattern(token_idx)
            | OriginalExprError::ExpectAssign(token_idx)
            | OriginalExprError::ExpectInitialValue(token_idx)
            | OriginalExprError::UnexpectedKeyword(token_idx)
            | OriginalExprError::ExpectResult(token_idx)
            | OriginalExprError::ExpectCondition(token_idx)
            | OriginalExprError::ExpectForExpr(token_idx)
            | OriginalExprError::ExpectBePattern(token_idx)
            | OriginalExprError::ExpectParameterPattern(token_idx)
            | OriginalExprError::UnterminatedList {
                bra_token_idx: token_idx,
            }
            | OriginalExprError::ExpectEolColon(token_idx)
            | OriginalExprError::ExpectIdentAfterMut(token_idx)
            | OriginalExprError::UnexpectedSheba(token_idx)
            | OriginalExprError::UnrecognizedIdent { token_idx, .. }
            | OriginalExprError::UnresolvedSubentity { token_idx, .. }
            | OriginalExprError::ExpectedLetVariablesType(token_idx)
            | OriginalExprError::ExpectedFieldType(token_idx)
            | OriginalExprError::ExpectedPatternExprAfterBe(token_idx)
            | OriginalExprError::ExpectParameterType(token_idx)
            | OriginalExprError::SelfTypeNotAllowed(token_idx)
            | OriginalExprError::SelfValueNotAllowed(token_idx)
            | OriginalExprError::ExpectedIdentAfterDot {
                dot_token_idx: token_idx,
                ..
            }
            | OriginalExprError::ExpectedExprBeforeDot {
                dot_token_idx: token_idx,
            } => ctx.ranged_token_sheet().token_text_range(*token_idx),
            OriginalExprError::ExpectBlock(_) => todo!(),
        }
    }
}

impl Diagnose for OriginalEntityPathExprError {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, ctx: &RegionDiagnosticsContext) -> String {
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

    fn range(&self, ctx: &RegionDiagnosticsContext) -> TextRange {
        match self {
            OriginalEntityPathExprError::EntityTree {
                token_idx,
                error: _,
            } => ctx.ranged_token_sheet().token_text_range(*token_idx),
            OriginalEntityPathExprError::ExpectIdentAfterScopeResolution(_) => todo!(),
        }
    }
}

impl Diagnose for StmtError {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, _ctx: &RegionDiagnosticsContext) -> String {
        todo!()
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(&self, _ctx: &RegionDiagnosticsContext) -> TextRange {
        todo!()
    }
}
