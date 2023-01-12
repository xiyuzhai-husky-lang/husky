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
) -> AstDiagnosticSheet {
    todo!()
}

fn expr_error_message(error: &ExprError) {
    match error {
        ExprError::MisMatchingBracket {
            bra,
            bra_token_idx,
            ket,
            ket_token_idx,
        } => todo!(),
        ExprError::MissingRightAngleBracket { langle_token_idx } => todo!(),
        ExprError::ExpectRightCurlyBrace(_) => todo!(),
        ExprError::ExpectIdentifier(_) => todo!(),
        ExprError::ExpectColon(_) => todo!(),
        ExprError::ExpectRightParenthesis(_) => todo!(),
        ExprError::NoMatchingBra { ket, ket_token_idx } => todo!(),
        ExprError::EntityTree(_) => todo!(),
        ExprError::ExpectIdentifierAfterDot => todo!(),
        ExprError::Token(_) => todo!(),
        ExprError::NoLeftOperandForBinaryOperator => todo!(),
        ExprError::NoRightOperandForBinaryOperator {
            lopd,
            punctuation,
            punctuation_token_idx,
        } => todo!(),
        ExprError::NoOperandForPrefixOperator {
            prefix,
            prefix_token_idx,
        } => todo!(),
        ExprError::MissingItemBeforeComma { comma_token_idx } => todo!(),
        ExprError::MissingItemBeforeBe { be_token_idx } => todo!(),
        ExprError::ExpectLetVariablePattern(_) => todo!(),
        ExprError::ExpectAssignToken(_) => todo!(),
        ExprError::MissingInitialValue => todo!(),
        ExprError::UnexpectedKeyword(_) => todo!(),
        ExprError::MissingResult => todo!(),
        ExprError::MissingCondition => todo!(),
        ExprError::MissingForExpr => todo!(),
        ExprError::ExpectBePattern(_) => todo!(),
        ExprError::ExpectParameterPattern(_) => todo!(),
        ExprError::UnterminatedList => todo!(),
        ExprError::ExpectEolColon(_) => todo!(),
        ExprError::ExpectIdentifierAfterMut(_) => todo!(),
        ExprError::ExpectIdentifierAfterScopeResolution(_) => todo!(),
        ExprError::MissingBlock => todo!(),
    }
}
