use super::*;
use husky_entity_path::{MajorEntityPath, MajorItemPath};
use husky_entity_syn_tree::EntitySynTreeError;
use parsec::IsStreamParser;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
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
