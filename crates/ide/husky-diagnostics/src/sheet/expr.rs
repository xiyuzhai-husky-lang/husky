use husky_expr::ExprError;

use super::*;

#[salsa::tracked(jar = DiagnosticsJar)]
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
    // todo
    ExprDiagnosticSheet::new(db, diagnostics)
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
        ExprError::ExpectIdentifierAfterDot => format!("Syntax Error: expect identifier after dot"),
        ExprError::Token(_) => return None,
        ExprError::NoLeftOperandForBinaryOperator => {
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
        ExprError::MissingInitialValue => format!("Syntax Error: missing initial value"),
        ExprError::UnexpectedKeyword(_) => format!("Syntax Error: unexpected keyword"),
        ExprError::MissingResult => format!("Syntax Error: missing result"),
        ExprError::MissingCondition => format!("Syntax Error: missing condition"),
        ExprError::MissingForExpr => format!("Syntax Error: expect for expr"),
        ExprError::ExpectBePattern(_) => format!("Syntax Error: expect be pattern"),
        ExprError::ExpectParameterPattern(_) => format!("Syntax Error: expect paramter pattern"),
        ExprError::UnterminatedList => format!("Syntax Error: unterminated list"),
        ExprError::ExpectEolColon(_) => format!("Syntax Error: expect `:` at end of line"),
        ExprError::ExpectIdentifierAfterMut(_) => {
            format!("Syntax Error: expect identifier after `mut`")
        }
        ExprError::ExpectIdentifierAfterScopeResolution(_) => {
            format!("Syntax Error: expect identifier after `::`")
        }
        ExprError::MissingBlock => format!("Syntax Error: missing block"),
    })
}
