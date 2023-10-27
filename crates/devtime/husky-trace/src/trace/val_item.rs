use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new)]
pub struct ValItemTracePath {
    pub val_item_path: FugitivePath,
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new)]
pub struct ValItemTrace {
    #[id]
    pub path: ValItemTracePath,
}

impl ValItemTrace {
    pub fn from_val_item_path(val_item_path: FugitivePath, db: &dyn TraceDb) -> Self {
        debug_assert_eq!(val_item_path.fugitive_kind(db), FugitiveKind::Val);
        ValItemTrace::new(db, ValItemTracePath::new(db, val_item_path))
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        let tokens = val_item_view_tokens(db, self);
        TraceViewData::new(tokens.data().to_vec())
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
pub(crate) fn val_item_view_tokens(
    db: &dyn TraceDb,
    val_item_trace: ValItemTrace,
) -> TraceViewTokens {
    use husky_entity_syn_tree::helpers::tokra_region::HasDeclTokraRegion;
    use husky_entity_syn_tree::HasSynNodePath;
    let val_item_trace_path = val_item_trace.path(db);
    let val_item_path = val_item_trace_path.val_item_path(db);
    let token_idx_range = val_item_path
        .syn_node_path(db)
        .decl_tokra_region_token_idx_range(db);
    TraceViewTokens::new(val_item_path.module_path(db), token_idx_range, db)
}
