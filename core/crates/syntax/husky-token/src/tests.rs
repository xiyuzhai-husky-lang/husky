use std::sync::Arc;

use print_utils::ep;
use word::new_word_interner;

use crate::TokenizedText;

fn standalone_tokenize(text: &'static str) -> Arc<TokenizedText> {
    let word_unique_allocator = new_word_interner();
    TokenizedText::parse(&word_unique_allocator, text)
}

#[test]
fn test_play() {
    let tokenized_text = standalone_tokenize("struct A {}");
    ep!(tokenized_text);
}
