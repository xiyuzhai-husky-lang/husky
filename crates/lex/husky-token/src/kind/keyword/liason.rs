use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LiasonKeyword {
    Mut,
}

impl const Into<Keyword> for LiasonKeyword {
    fn into(self) -> Keyword {
        Keyword::Liason(self)
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

impl const Into<TokenKind> for LiasonKeyword {
    fn into(self) -> TokenKind {
        TokenKind::Keyword(self.into())
    }
}

impl LiasonKeyword {
    pub const fn as_str(self) -> &'static str {
        match self {
            LiasonKeyword::Mut => "mut",
        }
    }
}
