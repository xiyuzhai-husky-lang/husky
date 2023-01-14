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
        FormKeyword::Def,
        FormKeyword::Func,
        FormKeyword::Proc,
        FormKeyword::Fn,
        FormKeyword::Function,
        FormKeyword::Theorem,
        FormKeyword::Lemma,
        FormKeyword::Proposition,
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
        AttributeKeyword::Pub,
        AttributeKeyword::Private,
        AttributeKeyword::Async,
        AttributeKeyword::Static,
        WordOpr::And,
        WordOpr::Or,
        WordOpr::As,
        WordOpr::Be,
        AmbiguousPretoken::For,
        PronounKeyword::Crate,
        PronounKeyword::SelfType,
        PronounKeyword::SelfValue,
        PronounKeyword::Super,
    ]
    .into_iter()
    .collect()
}
