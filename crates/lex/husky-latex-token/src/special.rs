use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnglishSpecialToken {
    /// =
    Equals,
    /// .
    Period,
    /// ,
    Comma,
    /// :
    Colon,
    /// +
    Plus,
    /// {
    LeftCurly,
    /// }
    RightCurly,
    /// [
    LeftBox,
    /// ]
    RightBox,
}

impl From<EnglishSpecialToken> for EnglishTokenVariant {
    fn from(val: EnglishSpecialToken) -> Self {
        EnglishTokenVariant::Special(val)
    }
}

impl EnglishSpecialToken {
    pub fn describe(self) -> &'static str {
        match self {
            EnglishSpecialToken::Equals => "an equals",
            EnglishSpecialToken::Period => "a period",
            EnglishSpecialToken::Comma => "a comma",
            EnglishSpecialToken::RightCurly => "a right brace",
            EnglishSpecialToken::LeftCurly => "a left brace",
            EnglishSpecialToken::RightBox => "a right bracket",
            EnglishSpecialToken::LeftBox => "a left bracket",
            EnglishSpecialToken::Colon => "a colon",
            EnglishSpecialToken::Plus => "a plus",
        }
    }
}
