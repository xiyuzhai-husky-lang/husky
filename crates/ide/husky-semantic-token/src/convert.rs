use crate::*;

pub(crate) fn to_semantic_tokens<'a>(
    tokens: impl IntoIterator<Item = &'a RangedSemanticToken>,
) -> Vec<ext::SemanticToken> {
    let mut semantic_tokens = vec![];
    let mut last_line = 0;
    let mut last_start = 0;
    for token in tokens {
        let new_line = token.range.start.i();
        let new_start = token.range.start.j();
        let length = token.range.end.j() - new_start;
        let delta_line = new_line - last_line;
        let delta_start = if new_line > last_line {
            new_start
        } else {
            new_start - last_start
        };
        semantic_tokens.push(ext::SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type: token.semantic_token.token_type(),
            token_modifiers_bitset: token.semantic_token.token_modifiers_bitset(),
        });
        last_line = new_line;
        last_start = new_start
    }
    semantic_tokens
}
