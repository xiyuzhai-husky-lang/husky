use super::*;
use husky_entity_path::MajorEntityPath;
use husky_entity_syn_tree::EntitySynTreeError;
use parsec::IsStreamParser;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum IdentifiableEntityPathExpr {
    Principal {
        item_path_expr: PrincipalEntityPathExprIdx,
        opt_path: Option<PrincipalEntityPath>,
    },
    ScopeResolution {
        parent_expr_idx: PrincipalEntityPathExprIdx,
        parent_path: MajorEntityPath,
        colon_colon_regional_token: ColonColonRegionalToken,
        ident_token: IdentRegionalToken,
    },
}
