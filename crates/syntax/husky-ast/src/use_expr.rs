// use crate::*;
// use husky_opn_syntax::{BinaryOpr, BinaryPureClosedOpr, Bracket};
// use husky_token::{Keyword, Punctuation, RangedTokenSheet, Token, TokenStream};
// use husky_word::Identifier;
// use idx_arena::map::ArenaMap;

// pub type UseExprArena = Arena<UseExpr>;
// pub type UseExprIdx = ArenaIdx<UseExpr>;
// pub type UseExprIdxRange = ArenaIdxRange<UseExpr>;

// #[derive(Debug, PartialEq, Eq)]
// pub enum UseExpr {
//     // *
//     All,
//     One {
//         ident: Identifier,
//     },
//     Parent {
//         ident: Identifier,
//         children: UseExprIdxRange,
//     },
//     Err(EntityUseExprError),
// }

// #[derive(Debug, PartialEq, Eq, thiserror::Error)]
// pub enum EntityUseExprError {
//     #[error("expect something")]
//     ExpectSomething,
//     #[error("expect identifier or `{{` or `*`")]
//     ExpectIdentifierOrLCurlyOrStar,
//     #[error("expect `::`")]
//     ExpectScopeResolution,
// }

// impl<Db: AstDb> salsa::DebugWithDb<Db> for UseExpr {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//         db: &Db,
//         level: salsa::DebugFormatLevel,
//     ) -> std::fmt::Result {
//         match self {
//             Self::All {} => f.debug_struct("All").finish(),
//             Self::One { ident } => f
//                 .debug_struct("One")
//                 .field("ident", &ident.debug_with(db, level))
//                 .finish(),
//             Self::Parent { ident, children } => f
//                 .debug_struct("ScopeResolution")
//                 .field("ident", &ident.debug_with(db, level))
//                 .field("children", children)
//                 .finish(),
//             Self::Err(arg0) => f.debug_tuple("Err").field(arg0).finish(),
//         }
//     }
// }
