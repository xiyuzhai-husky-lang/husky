use super::*;
use husky_coword::Coword;
use husky_tex_command::path::TexCommandPath;
use husky_tex_math_letter::TexMathLetter;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexMathTokenData {
    Command(TexCommandPath),
    Delimiter(TexMathDelimiter),
    Letter(TexMathLetter),
    Nat32(u32),
    Other(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexMathDelimiter {
    /// `{`,  `}`
    Curl,
    /// `[`, `]`
    Box,
    /// `(`, `)`
    Par,
    /// `\{`, `\}`
    Set,
    /// `{`, `}`
    Attach,
}

pub enum Script {}

impl<'a> TexLexer<'a> {
    pub(super) fn next_math_token_data(&mut self) -> Option<TexMathTokenData> {
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
                    Err(_) => {
                        use husky_print_utils::p;

                        p!(numeric_str_slice);
                        todo!()
                    }
                }
            }
            '{' => todo!(),
            c => {
                self.chars.next();
                match TexMathLetter::try_from_char(c) {
                    Some(letter) => Some(letter.into()),
                    None => Some(c.into()),
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
}
