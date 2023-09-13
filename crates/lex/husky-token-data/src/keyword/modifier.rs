use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ModifierKeyword {
    Mut,
    Ref,
    Le,
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

impl From<ModifierKeyword> for TokenData {
    fn from(val: ModifierKeyword) -> Self {
        TokenData::Keyword(val.into())
    }
}

impl ModifierKeyword {
    pub const fn code(self) -> &'static str {
        match self {
            ModifierKeyword::Mut => "mut",
            ModifierKeyword::Ref => "ref",
            ModifierKeyword::Le => "le",
            ModifierKeyword::Covariant => "covariant",
            ModifierKeyword::Contravariant => "contravariant",
            ModifierKeyword::Invariant => "invariant",
        }
    }
}
