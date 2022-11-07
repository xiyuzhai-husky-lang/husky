use crate::*;
use husky_ast::AstText;
use husky_expr_syntax::RawExprMap;
use husky_term_pattern::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermPatternInferSheet {
    ast_text: AstText,
    term_pattern_interner: TermPatternInterner,
    expr_opt_term_patt_results: RawExprMap<TermPatternInferResult<Option<TermPatternItd>>>,
    expr_ty_patt_results: RawExprMap<TermPatternInferResult<TermPatternItd>>,
}
