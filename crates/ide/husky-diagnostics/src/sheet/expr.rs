use husky_expr::{Expr, ExprError, ExprSheet, Stmt, StmtError};
use husky_token::RangedTokenSheet;

use super::*;

#[salsa::tracked(jar = DiagnosticsJar)]
pub struct ExprDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}
// ad hoc
impl<Db: DiagnosticsDb> salsa::DebugWithDb<Db> for ExprDiagnosticSheet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn DiagnosticsDb, include_all_fields)
    }
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
        for defn in defn_sheet.defns() {
            let decl = defn.decl(db);
            collect_expr_diagnostics(
                db,
                ranged_token_sheet,
                decl.expr_sheet(db),
                &mut diagnostics,
            );
            if let Some(expr_sheet) = defn.expr_sheet(db) {
                collect_expr_diagnostics(db, ranged_token_sheet, expr_sheet, &mut diagnostics);
            }
        }
    }
    // todo
    ExprDiagnosticSheet::new(db, diagnostics)
}

fn collect_expr_diagnostics(
    db: &dyn DiagnosticsDb,
    ranged_token_sheet: &RangedTokenSheet,
    expr_sheet: ExprSheet,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for expr in expr_sheet.expr_arena(db).data() {
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
    for stmt in expr_sheet.stmt_arena(db).data() {
        match stmt {
            Stmt::Err(error) => {
                if let Some(message) = stmt_error_message(error) {
                    diagnostics.push(Diagnostic {
                        message,
                        severity: stmt_error_severity(error),
                        range: stmt_error_range(error, ranged_token_sheet),
                    })
                }
            }
            _ => (),
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
        ExprError::EntityTree(_) => format!("Syntax Error: entity tree"),
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
        ExprError::ExpectIdentifierAfterScopeResolution(_) => {
            format!("Syntax Error: expect identifier after `::`")
        }
        ExprError::MissingBlock(_) => format!("Syntax Error: missing block"),
        ExprError::UnexpectedSheba(_) => format!("Syntax Error: unexpected `$`"),
        ExprError::UnrecognizedIdentifier { token_idx, ident } => {
            format!("Syntax Error: unrecognized identifier")
        }
        ExprError::UnresolvedSubentity { token_idx, ident } => {
            format!("Syntax Error: unresolved subentity")
        }
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
        | ExprError::ExpectIdentifierAfterScopeResolution(token_idx)
        | ExprError::UnexpectedSheba(token_idx)
        | ExprError::UnrecognizedIdentifier { token_idx, .. }
        | ExprError::UnresolvedSubentity { token_idx, .. } => {
            ranged_token_sheet.token_range(*token_idx)
        }
        ExprError::EntityTree(_) => todo!(),
        ExprError::Token(_) => todo!(),
        ExprError::MissingBlock(_) => todo!(),
    }
}

fn stmt_error_message(error: &StmtError) -> Option<String> {
    todo!()
}

fn stmt_error_severity(error: &StmtError) -> DiagnosticSeverity {
    todo!()
}

fn stmt_error_range(error: &StmtError, ranged_token_sheet: &RangedTokenSheet) -> TextRange {
    todo!()
}
