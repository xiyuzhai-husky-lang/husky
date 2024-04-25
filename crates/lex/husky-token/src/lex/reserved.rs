use vec_like::VecPairMap;

use super::*;

#[salsa::tracked(jar = TokenJar, return_ref)]
pub(crate) fn reserved_cowords(_db: &::salsa::Db) -> VecPairMap<&'static str, Pretoken> {
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
            Keyword::Unreachable,
            TypeEntityKeyword::Extern,
            TypeEntityKeyword::Struct,
            TypeEntityKeyword::Enum,
            TypeEntityKeyword::Record,
            TypeEntityKeyword::Structure,
            TypeEntityKeyword::Inductive,
            FormKeyword::Def,
            FormKeyword::Val,
            FormKeyword::Fn,
            FormKeyword::Gn,
            FormKeyword::Vn,
            FormKeyword::Pn,
            FormKeyword::Qn,
            FormKeyword::Tn,
            FormKeyword::Theorem,
            FormKeyword::Lemma,
            FormKeyword::Proposition,
            FormKeyword::Memo,
            FormKeyword::Type,
            StmtKeyword::Let,
            StmtKeyword::Elif,
            StmtKeyword::Else,
            StmtKeyword::If,
            StmtKeyword::Match,
            StmtKeyword::Forext,
            StmtKeyword::While,
            StmtKeyword::Do,
            StmtKeyword::Break,
            StmtKeyword::Return,
            StmtKeyword::Assert,
            StmtKeyword::Require,
            ModifierKeyword::Mut,
            ModifierKeyword::Ref,
            ModifierKeyword::Le,
            ModifierKeyword::Covariant,
            ModifierKeyword::Contravariant,
            ModifierKeyword::Invariant,
            ConnectionKeyword::Extends,
            ConnectionKeyword::Where,
            Keyword::Pub,
            Keyword::Static,
            WordOpr::And,
            WordOpr::Or,
            WordOpr::As,
            WordOpr::Be,
            AmbiguousPretoken::For,
            Keyword::Const,
            PronounKeyword::Crate,
            PronounKeyword::SelfType,
            PronounKeyword::SelfValue,
            PronounKeyword::Super,
            EndKeyword::With,
            BoolLiteralTokenData::True,
            BoolLiteralTokenData::False,
        ]
        .into_iter(),
    )
    .unwrap()
}
