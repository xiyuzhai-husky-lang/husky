use super::*;
use husky_coword::Coword;
use husky_tex_command::path::TexCommandPath;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexRoseTokenData {
    Word(Coword),
    Command(TexCommandPath),
    Dollar,
    Nat32(u32),
    NewParagraph,
}

impl<'a> TexLexer<'a> {
    pub(super) fn next_text_token_data(&mut self) -> Option<TexRoseTokenData> {
        match self.chars.peek()? {
            '\\' => {
                self.chars.eat_char();
                match self.chars.peek() {
                    Some(c) => match c {
                        c if c.is_alphanumeric() => Some(
                            TexCommandPath::Coword(
                                self.next_coword_with(|c| c.is_alphanumeric()).unwrap(),
                            )
                            .into(),
                        ),
                        _ => todo!(),
                    },
                    None => todo!(),
                }
            }
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
                    self.chars.next_str_slice_while(|c| c.is_alphabetic()),
                )
                .into(),
            ),
            c => {
                use husky_print_utils::p;

                p!(c);
                todo!()
            }
        }
    }
}

#[test]
fn next_text_token_data_works() {
    fn t(input: &str, expected: &Expect) {
        let db = &DB::default();
        let lexer = TexLexer::new(db, input, TexMode::Rose);
        let tokens: Vec<_> = lexer.map(|(_, token_data)| token_data).collect();
        expected.assert_debug_eq(&(tokens.debug(db)));
    }
    t(
        "",
        &expect![[r#"
        []
    "#]],
    );
    t(
        " ",
        &expect![[r#"
        []
    "#]],
    );
    t(
        "  ",
        &expect![[r#"
        []
    "#]],
    );
    t(
        "\n",
        &expect![[r#"
        []
    "#]],
    );
    t(
        "\n\n",
        &expect![[r#"
            [
                TexTokenData::Rose(
                    TexRoseTokenData::NewParagraph,
                ),
            ]
        "#]],
    );
    t(
        "hello",
        &expect![[r#"
            [
                TexTokenData::Rose(
                    TexRoseTokenData::Word(
                        Coword(
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
                TexTokenData::Rose(
                    TexRoseTokenData::Nat32(
                        0,
                    ),
                ),
            ]
        "#]],
    );
    t(
        " 0",
        &expect![[r#"
        [
            TexTokenData::Rose(
                TexRoseTokenData::Nat32(
                    0,
                ),
            ),
        ]
    "#]],
    );
    t(
        "0 0",
        &expect![[r#"
        [
            TexTokenData::Rose(
                TexRoseTokenData::Nat32(
                    0,
                ),
            ),
            TexTokenData::Rose(
                TexRoseTokenData::Nat32(
                    0,
                ),
            ),
        ]
    "#]],
    );
    t(
        "0\n0",
        &expect![[r#"
        [
            TexTokenData::Rose(
                TexRoseTokenData::Nat32(
                    0,
                ),
            ),
            TexTokenData::Rose(
                TexRoseTokenData::Nat32(
                    0,
                ),
            ),
        ]
    "#]],
    );
    t(
        "0  0",
        &expect![[r#"
        [
            TexTokenData::Rose(
                TexRoseTokenData::Nat32(
                    0,
                ),
            ),
            TexTokenData::Rose(
                TexRoseTokenData::Nat32(
                    0,
                ),
            ),
        ]
    "#]],
    );
    t(
        "\\emph",
        &expect![[r#"
            [
                TexTokenData::Rose(
                    TexRoseTokenData::Command(
                        TexCommandPath::Coword(
                            Coword(
                                "emph",
                            ),
                        ),
                    ),
                ),
            ]
        "#]],
    );
    t(
        "\\emph",
        &expect![[r#"
            [
                TexTokenData::Rose(
                    TexRoseTokenData::Command(
                        TexCommandPath::Coword(
                            Coword(
                                "emph",
                            ),
                        ),
                    ),
                ),
            ]
        "#]],
    );
}
