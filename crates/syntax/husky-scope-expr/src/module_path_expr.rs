use crate::*;
use husky_token::*;

/// mod path expr is top-down
/// because path is resolved top-down
#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum ModulePathExpr {
    Leaf {
        ident: IdentToken,
    },
    Parent {
        ident: IdentToken,
        colon_colon_token: ScopeResolutionToken,
        child: ModulePathExprIdx,
    },
}

pub type ModulePathExprArena = Arena<ModulePathExpr>;
pub type ModulePathExprIdx = ArenaIdx<ModulePathExpr>;
pub type ModulePathExprIdxRange = ArenaIdxRange<ModulePathExpr>;
