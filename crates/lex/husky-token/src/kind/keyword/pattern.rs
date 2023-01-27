use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PatternKeyword {
    Mut,
    Covariant,
    Contravariant,
}

impl const From<PatternKeyword> for Keyword {
    fn from(val: PatternKeyword) -> Self {
        Keyword::Pattern(val)
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

impl const From<PatternKeyword> for Token {
    fn from(val: PatternKeyword) -> Self {
        Token::Keyword(val.into())
    }
}

impl PatternKeyword {
    pub const fn code(self) -> &'static str {
        match self {
            PatternKeyword::Mut => "mut",
            PatternKeyword::Covariant => "covariant",
            PatternKeyword::Contravariant => "contravariant",
        }
    }
}
