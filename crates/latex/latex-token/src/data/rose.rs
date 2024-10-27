use super::*;
use husky_coword::Coword;
use latex_command::path::LxCommandPath;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxRoseTokenData {
    Word(Coword),
    Command(LxCommandPath),
    /// `$`
    Dollar,
    /// `\(`
    EscapedLpar,
    /// `\[`
    EscapedLbox,
    Nat32(u32),
    NewParagraph,
}

impl<'a> LxLexer<'a> {
    pub(super) fn next_text_token_data(&mut self) -> Option<LxRoseTokenData> {
        match self.chars.peek()? {
            '\\' => {
                self.chars.eat_char();
                match self.chars.peek() {
                    Some(c) => match c {
                        c if c.is_alphanumeric() => Some(
                            LxCommandPath::Coword(
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
        let lexer = LxLexer::new(db, input, LxMode::Rose);
        let tokens: Vec<_> = lexer.map(|(_, _, _, token_data)| token_data).collect();
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
                LxTokenData::Rose(
                    LxRoseTokenData::NewParagraph,
                ),
            ]
        "#]],
    );
    t(
        "hello",
        &expect![[r#"
            [
                LxTokenData::Rose(
                    LxRoseTokenData::Word(
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
                LxTokenData::Rose(
                    LxRoseTokenData::Nat32(
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
                LxTokenData::Rose(
                    LxRoseTokenData::Nat32(
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
                LxTokenData::Rose(
                    LxRoseTokenData::Nat32(
                        0,
                    ),
                ),
                LxTokenData::Rose(
                    LxRoseTokenData::Nat32(
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
                LxTokenData::Rose(
                    LxRoseTokenData::Nat32(
                        0,
                    ),
                ),
                LxTokenData::Rose(
                    LxRoseTokenData::Nat32(
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
                LxTokenData::Rose(
                    LxRoseTokenData::Nat32(
                        0,
                    ),
                ),
                LxTokenData::Rose(
                    LxRoseTokenData::Nat32(
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
                LxTokenData::Rose(
                    LxRoseTokenData::Command(
                        LxCommandPath::Coword(
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
                LxTokenData::Rose(
                    LxRoseTokenData::Command(
                        LxCommandPath::Coword(
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
