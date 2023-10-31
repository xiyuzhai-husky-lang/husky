use husky_sema_expr::{
    helpers::analysis::sema_expr_region_contains_gn, SemaExprData, SemaStmtData,
};
use husky_syn_decl::{FugitiveSynDecl, HasSynDecl};
use husky_syn_defn::{FugitiveSynDefn, HasSynDefn};

use crate::registry::{
    associated_trace::VoidAssociatedTraceRegistry, trace_path::TracePathRegistry,
};

use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new)]
pub struct ValItemTracePath {
    // todo: make it like submodule trace path??
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
        let tokens = val_item_trace_view_lines(db, self);
        TraceViewData::new(tokens.data().to_vec(), /* ad hoc */ true)
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        val_item_trace_subtraces(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn val_item_trace_view_lines(db: &dyn TraceDb, val_item_trace: ValItemTrace) -> TraceViewLines {
    use husky_entity_syn_tree::helpers::tokra_region::HasDeclTokraRegion;
    use husky_entity_syn_tree::HasSynNodePath;
    let val_item_trace_path = val_item_trace.path(db);
    let val_item_path = val_item_trace_path.val_item_path(db);
    let token_idx_range = val_item_path
        .syn_node_path(db)
        .decl_tokra_region_token_idx_range(db);
    TraceViewLines::new(
        val_item_path.module_path(db),
        token_idx_range,
        VoidAssociatedTraceRegistry,
        db,
    )
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn val_item_trace_subtraces(db: &dyn TraceDb, val_item_trace: ValItemTrace) -> Vec<Trace> {
    let val_item_trace_path = val_item_trace.path(db);
    let val_item_path = val_item_trace_path.val_item_path(db);
    let Ok(FugitiveSynDefn::Val(val_item_defn)) = val_item_path.syn_defn(db) else {
        unreachable!("no error at trace stage")
    };
    let Some((body, syn_expr_region)) = val_item_defn.body_with_syn_expr_region(db) else {
        return vec![];
    };
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    let sema_expr_region_data = sema_expr_region.data(db);
    let body: SemaExprIdx = sema_expr_region_data.syn_root_to_sema_expr_idx(body);
    let sema_expr_arena = sema_expr_region_data.sema_expr_arena();
    match sema_expr_region_contains_gn(db, sema_expr_region) {
        true => match body.data(sema_expr_arena) {
            &SemaExprData::Block { stmts } => LazyStmtTrace::from_stmts(
                val_item_trace_path,
                val_item_trace,
                stmts,
                sema_expr_region,
                db,
            ),
            _ => todo!(),
        },
        false => match body.data(sema_expr_arena) {
            &SemaExprData::Block { stmts } => EagerStmtTrace::from_stmts(
                val_item_trace_path,
                val_item_trace,
                stmts,
                sema_expr_region,
                db,
            ),
            _ => todo!(),
        },
    }
}
