use vec_like::VecPairMap;

use super::*;

#[salsa::tracked(jar = TokenJar, return_ref)]
pub(crate) fn reserved_words(_db: &dyn TokenDb) -> VecPairMap<&'static str, Pretoken> {
    macro_rules! gen_reserved_words {
        ($($args: expr),*,) => {
            [
                $(($args.code(), $args.into())),*
            ]
        };
    }
    gen_reserved_words![
        Keyword::Main,
        Keyword::Use,
        Keyword::Mod,
        Keyword::Trait,
        Keyword::Impl,
        Keyword::Visual,
        ConfigKeyword::Task,
        TypeEntityKeyword::Extern,
        TypeEntityKeyword::Struct,
        TypeEntityKeyword::Enum,
        TypeEntityKeyword::Record,
        TypeEntityKeyword::Structure,
        TypeEntityKeyword::Inductive,
        FormKeyword::Def,
        FormKeyword::Fn,
        FormKeyword::Theorem,
        FormKeyword::Lemma,
        FormKeyword::Proposition,
        FormKeyword::Const,
        FormKeyword::Var,
        FormKeyword::Type,
        FormKeyword::Gn,
        StmtKeyword::Let,
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
        PatternKeyword::Mut,
        PatternKeyword::Covariant,
        PatternKeyword::Contravariant,
        PatternKeyword::Invariant,
        Keyword::Pub,
        Keyword::Async,
        Keyword::Static,
        WordOpr::And,
        WordOpr::Or,
        WordOpr::As,
        WordOpr::Be,
        AmbiguousPretoken::For,
        PronounKeyword::Crate,
        PronounKeyword::SelfType,
        PronounKeyword::SelfValue,
        PronounKeyword::Super,
        EndKeyword::With,
        EndKeyword::Into,
        BoolLiteral::True,
        BoolLiteral::False,
    ]
    .into_iter()
    .collect()
}
