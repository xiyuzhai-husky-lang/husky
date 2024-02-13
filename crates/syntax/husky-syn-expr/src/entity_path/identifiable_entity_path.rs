use super::*;
use husky_entity_path::MajorItemPath;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IdentifiableEntityPathExpr {
    Principal {
        path_expr_idx: PrincipalEntityPathSynExprIdx,
        opt_path: Option<PrincipalEntityPath>,
    },
    AssocItem {
        parent_expr_idx: PrincipalEntityPathSynExprIdx,
        parent_path: MajorItemPath,
        colon_colon_regional_token: ColonColonRegionalToken,
        ident_token: IdentRegionalToken,
    },
}
