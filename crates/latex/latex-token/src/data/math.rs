use super::*;
use latex_command::path::TexCommandPath;
use latex_math_letter::LxMathLetter;
use latex_math_opr::LxMathOpr;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxMathTokenData {
    Command(TexCommandPath),
    LeftDelimiter(LxMathDelimiter),
    RightDelimiter(LxMathDelimiter),
    Letter(LxMathLetter),
    Opr(LxMathOpr),
    Nat32(u32),
    Other(char),
    Subscript,
    Superscript,
    Error(LxMathTokenError),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxMathDelimiter {
    /// `{`,  `}`
    Curl,
    /// `(`, `)`
    Par,
    /// `[`, `]`
    Box,
    /// `\{`, `\}`
    Set,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxMathTokenError {
    UnexpectedNewParagraph,
}

impl<'a> TexLexer<'a> {
    pub(super) fn next_math_token_data(&mut self) -> Option<LxMathTokenData> {
        match self.chars.peek()? {
            '\\' => {
                self.chars.eat_char();
                match self.chars.peek() {
                    Some(c) => match c {
                        c if c.is_alphanumeric() => {
                            Some(LxMathTokenData::Command(TexCommandPath::Coword(
                                self.next_coword_with(|c| c.is_alphanumeric()).unwrap(),
                            )))
                        }
                        c => {
                            self.chars.eat_char();
                            match c {
                                '{' => Some(LxMathTokenData::LeftDelimiter(LxMathDelimiter::Set)),
                                '}' => Some(LxMathTokenData::RightDelimiter(LxMathDelimiter::Set)),
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
                    Ok(number) => Some(LxMathTokenData::Nat32(number)), // ad hoc
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
                    '_' => Some(LxMathTokenData::Subscript),
                    '^' => Some(LxMathTokenData::Superscript),
                    '{' => Some(LxMathTokenData::LeftDelimiter(LxMathDelimiter::Curl)),
                    '}' => Some(LxMathTokenData::RightDelimiter(LxMathDelimiter::Curl)),
                    '(' => Some(LxMathTokenData::LeftDelimiter(LxMathDelimiter::Par)),
                    ')' => Some(LxMathTokenData::RightDelimiter(LxMathDelimiter::Par)),
                    '[' => Some(LxMathTokenData::LeftDelimiter(LxMathDelimiter::Box)),
                    ']' => Some(LxMathTokenData::RightDelimiter(LxMathDelimiter::Box)),
                    c => {
                        if let Some(letter) = LxMathLetter::try_from_char(c) {
                            Some(LxMathTokenData::Letter(letter))
                        } else if let Some(opr) = LxMathOpr::try_from_char(c) {
                            Some(LxMathTokenData::Opr(opr))
                        } else {
                            Some(LxMathTokenData::Other(c))
                        }
                    }
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
                    LxMathTokenData::Letter(
                        LowerH,
                    ),
                ),
                TexTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerE,
                    ),
                ),
                TexTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerL,
                    ),
                ),
                TexTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerL,
                    ),
                ),
                TexTokenData::Math(
                    LxMathTokenData::Letter(
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
                    LxMathTokenData::Nat32(
                        0,
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
                    LxMathTokenData::Nat32(
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
                TexTokenData::Math(
                    LxMathTokenData::Nat32(
                        0,
                    ),
                ),
                TexTokenData::Math(
                    LxMathTokenData::Nat32(
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
                TexTokenData::Math(
                    LxMathTokenData::Nat32(
                        0,
                    ),
                ),
                TexTokenData::Math(
                    LxMathTokenData::Nat32(
                        0,
                    ),
                ),
            ]
        "#]],
    );
    t(
        "0\n\n0",
        &expect![[r#"
            [
                TexTokenData::Math(
                    LxMathTokenData::Nat32(
                        0,
                    ),
                ),
                TexTokenData::Math(
                    LxMathTokenData::Error(
                        LxMathTokenError::UnexpectedNewParagraph,
                    ),
                ),
                TexTokenData::Math(
                    LxMathTokenData::Nat32(
                        0,
                    ),
                ),
            ]
        "#]],
    );
    t(
        "{",
        &expect![[r#"
        [
            TexTokenData::Math(
                LxMathTokenData::LeftDelimiter(
                    Curl,
                ),
            ),
        ]
    "#]],
    );
    t(
        "}",
        &expect![[r#"
        [
            TexTokenData::Math(
                LxMathTokenData::RightDelimiter(
                    Curl,
                ),
            ),
        ]
    "#]],
    );
    t(
        "(",
        &expect![[r#"
        [
            TexTokenData::Math(
                LxMathTokenData::LeftDelimiter(
                    Par,
                ),
            ),
        ]
    "#]],
    );
    t(
        ")",
        &expect![[r#"
        [
            TexTokenData::Math(
                LxMathTokenData::RightDelimiter(
                    Par,
                ),
            ),
        ]
    "#]],
    );
    t(
        "[",
        &expect![[r#"
        [
            TexTokenData::Math(
                LxMathTokenData::LeftDelimiter(
                    Box,
                ),
            ),
        ]
    "#]],
    );
    t(
        "]",
        &expect![[r#"
        [
            TexTokenData::Math(
                LxMathTokenData::RightDelimiter(
                    Box,
                ),
            ),
        ]
    "#]],
    );
    t(
        "\\{",
        &expect![[r#"
        [
            TexTokenData::Math(
                LxMathTokenData::LeftDelimiter(
                    Set,
                ),
            ),
        ]
    "#]],
    );
    t(
        "\\}",
        &expect![[r#"
        [
            TexTokenData::Math(
                LxMathTokenData::RightDelimiter(
                    Set,
                ),
            ),
        ]
    "#]],
    );
    t(
        "+",
        &expect![[r#"
        [
            TexTokenData::Math(
                LxMathTokenData::Opr(
                    Add,
                ),
            ),
        ]
    "#]],
    );
    t(
        "x+1",
        &expect![[r#"
        [
            TexTokenData::Math(
                LxMathTokenData::Letter(
                    LowerX,
                ),
            ),
            TexTokenData::Math(
                LxMathTokenData::Opr(
                    Add,
                ),
            ),
            TexTokenData::Math(
                LxMathTokenData::Nat32(
                    1,
                ),
            ),
        ]
    "#]],
    );
    t(
        "x_1^a+1",
        &expect![[r#"
            [
                TexTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerX,
                    ),
                ),
                TexTokenData::Math(
                    LxMathTokenData::Subscript,
                ),
                TexTokenData::Math(
                    LxMathTokenData::Nat32(
                        1,
                    ),
                ),
                TexTokenData::Math(
                    LxMathTokenData::Superscript,
                ),
                TexTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerA,
                    ),
                ),
                TexTokenData::Math(
                    LxMathTokenData::Opr(
                        Add,
                    ),
                ),
                TexTokenData::Math(
                    LxMathTokenData::Nat32(
                        1,
                    ),
                ),
            ]
        "#]],
    );
}
