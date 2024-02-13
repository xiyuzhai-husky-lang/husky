use super::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx};

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum SynPrincipalEntityPathExpr {
    Root {
        path_name_token: PathNameRegionalToken,
        principal_entity_path: PrincipalEntityPath,
    },
    Subitem {
        parent: PrincipalEntityPathSynExprIdx,
        colon_colon_token: ColonColonRegionalToken,
        ident_token: SynExprResult<IdentRegionalToken>,
        path: SynExprResult<PrincipalEntityPath>,
    },
}

pub type SynPrincipalEntityPathExprArena = Arena<SynPrincipalEntityPathExpr>;
pub type PrincipalEntityPathSynExprIdx = ArenaIdx<SynPrincipalEntityPathExpr>;
pub type SynPrincipalEntityPathExprMap<V> = ArenaMap<SynPrincipalEntityPathExpr, V>;
