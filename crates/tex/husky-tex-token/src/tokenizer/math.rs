use super::*;
use husky_tex_math_letter::TexMathLetter;

impl<'a> TexTokenizer<'a> {
    pub(super) fn next_math_token_data(&mut self) -> Option<TexMathTokenData> {
        match self.chars.next()? {
            '\\' => todo!(),
            numeric if numeric.is_numeric() => todo!(),
            '{' => todo!(),
            c => match TexMathLetter::try_from_char(c) {
                Some(letter) => Some(letter.into()),
                None => Some(c.into()),
            },
        }
    }
}

#[test]
fn next_math_token_data_works() {
    let db = &DB::default();
    let tokenizer = TexTokenizer::new(db, "hello", TexMode::Text);
    let tokens: Vec<_> = tokenizer.map(|(_, token_data)| token_data).collect();
    expect![[""]].assert_debug_eq(&tokens)
}
