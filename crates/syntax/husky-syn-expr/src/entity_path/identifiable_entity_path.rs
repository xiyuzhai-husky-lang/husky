use super::*;
use husky_entity_path::MajorItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db]
pub enum IdentifiableEntityPathExpr {
    Principal {
        path_expr_idx: SynPrincipalEntityPathExprIdx,
        opt_path: Option<PrincipalEntityPath>,
    },
    AssociatedItem {
        parent_expr_idx: SynPrincipalEntityPathExprIdx,
        parent_path: MajorItemPath,
        colon_colon_regional_token: ColonColonRegionalToken,
        ident_token: IdentRegionalToken,
    },
}
