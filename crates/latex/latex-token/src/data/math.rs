pub mod digit;

use self::digit::LxMathDigit;
use super::*;
use latex_command::path::LxCommandPath;
use latex_math_letter::LxMathLetter;
use latex_math_opr::LxMathPunctuation;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxMathTokenData {
    Command(LxCommandPath),
    LeftDelimiter(LxMathDelimiter),
    RightDelimiter(LxMathDelimiter),
    Letter(LxMathLetter),
    Punctuation(LxMathPunctuation),
    Digit(LxMathDigit),
    Other(char),
    Subscript,
    Superscript,
    Error(LxMathTokenError),
    MathModeEnd,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxMathDelimiter {
    /// `{`,  `}`
    Curl,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxMathTokenError {
    UnexpectedNewParagraph,
}

impl<'a> LxLexer<'a> {
    pub(crate) fn next_math_token_data(&mut self) -> Option<LxMathTokenData> {
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
                                '{' => Some(LxMathTokenData::Punctuation(
                                    LxMathPunctuation::EscapedLcurl,
                                )),
                                '}' => Some(LxMathTokenData::Punctuation(
                                    LxMathPunctuation::EscapedRcurl,
                                )),
                                _ => todo!(),
                            }
                        }
                    },
                    None => todo!(),
                }
            }
            n if n.is_numeric() => {
                self.chars.eat_char();
                Some(LxMathTokenData::Digit(n.try_into().unwrap()))
            }
            c => {
                self.chars.eat_char();
                match c {
                    '_' => Some(LxMathTokenData::Subscript),
                    '^' => Some(LxMathTokenData::Superscript),
                    '{' => Some(LxMathTokenData::LeftDelimiter(LxMathDelimiter::Curl)),
                    '}' => Some(LxMathTokenData::RightDelimiter(LxMathDelimiter::Curl)),
                    c => {
                        if let Some(letter) = LxMathLetter::try_from_char(c) {
                            Some(LxMathTokenData::Letter(letter))
                        } else if let Some(opr) = LxMathPunctuation::try_from_char(c) {
                            Some(LxMathTokenData::Punctuation(opr))
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
        let mut storage = LxTokenStorage::default();
        let stream = LxLexer::new(db, input, &mut storage).into_math_stream();
        let tokens: Vec<_> = stream.map(|(_, token_data)| token_data).collect();
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
                    LxMathTokenData::Digit(
                        Zero,
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
                    LxMathTokenData::Digit(
                        Zero,
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
                    LxMathTokenData::Digit(
                        Zero,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Digit(
                        Zero,
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
                    LxMathTokenData::Digit(
                        Zero,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Digit(
                        Zero,
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
                    LxMathTokenData::Digit(
                        Zero,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Error(
                        LxMathTokenError::UnexpectedNewParagraph,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Digit(
                        Zero,
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
                    LxMathTokenData::Punctuation(
                        Lpar,
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
                    LxMathTokenData::Punctuation(
                        Rpar,
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
                    LxMathTokenData::Punctuation(
                        Lbox,
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
                    LxMathTokenData::Punctuation(
                        Rbox,
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
                    LxMathTokenData::Punctuation(
                        EscapedLcurl,
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
                    LxMathTokenData::Punctuation(
                        EscapedRcurl,
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
                    LxMathTokenData::Punctuation(
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
                    LxMathTokenData::Punctuation(
                        Add,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Digit(
                        One,
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
                    LxMathTokenData::Digit(
                        One,
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
                    LxMathTokenData::Punctuation(
                        Add,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Digit(
                        One,
                    ),
                ),
            ]
        "#]],
    );
    t(
        "\\int",
        &expect![[r#"
            [
                LxTokenData::Math(
                    LxMathTokenData::Command(
                        LxCommandPath::Coword(
                            Coword(
                                "int",
                            ),
                        ),
                    ),
                ),
            ]
        "#]],
    );
    t(
        "\\int x^3\\sin^3xdx",
        &expect![[r#"
            [
                LxTokenData::Math(
                    LxMathTokenData::Command(
                        LxCommandPath::Coword(
                            Coword(
                                "int",
                            ),
                        ),
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerX,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Superscript,
                ),
                LxTokenData::Math(
                    LxMathTokenData::Digit(
                        Three,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Command(
                        LxCommandPath::Coword(
                            Coword(
                                "sin",
                            ),
                        ),
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Superscript,
                ),
                LxTokenData::Math(
                    LxMathTokenData::Digit(
                        Three,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerX,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerD,
                    ),
                ),
                LxTokenData::Math(
                    LxMathTokenData::Letter(
                        LowerX,
                    ),
                ),
            ]
        "#]],
    );
}
