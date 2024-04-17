use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemPathExpr {
    Principal {
        path_expr_idx: SynPrincipalEntityPathSynExprIdx,
        opt_path: Option<PrincipalEntityPath>,
    },
    AssocItem {
        parent_expr_idx: SynPrincipalEntityPathSynExprIdx,
        parent_path: MajorItemPath,
        colon_colon_regional_token: ColonColonRegionalToken,
        ident_token: IdentRegionalToken,
    },
}
