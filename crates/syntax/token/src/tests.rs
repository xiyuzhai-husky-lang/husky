use print_utils::ep;
use word::new_word_unique_allocator;

use crate::TokenizedText;

fn standalone_tokenize(text: &'static str) -> TokenizedText {
    let word_unique_allocator = new_word_unique_allocator();
    TokenizedText::parse(&word_unique_allocator, text)
}

#[test]
fn test_play() {
    let tokenized_text = standalone_tokenize("struct A {}");
    ep!(tokenized_text);
}
