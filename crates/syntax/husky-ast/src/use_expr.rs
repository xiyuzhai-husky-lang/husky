use crate::*;
use husky_opn_syntax::{BinaryPunctuation, BinaryPureClosedPunctuation, Bracket};
use husky_token::{Keyword, Punctuation, TokenKind, TokenSheet, TokenStream};
use husky_word::Identifier;
use idx_arena::map::ArenaMap;

pub type UseExprArena = Arena<UseExpr>;
pub type UseExprIdx = ArenaIdx<UseExpr>;
pub type UseExprIdxRange = ArenaIdxRange<UseExpr>;

#[derive(Debug, PartialEq, Eq)]
pub enum UseExpr {
    // *
    All {},
    One {
        ident: Identifier,
    },
    ScopeResolution {
        parent: Identifier,
        child: UseExprIdx,
    },
    Multiple {
        exprs: UseExprIdxRange,
    },
    Err(EntityUseExprError),
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum EntityUseExprError {
    #[error("expect something")]
    ExpectSomething,
    #[error("expect identifier or `{{` or `*`")]
    ExpectIdentifierOrLCurlyOrStar,
    #[error("expect `::`")]
    ExpectScopeResolution,
}

impl<Db: AstDb> salsa::DebugWithDb<Db> for UseExpr {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            Self::All {} => f.debug_struct("All").finish(),
            Self::One { ident } => f
                .debug_struct("One")
                .field("ident", &ident.debug_with(db, include_all_fields))
                .finish(),
            Self::ScopeResolution { parent, child } => f
                .debug_struct("ScopeResolution")
                .field("parent", &parent.debug_with(db, include_all_fields))
                .field("child", child)
                .finish(),
            Self::Multiple { exprs } => f.debug_struct("Multiple").field("exprs", exprs).finish(),
            Self::Err(arg0) => f.debug_tuple("Err").field(arg0).finish(),
        }
    }
}
