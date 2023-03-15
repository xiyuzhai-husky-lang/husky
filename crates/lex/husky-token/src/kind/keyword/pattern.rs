use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PatternKeyword {
    Mut,
    Covariant,
    Contravariant,
    Invariant,
}

// Into<TokenKind> for LiasonKeyword

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum TokenKind {
//     Decorator(Decorator),
//     Keyword(Keyword),
//     Ident(Ident),
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
            PatternKeyword::Invariant => "invariant",
        }
    }
}
