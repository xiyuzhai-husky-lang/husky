use super::*;
use latex_command::path::LxCommandPath;
use latex_math_letter::LxMathLetter;
use latex_math_opr::LxMathOpr;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxMathTokenData {
    Command(LxCommandPath),
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

impl<'a> LxLexer<'a> {
    pub(super) fn next_math_token_data(&mut self) -> Option<LxMathTokenData> {
        match self.chars.peek()? {
            '\\' => {
                self.chars.eat_char();
                match self.chars.peek() {
                    Some(c) => match c {
                        c if c.is_alphanumeric() => {
                            Some(LxMathTokenData::Command(LxCommandPath::Coword(
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
        let tokenizer = LxLexer::new(db, input, LxMode::Math);
        let tokens: Vec<_> = tokenizer.map(|(_, token_data)| token_data).collect();
        expected.assert_debug_eq(&(tokens.debug(db)));
    }
    t(
        "hello",
        &expect![[r#"
            [
                LxTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerH,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerE,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerL,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerL,
                    ),
                ),
                LxTokenData::Math(
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
                LxTokenData::Math(
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
                LxTokenData::Math(
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
                LxTokenData::Math(
                    LxMathTokenData::Nat32(
                        0,
                    ),
                ),
                LxTokenData::Math(
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
                LxTokenData::Math(
                    LxMathTokenData::Nat32(
                        0,
                    ),
                ),
                LxTokenData::Math(
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
                LxTokenData::Math(
                    LxMathTokenData::Nat32(
                        0,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Error(
                        LxMathTokenError::UnexpectedNewParagraph,
                    ),
                ),
                LxTokenData::Math(
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
            LxTokenData::Math(
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
            LxTokenData::Math(
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
            LxTokenData::Math(
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
            LxTokenData::Math(
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
            LxTokenData::Math(
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
            LxTokenData::Math(
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
            LxTokenData::Math(
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
            LxTokenData::Math(
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
            LxTokenData::Math(
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
            LxTokenData::Math(
                LxMathTokenData::Letter(
                    LowerX,
                ),
            ),
            LxTokenData::Math(
                LxMathTokenData::Opr(
                    Add,
                ),
            ),
            LxTokenData::Math(
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
                LxTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerX,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Subscript,
                ),
                LxTokenData::Math(
                    LxMathTokenData::Nat32(
                        1,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Superscript,
                ),
                LxTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerA,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Opr(
                        Add,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Nat32(
                        1,
                    ),
                ),
            ]
        "#]],
    );
}
