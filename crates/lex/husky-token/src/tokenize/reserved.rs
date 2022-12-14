use super::*;

macro_rules! gen_reserved_words {
    ($($args: expr),*,) => {
        &[
            $(($args.as_str(), $args.into())),*
        ]
    };
}

pub(crate) const RESERVED_WORDS: &[(&'static str, RawTokenVariant)] = gen_reserved_words![
    Keyword::Main,
    Keyword::Use,
    Keyword::Mod,
    Keyword::Visual,
    Keyword::Impl,
    ConfigKeyword::Task,
    ParadigmKeyword::Def,
    ParadigmKeyword::Func,
    ParadigmKeyword::Proc,
    ParadigmKeyword::Fn,
    StmtKeyword::Let,
    StmtKeyword::Var,
    StmtKeyword::Elif,
    StmtKeyword::Else,
    StmtKeyword::If,
    StmtKeyword::Match,
    StmtKeyword::ForExt,
    StmtKeyword::While,
    StmtKeyword::Do,
    StmtKeyword::Break,
    StmtKeyword::Return,
    StmtKeyword::Assert,
    StmtKeyword::Require,
    TypeKeyword::Type,
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
