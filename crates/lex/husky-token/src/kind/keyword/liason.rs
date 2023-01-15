use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LiasonKeyword {
    Mut,
}

impl const From<LiasonKeyword> for Keyword {
    fn from(val: LiasonKeyword) -> Self {
        Keyword::Liason(val)
    }
}

// Into<TokenKind> for LiasonKeyword

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum TokenKind {
//     Decorator(Decorator),
//     Keyword(Keyword),
//     Identifier(Identifier),
//     Special(SpecialToken),
//     WordOpr(WordOpr),
//     Literal(LiteralToken),
//     Unrecognized(char),
//     IllFormedLiteral(LiteralToken),
// }

impl const From<LiasonKeyword> for Token {
    fn from(val: LiasonKeyword) -> Self {
        Token::Keyword(val.into())
    }
}

impl LiasonKeyword {
    pub const fn code(self) -> &'static str {
        match self {
            LiasonKeyword::Mut => "mut",
        }
    }
}
