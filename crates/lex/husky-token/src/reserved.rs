use crate::*;

pub const RESERVED_WORDS: &[(&'static str, TokenKind)] = &[
    ("task", ConfigKeyword::Task.into()),
    ("let", StmtKeyword::Let.into()),
    ("and", WordOpr::And.into()),
];
