use super::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx};

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db]
pub enum SynPrincipalEntityPathExpr {
    Root {
        path_name_token: PathNameRegionalToken,
        principal_entity_path: PrincipalEntityPath,
    },
    Subitem {
        parent: SynPrincipalEntityPathExprIdx,
        colon_colon_token: ColonColonRegionalToken,
        ident_token: SynExprResult<IdentRegionalToken>,
        path: SynExprResult<PrincipalEntityPath>,
    },
}

pub type SynPrincipalEntityPathExprArena = Arena<SynPrincipalEntityPathExpr>;
pub type SynPrincipalEntityPathExprIdx = ArenaIdx<SynPrincipalEntityPathExpr>;
pub type SynPrincipalEntityPathExprMap<V> = ArenaMap<SynPrincipalEntityPathExpr, V>;
