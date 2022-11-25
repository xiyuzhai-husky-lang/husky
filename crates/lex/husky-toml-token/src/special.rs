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

impl Into<TomlTokenVariant> for TomlSpecialToken {
    fn into(self) -> TomlTokenVariant {
        TomlTokenVariant::Special(self)
    }
}
