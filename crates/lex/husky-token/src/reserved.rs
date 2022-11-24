use crate::*;

macro_rules! gen_reserved_words {
    ($($args: expr),*,) => {
        &[
            $(($args.as_str(), $args.into())),*
        ]
    };
}

pub const RESERVED_WORDS: &[(&'static str, TokenKind)] = gen_reserved_words![
    ConfigKeyword::Task,
    StmtKeyword::Let,
    StmtKeyword::Var,
    StmtKeyword::Elif,
    StmtKeyword::Else,
    StmtKeyword::If,
    StmtKeyword::Match,
    StmtKeyword::Case,
    StmtKeyword::ForExt,
    StmtKeyword::While,
    StmtKeyword::Do,
    StmtKeyword::Break,
    StmtKeyword::Return,
    StmtKeyword::Assert,
    StmtKeyword::Require,
    TypeKeyword::Struct,
    TypeKeyword::Enum,
    TypeKeyword::Record,
    TypeKeyword::Structure,
    TypeKeyword::Inductive,
    LiasonKeyword::Mut,
    Decorator::Pub,
    Decorator::Private,
    Decorator::Async,
    Decorator::Static,
    WordOpr::And,
    WordOpr::Or,
    WordOpr::As,
    WordOpr::Be,
];
