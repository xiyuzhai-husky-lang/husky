use crate::*;
use husky_token::*;

/// mod path expr is top-down
/// because path is resolved top-down
#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum ModPathExpr {
    Leaf {
        ident: IdentToken,
    },
    Parent {
        ident: IdentToken,
        scope_resolution_token: ScopeResolutionToken,
        child: ModPathExprIdx,
    },
}

pub type ModPathExprArena = Arena<ModPathExpr>;
pub type ModPathExprIdx = ArenaIdx<ModPathExpr>;
pub type ModPathExprIdxRange = ArenaIdxRange<ModPathExpr>;
