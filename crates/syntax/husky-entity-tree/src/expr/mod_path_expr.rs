use crate::*;
use husky_token::*;

/// mod path expr is top-down
/// because path is resolved top-down
#[derive(Debug, PartialEq, Eq)]
pub enum ModPathExpr {
    Leaf {
        ident: IdentifierToken,
    },
    Parent {
        ident: IdentifierToken,
        scope_resolution_token: ScopeResolutionToken,
        child: ModPathExprIdx,
    },
}

pub type ModPathExprArena = Arena<ModPathExpr>;
pub type ModPathExprIdx = ArenaIdx<ModPathExpr>;
pub type ModPathExprIdxRange = ArenaIdxRange<ModPathExpr>;
