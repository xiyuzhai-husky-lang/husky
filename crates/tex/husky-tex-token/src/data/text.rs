use super::*;
use husky_coword::Coword;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexTextTokenData {
    Word(Coword),
    Dollar,
    Nat32(u32),
}

impl<'a> TexTokenizer<'a> {
    pub(super) fn next_text_token_data(&mut self) -> Option<TexTextTokenData> {
        match self.chars.peek()? {
            n if n.is_numeric() => {
                let numeric_str_slice = self.chars.next_numeric_str_slice();
                match numeric_str_slice.parse::<u32>() {
                    Ok(number) => Some(number.into()), // ad hoc
                    Err(_) => todo!(),
                }
            }
            a if a.is_alphabetic() => Some(
                Coword::from_ref(
                    self.db,
                    self.chars.next_str_slice_with(|c| c.is_alphabetic()),
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
    fn t(input: &str, expected: &Expect) {
        let db = &DB::default();
        let tokenizer = TexTokenizer::new(db, input, TexMode::Text);
        let tokens: Vec<_> = tokenizer.map(|(_, token_data)| token_data).collect();
        expected.assert_debug_eq(&(tokens.debug(db)));
    }
    t(
        "hello",
        &expect![[r#"
    [
        TexTokenData::Text(
            TexTextTokenData::Word(
                Word(
                    "hello",
                ),
            ),
        ),
    ]
"#]],
    );
    t(
        "0",
        &expect![[r#"
            [
                TexTokenData::Text(
                    TexTextTokenData::Nat32(
                        0,
                    ),
                ),
            ]
        "#]],
    )
}
