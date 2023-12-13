use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TomlSpecialToken {
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

impl From<TomlSpecialToken> for TomlTokenData {
    fn from(val: TomlSpecialToken) -> Self {
        TomlTokenData::Special(val)
    }
}

impl TomlSpecialToken {
    pub fn describe(self) -> &'static str {
        match self {
            TomlSpecialToken::Equals => "an equals",
            TomlSpecialToken::Period => "a period",
            TomlSpecialToken::Comma => "a comma",
            TomlSpecialToken::RightCurly => "a right brace",
            TomlSpecialToken::LeftCurly => "a left brace",
            TomlSpecialToken::RightBox => "a right bracket",
            TomlSpecialToken::LeftBox => "a left bracket",
            TomlSpecialToken::Colon => "a colon",
            TomlSpecialToken::Plus => "a plus",
        }
    }
}
