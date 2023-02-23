use husky_expr::{
    EntityPathExpr, EntityPathExprError, Expr, ExprError, ExprRegion, OriginalEntityPathExprError,
    OriginalExprError, Stmt, StmtError,
};
use husky_token::RangedTokenSheet;
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
        db.defn_sheet(module_path),
    ) {
        let token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for defn in defn_sheet.defns() {
            let decl = defn.decl(db);
            collect_expr_diagnostics(db, decl.expr_region(db), &mut diagnostics);
            if let Some(expr_region) = defn.expr_region(db) {
                collect_expr_diagnostics(db, expr_region, &mut diagnostics);
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
    let ctx: DiagnosticsRegionContext = DiagnosticsRegionContext::new(db, expr_region);
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
    type Context<'a> = DiagnosticsRegionContext<'a>;

    fn message(&self, db: &Self::Context<'_>) -> String {
        match self {
            OriginalExprError::MismatchingBracket { .. } => {
                format!("Syntax Error: mismatching bracket")
            }
            OriginalExprError::MissingRightAngleBracket { .. } => {
                format!("Syntax Error: missing `>`")
            }
            OriginalExprError::ExpectRightCurlyBrace(_) => format!("Syntax Error: expect `}}`"),
            OriginalExprError::ExpectIdentifier(_) => format!("Syntax Error: expect identifier"),
            OriginalExprError::ExpectColon(_) => format!("Syntax Error: expect `:`"),
            OriginalExprError::ExpectRightParenthesis(_) => format!("Syntax Error: expect `)`"),
            OriginalExprError::NoMatchingBra { .. } => format!("Syntax Error: no matching bracket"),
            OriginalExprError::ExpectIdentifierAfterDot { .. } => {
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
            OriginalExprError::MissingItemBeforeComma { .. } => {
                format!("Syntax Error: missing item before `,`")
            }
            OriginalExprError::MissingItemBeforeBe { .. } => {
                format!("Syntax Error: missing item before `be`")
            }
            OriginalExprError::ExpectLetVariablePattern(_) => {
                format!("Syntax Error: expect variable pattern")
            }
            OriginalExprError::ExpectAssignToken(_) => format!("Syntax Error: expect `=`"),
            OriginalExprError::MissingInitialValue(_) => {
                format!("Syntax Error: missing initial value")
            }
            OriginalExprError::UnexpectedKeyword(_) => format!("Syntax Error: unexpected keyword"),
            OriginalExprError::MissingResult(_) => format!("Syntax Error: missing result"),
            OriginalExprError::MissingCondition(_) => format!("Syntax Error: missing condition"),
            OriginalExprError::MissingForExpr(_) => format!("Syntax Error: expect for expr"),
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
            OriginalExprError::ExpectIdentifierAfterMut(_) => {
                format!("Syntax Error: expect identifier after `mut`")
            }
            OriginalExprError::MissingBlock(_) => format!("Syntax Error: missing block"),
            OriginalExprError::UnexpectedSheba(_) => format!("Syntax Error: unexpected `$`"),
            OriginalExprError::UnrecognizedIdentifier { token_idx, ident } => {
                format!("Syntax Error: unrecognized identifier")
            }
            OriginalExprError::UnresolvedSubentity { token_idx, ident } => {
                format!("Syntax Error: unresolved subentity")
            }
            OriginalExprError::MissingLetVariablesType(_) => todo!(),
            OriginalExprError::MissingFieldType(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        match self {
            OriginalExprError::MismatchingBracket {
                ket_token_idx: token_idx,
                ..
            }
            | OriginalExprError::MissingRightAngleBracket {
                langle_token_idx: token_idx,
            }
            | OriginalExprError::ExpectRightCurlyBrace(token_idx)
            | OriginalExprError::ExpectIdentifier(token_idx)
            | OriginalExprError::ExpectColon(token_idx)
            | OriginalExprError::ExpectRightParenthesis(token_idx)
            | OriginalExprError::NoMatchingBra {
                ket_token_idx: token_idx,
                ..
            }
            | OriginalExprError::ExpectIdentifierAfterDot(token_idx)
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
            | OriginalExprError::MissingItemBeforeComma {
                comma_token_idx: token_idx,
            }
            | OriginalExprError::MissingItemBeforeBe {
                be_token_idx: token_idx,
            }
            | OriginalExprError::ExpectLetVariablePattern(token_idx)
            | OriginalExprError::ExpectAssignToken(token_idx)
            | OriginalExprError::MissingInitialValue(token_idx)
            | OriginalExprError::UnexpectedKeyword(token_idx)
            | OriginalExprError::MissingResult(token_idx)
            | OriginalExprError::MissingCondition(token_idx)
            | OriginalExprError::MissingForExpr(token_idx)
            | OriginalExprError::ExpectBePattern(token_idx)
            | OriginalExprError::ExpectParameterPattern(token_idx)
            | OriginalExprError::UnterminatedList {
                bra_token_idx: token_idx,
            }
            | OriginalExprError::ExpectEolColon(token_idx)
            | OriginalExprError::ExpectIdentifierAfterMut(token_idx)
            | OriginalExprError::UnexpectedSheba(token_idx)
            | OriginalExprError::UnrecognizedIdentifier { token_idx, .. }
            | OriginalExprError::UnresolvedSubentity { token_idx, .. } => {
                ctx.ranged_token_sheet().token_text_range(*token_idx)
            }
            OriginalExprError::MissingBlock(_) => todo!(),
            OriginalExprError::MissingLetVariablesType(_) => todo!(),
            OriginalExprError::MissingFieldType(_) => todo!(),
        }
    }
}

impl Diagnose for OriginalEntityPathExprError {
    type Context<'a> = DiagnosticsRegionContext<'a>;

    fn message(&self, ctx: &DiagnosticsRegionContext) -> String {
        match self {
            OriginalEntityPathExprError::EntityTree { token_idx, error } => {
                format!("entity tree error {:?}", error.debug(ctx.db()))
            }
            OriginalEntityPathExprError::ExpectIdentifierAfterScopeResolution(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        match self {
            OriginalEntityPathExprError::EntityTree { token_idx, error } => {
                DiagnosticSeverity::Error
            }
            OriginalEntityPathExprError::ExpectIdentifierAfterScopeResolution(_) => {
                DiagnosticSeverity::Error
            }
        }
    }

    fn range(&self, ctx: &DiagnosticsRegionContext) -> TextRange {
        match self {
            OriginalEntityPathExprError::EntityTree { token_idx, error } => {
                ctx.ranged_token_sheet().token_text_range(*token_idx)
            }
            OriginalEntityPathExprError::ExpectIdentifierAfterScopeResolution(_) => todo!(),
        }
    }
}

impl Diagnose for StmtError {
    type Context<'a> = DiagnosticsRegionContext<'a>;

    fn message(&self, ctx: &DiagnosticsRegionContext) -> String {
        todo!()
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(&self, ctx: &DiagnosticsRegionContext) -> TextRange {
        todo!()
    }
}
