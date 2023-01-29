use husky_expr::{
    EntityPathExpr, EntityPathExprError, Expr, ExprError, ExprRegion, OriginalEntityPathExprError,
    Stmt, StmtError,
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
            collect_expr_diagnostics(
                db,
                ranged_token_sheet,
                token_sheet_data,
                decl.expr_region(db),
                &mut diagnostics,
            );
            if let Some(expr_region) = defn.expr_region(db) {
                collect_expr_diagnostics(
                    db,
                    ranged_token_sheet,
                    token_sheet_data,
                    expr_region,
                    &mut diagnostics,
                );
            }
        }
    }
    // todo
    ExprDiagnosticSheet::new(db, diagnostics)
}

fn collect_expr_diagnostics(
    db: &dyn DiagnosticsDb,
    ranged_token_sheet: &RangedTokenSheet,
    token_sheet_data: &TokenSheetData,
    expr_region: ExprRegion,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let expr_region_data = expr_region.data(db);
    for expr in expr_region_data.expr_arena().data() {
        match expr {
            Expr::Err(error) => {
                if let Some(message) = expr_error_message(error) {
                    diagnostics.push(Diagnostic {
                        message,
                        severity: expr_error_severity(error),
                        range: expr_error_range(error, ranged_token_sheet),
                    })
                }
            }
            _ => (),
        }
    }
    for stmt in expr_region_data.stmt_arena().data() {
        match stmt {
            Stmt::Err(e) => {
                diagnostics.push(e.to_diagnostic(db, ranged_token_sheet, token_sheet_data))
            }
            _ => (),
        }
    }
    for entity_path_expr in expr_region_data.entity_path_expr_arena().data() {
        match entity_path_expr {
            EntityPathExpr::Root { .. } => (),
            EntityPathExpr::Subentity {
                ident_token,
                path: entity_path,
                ..
            } => {
                match ident_token {
                    Ok(_) => (),
                    Err(e) => todo!(),
                }
                match entity_path {
                    Err(EntityPathExprError::Original(e)) => {
                        diagnostics.push(e.to_diagnostic(db, ranged_token_sheet, token_sheet_data))
                    }
                    _ => (),
                }
            }
        }
    }
}

fn expr_error_message(error: &ExprError) -> Option<String> {
    Some(match error {
        ExprError::MismatchingBracket { .. } => format!("Syntax Error: mismatching bracket"),
        ExprError::MissingRightAngleBracket { .. } => format!("Syntax Error: missing `>`"),
        ExprError::ExpectRightCurlyBrace(_) => format!("Syntax Error: expect `}}`"),
        ExprError::ExpectIdentifier(_) => format!("Syntax Error: expect identifier"),
        ExprError::ExpectColon(_) => format!("Syntax Error: expect `:`"),
        ExprError::ExpectRightParenthesis(_) => format!("Syntax Error: expect `)`"),
        ExprError::NoMatchingBra { .. } => format!("Syntax Error: no matching bracket"),
        ExprError::ExpectIdentifierAfterDot { .. } => {
            format!("Syntax Error: expect identifier after dot")
        }
        ExprError::Token(_) => return None,
        ExprError::NoLeftOperandForBinaryOperator { .. } => {
            format!("Syntax Error: no left operand for binary operator")
        }
        ExprError::NoRightOperandForBinaryOperator { .. } => {
            format!("Syntax Error: no right operand for binary operator")
        }
        ExprError::NoOperandForPrefixOperator { .. } => {
            format!("Syntax Error:no operand for prefix operator")
        }
        ExprError::MissingItemBeforeComma { .. } => {
            format!("Syntax Error: missing item before `,`")
        }
        ExprError::MissingItemBeforeBe { .. } => format!("Syntax Error: missing item before `be`"),
        ExprError::ExpectLetVariablePattern(_) => format!("Syntax Error: expect variable pattern"),
        ExprError::ExpectAssignToken(_) => format!("Syntax Error: expect `=`"),
        ExprError::MissingInitialValue(_) => format!("Syntax Error: missing initial value"),
        ExprError::UnexpectedKeyword(_) => format!("Syntax Error: unexpected keyword"),
        ExprError::MissingResult(_) => format!("Syntax Error: missing result"),
        ExprError::MissingCondition(_) => format!("Syntax Error: missing condition"),
        ExprError::MissingForExpr(_) => format!("Syntax Error: expect for expr"),
        ExprError::ExpectBePattern(_) => format!("Syntax Error: expect be pattern"),
        ExprError::ExpectParameterPattern(_) => format!("Syntax Error: expect paramter pattern"),
        ExprError::UnterminatedList { .. } => format!("Syntax Error: unterminated list"),
        ExprError::ExpectEolColon(_) => format!("Syntax Error: expect `:` at end of line"),
        ExprError::ExpectIdentifierAfterMut(_) => {
            format!("Syntax Error: expect identifier after `mut`")
        }
        ExprError::MissingBlock(_) => format!("Syntax Error: missing block"),
        ExprError::UnexpectedSheba(_) => format!("Syntax Error: unexpected `$`"),
        ExprError::UnrecognizedIdentifier { token_idx, ident } => {
            format!("Syntax Error: unrecognized identifier")
        }
        ExprError::UnresolvedSubentity { token_idx, ident } => {
            format!("Syntax Error: unresolved subentity")
        }
        ExprError::MissingLetVariablesType(_) => todo!(),
        ExprError::MissingFieldType(_) => todo!(),
    })
}

