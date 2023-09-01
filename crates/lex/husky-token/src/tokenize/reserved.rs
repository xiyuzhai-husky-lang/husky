use vec_like::VecPairMap;

use super::*;

#[salsa::tracked(jar = TokenJar, return_ref)]
pub(crate) fn reserved_cowords(_db: &dyn TokenDb) -> VecPairMap<&'static str, Pretoken> {
    macro_rules! gen_reserved_cowords {
        ($($args: expr),*,) => {
            [
                $(($args.code(), $args.into())),*
            ]
        };
    }
    VecPairMap::from_iter_assuming_no_repetitions(
        gen_reserved_cowords![
            Keyword::Use,
            Keyword::Mod,
            Keyword::Trait,
            Keyword::Impl,
            Keyword::Sorry,
            Keyword::Todo,
            ConfigKeyword::Task,
            TypeEntityKeyword::Extern,
            TypeEntityKeyword::Struct,
            TypeEntityKeyword::Enum,
            TypeEntityKeyword::Record,
            TypeEntityKeyword::Structure,
            TypeEntityKeyword::Inductive,
            FugitiveKeyword::Def,
            FugitiveKeyword::Fn,
            FugitiveKeyword::Theorem,
            FugitiveKeyword::Lemma,
            FugitiveKeyword::Proposition,
            FugitiveKeyword::Constexpr,
            FugitiveKeyword::Val,
            FugitiveKeyword::Type,
            FugitiveKeyword::Gn,
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
            Keyword::Const,
            ModifierKeyword::Mut,
            ModifierKeyword::Ref,
            ModifierKeyword::Le,
            ModifierKeyword::Covariant,
            ModifierKeyword::Contravariant,
            ModifierKeyword::Invariant,
            ConnectionKeyword::Extends,
            ConnectionKeyword::Where,
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
        .into_iter(),
    )
    .unwrap()
}
