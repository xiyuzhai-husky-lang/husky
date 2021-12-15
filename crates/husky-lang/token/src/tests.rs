use test_data::*;
use word::new_word_interner;

use common::*;

use crate::TokenizedText;

#[test]
fn test_play() {
    let word_interner = new_word_interner();
    let tokenized_text = TokenizedText::parse(&word_interner, "struct A {}");
    p!(tokenized_text);
}
