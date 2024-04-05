use super::*;
use husky_coword::Coword;
use salsa::DebugWithDb;

impl<'a> TexTokenizer<'a> {
    pub(super) fn next_text_token_data(&mut self) -> Option<TexTextTokenData> {
        use husky_print_utils::p;

        match self.chars.peek()? {
            n if n.is_numeric() => todo!(),
            a if a.is_alphabetic() => Some(
                Coword::from_ref(
                    self.db,
                    self.chars.get_str_slice_with(|c| c.is_alphabetic()),
                )
                .into(),
            ),
            c => {
                todo!()
            }
        }
    }
}

#[test]
fn next_text_token_data_works() {
    let db = &DB::default();
    let tokenizer = TexTokenizer::new(db, "hello", TexMode::Text);
    let tokens: Vec<_> = tokenizer.map(|(_, token_data)| token_data).collect();
    expect![[r#"
        [
            TexTokenData::Text(
                TexTextTokenData::Word(
                    Word(
                        "hello",
                    ),
                ),
            ),
        ]
    "#]]
    .assert_debug_eq(&(tokens.debug(db)));
}
