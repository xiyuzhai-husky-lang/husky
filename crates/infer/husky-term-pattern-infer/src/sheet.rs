use husky_ast::AstText;
use husky_term_pattern::*;

pub struct TermPatternInferSheet {
    ast_text: AstText,
    term_pattern_interner: TermPatternInterner,
    expr_term_arena: Vec<TermPatternIdx>,
}