fn expr_error_severity(error: &ExprError) -> DiagnosticSeverity {
    DiagnosticSeverity::Error
}

fn expr_error_range(error: &ExprError, ranged_token_sheet: &RangedTokenSheet) -> TextRange {
    match error {
        ExprError::MismatchingBracket {
            ket_token_idx: token_idx,
            ..
        }
        | ExprError::MissingRightAngleBracket {
            langle_token_idx: token_idx,
        }
        | ExprError::ExpectRightCurlyBrace(token_idx)
        | ExprError::ExpectIdentifier(token_idx)
        | ExprError::ExpectColon(token_idx)
        | ExprError::ExpectRightParenthesis(token_idx)
        | ExprError::NoMatchingBra {
            ket_token_idx: token_idx,
            ..
        }
        | ExprError::ExpectIdentifierAfterDot(token_idx)
        | ExprError::NoLeftOperandForBinaryOperator {
            binary_token_idx: token_idx,
        }
        | ExprError::NoRightOperandForBinaryOperator {
            punctuation_token_idx: token_idx,
            ..
        }
        | ExprError::NoOperandForPrefixOperator {
            prefix_token_idx: token_idx,
            ..
        }
        | ExprError::MissingItemBeforeComma {
            comma_token_idx: token_idx,
        }
        | ExprError::MissingItemBeforeBe {
            be_token_idx: token_idx,
        }
        | ExprError::ExpectLetVariablePattern(token_idx)
        | ExprError::ExpectAssignToken(token_idx)
        | ExprError::MissingInitialValue(token_idx)
        | ExprError::UnexpectedKeyword(token_idx)
        | ExprError::MissingResult(token_idx)
        | ExprError::MissingCondition(token_idx)
        | ExprError::MissingForExpr(token_idx)
        | ExprError::ExpectBePattern(token_idx)
        | ExprError::ExpectParameterPattern(token_idx)
        | ExprError::UnterminatedList {
            bra_token_idx: token_idx,
        }
        | ExprError::ExpectEolColon(token_idx)
        | ExprError::ExpectIdentifierAfterMut(token_idx)
        | ExprError::UnexpectedSheba(token_idx)
        | ExprError::UnrecognizedIdentifier { token_idx, .. }
        | ExprError::UnresolvedSubentity { token_idx, .. } => {
            ranged_token_sheet.token_text_range(*token_idx)
        }
        ExprError::Token(_) => todo!(),
        ExprError::MissingBlock(_) => todo!(),
        ExprError::MissingLetVariablesType(_) => todo!(),
        ExprError::MissingFieldType(_) => todo!(),
    }
}

impl Diagnose for OriginalEntityPathExprError {
    fn message(&self, db: &dyn DiagnosticsDb) -> String {
        match self {
            OriginalEntityPathExprError::EntityTree { token_idx, error } => {
                format!("entity tree error {:?}", error.debug(db))
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

    fn range(
        &self,
        ranged_token_sheet: &RangedTokenSheet,
        token_sheet_data: &TokenSheetData,
    ) -> TextRange {
        match self {
            OriginalEntityPathExprError::EntityTree { token_idx, error } => {
                ranged_token_sheet.token_text_range(*token_idx)
            }
            OriginalEntityPathExprError::ExpectIdentifierAfterScopeResolution(_) => todo!(),
        }
    }
}

impl Diagnose for StmtError {
    fn message(&self, db: &dyn DiagnosticsDb) -> String {
        todo!()
    }

    fn severity(&self) -> DiagnosticSeverity {
        todo!()
    }

    fn range(
        &self,
        ranged_token_sheet: &RangedTokenSheet,
        token_sheet_data: &TokenSheetData,
    ) -> TextRange {
        todo!()
    }
}
