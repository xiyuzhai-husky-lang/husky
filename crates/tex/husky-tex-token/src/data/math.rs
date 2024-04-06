use super::*;
use husky_tex_math_letter::TexMathLetter;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexMathTokenData {
    Command(TexMathCommand),
    Delimiter(TexMathDelimiter),
    Letter(TexMathLetter),
    Nat32(u32),
    Other(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TexMathCommand {
    Frac,
    Abs,
    Norm,
    Ang,
    Perp,
    Int,
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
            '\\' => todo!(),
            numeric if numeric.is_numeric() => {
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
    )
}
