use super::*;
use husky_entity_path::{MajorEntityPath, MajorItemPath};
use husky_entity_syn_tree::EntitySynTreeError;
use parsec::IsStreamParser;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum IdentifiableEntityPathExpr {
    Principal {
        item_path_expr: PrincipalEntityPathExprIdx,
        opt_path: Option<PrincipalEntityPath>,
    },
    AssociatedItem {
        parent_expr_idx: PrincipalEntityPathExprIdx,
        parent_path: MajorItemPath,
        colon_colon_regional_token: ColonColonRegionalToken,
        ident_token: IdentRegionalToken,
    },
}
