use std::sync::Arc;

use husky_print_utils::ep;
use husky_word::new_word_interner;

use crate::TokenizedText;

fn standalone_tokenize(text: &'static str) -> Arc<TokenizedText> {
    let word_interner = new_word_interner();
    TokenizedText::parse(&word_interner, text)
}

#[test]
fn test_play() {
    let tokenized_text = standalone_tokenize("struct A {}");
    ep!(tokenized_text);
}
