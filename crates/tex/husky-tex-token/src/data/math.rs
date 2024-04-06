use super::*;
use husky_tex_command::path::TexCommandPath;
use husky_tex_math_letter::TexMathLetter;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexMathTokenData {
    Command(TexCommandPath),
    LeftDelimiter(TexMathDelimiter),
    RightDelimiter(TexMathDelimiter),
    Letter(TexMathLetter),
    Nat32(u32),
    Other(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexMathDelimiter {
    /// `{`,  `}`
    Curl,
    /// `(`, `)`
    Par,
    /// `[`, `]`
    Box,
    /// `\{`, `\}`
    Set,
}

pub enum Script {}

impl<'a> TexLexer<'a> {
    pub(super) fn next_math_token_data(&mut self) -> Option<TexMathTokenData> {
        match self.chars.peek()? {
            '\\' => {
                self.chars.eat_char();
                match self.chars.peek() {
                    Some(c) => match c {
                        c if c.is_alphanumeric() => {
                            Some(TexMathTokenData::Command(TexCommandPath::Coword(
                                self.next_coword_with(|c| c.is_alphanumeric()).unwrap(),
                            )))
                        }
                        c => {
                            self.chars.eat_char();
                            match c {
                                '{' => Some(TexMathTokenData::LeftDelimiter(TexMathDelimiter::Set)),
                                '}' => {
                                    Some(TexMathTokenData::RightDelimiter(TexMathDelimiter::Set))
                                }
                                _ => todo!(),
                            }
                        }
                    },
                    None => todo!(),
                }
            }
            n if n.is_numeric() => {
                let numeric_str_slice = self.chars.next_numeric_str_slice();
                match numeric_str_slice.parse::<u32>() {
                    Ok(number) => Some(TexMathTokenData::Nat32(number)), // ad hoc
                    Err(_) => {
                        use husky_print_utils::p;

                        p!(numeric_str_slice);
                        todo!()
                    }
                }
            }
            c => {
                self.chars.eat_char();
                match c {
                    '{' => Some(TexMathTokenData::LeftDelimiter(TexMathDelimiter::Curl)),
                    '}' => Some(TexMathTokenData::RightDelimiter(TexMathDelimiter::Curl)),
                    '(' => Some(TexMathTokenData::LeftDelimiter(TexMathDelimiter::Par)),
                    ')' => Some(TexMathTokenData::RightDelimiter(TexMathDelimiter::Par)),
                    '[' => Some(TexMathTokenData::LeftDelimiter(TexMathDelimiter::Box)),
                    ']' => Some(TexMathTokenData::RightDelimiter(TexMathDelimiter::Box)),
                    c => match TexMathLetter::try_from_char(c) {
                        Some(letter) => Some(TexMathTokenData::Letter(letter)),
                        None => Some(TexMathTokenData::Other(c)),
                    },
                }
            }
        }
    }
}

#[test]
fn next_text_token_data_works() {
    fn t(input: &str, expected: &Expect) {
        let db = &DB::default();
        let tokenizer = TexLexer::new(db, input, TexMode::Math);
        let tokens: Vec<_> = tokenizer.map(|(_, token_data)| token_data).collect();
        expected.assert_debug_eq(&(tokens.debug(db)));
    }
    t(
        "hello",
        &expect![[r#"
            [
                TexTokenData::Math(
                    TexMathTokenData::Letter(
                        LowerH,
                    ),
                ),
                TexTokenData::Math(
                    TexMathTokenData::Letter(
                        LowerE,
                    ),
                ),
                TexTokenData::Math(
                    TexMathTokenData::Letter(
                        LowerL,
                    ),
                ),
                TexTokenData::Math(
                    TexMathTokenData::Letter(
                        LowerL,
                    ),
                ),
                TexTokenData::Math(
                    TexMathTokenData::Letter(
                        LowerO,
                    ),
                ),
            ]
        "#]],
    );
    t(
        "0",
        &expect![[r#"
            [
                TexTokenData::Math(
                    TexMathTokenData::Nat32(
                        0,
                    ),
                ),
            ]
        "#]],
    );
    t(
        "\\alpha",
        &expect![[r#"
            [
                TexTokenData::Math(
                    TexMathTokenData::Command(
                        TexCommandPath::Coword(
                            Word(
                                "alpha",
                            ),
                        ),
                    ),
                ),
            ]
        "#]],
    );
    t("{", &expect![[r#"
        [
            TexTokenData::Math(
                TexMathTokenData::LeftDelimiter(
                    Curl,
                ),
            ),
        ]
    "#]]);
    t("}", &expect![[r#"
        [
            TexTokenData::Math(
                TexMathTokenData::RightDelimiter(
                    Curl,
                ),
            ),
        ]
    "#]]);
    t("(", &expect![[r#"
        [
            TexTokenData::Math(
                TexMathTokenData::LeftDelimiter(
                    Par,
                ),
            ),
        ]
    "#]]);
    t(")", &expect![[r#"
        [
            TexTokenData::Math(
                TexMathTokenData::RightDelimiter(
                    Par,
                ),
            ),
        ]
    "#]]);
    t("[", &expect![[r#"
        [
            TexTokenData::Math(
                TexMathTokenData::LeftDelimiter(
                    Box,
                ),
            ),
        ]
    "#]]);
    t("]", &expect![[r#"
        [
            TexTokenData::Math(
                TexMathTokenData::RightDelimiter(
                    Box,
                ),
            ),
        ]
    "#]]);
    t("\\{", &expect![[r#"
        [
            TexTokenData::Math(
                TexMathTokenData::LeftDelimiter(
                    Set,
                ),
            ),
        ]
    "#]]);
    t("\\}", &expect![[r#"
        [
            TexTokenData::Math(
                TexMathTokenData::RightDelimiter(
                    Set,
                ),
            ),
        ]
    "#]]);
}
