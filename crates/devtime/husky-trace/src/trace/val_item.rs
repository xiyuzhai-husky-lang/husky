use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_hir_defn::HasHirDefn;
use husky_sema_expr::{helpers::analysis::sema_expr_region_contains_gn, SemaExprData, SemaExprDb};
use husky_syn_defn::{item_syn_defn, ItemSynDefn};

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValItemTracePathData {
    val_item_path: FugitivePath,
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValItemTraceData {
    path: TracePath,
    val_item_path: FugitivePath,
}

impl Trace {
    pub fn from_val_item_path(val_item_path: FugitivePath, db: &::salsa::Db) -> Self {
        debug_assert_eq!(val_item_path.fugitive_kind(db), FugitiveKind::Val);
        let path = TracePath::new(ValItemTracePathData { val_item_path }, db);
        Trace::new(
            path,
            ValItemTraceData {
                path,
                val_item_path,
            }
            .into(),
            db,
        )
    }
}

impl ValItemTraceData {
    pub(super) fn view_lines(&self, db: &::salsa::Db) -> TraceViewLines {
        use husky_entity_syn_tree::HasSynNodePath;
        let val_item_path = self.val_item_path;
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

    pub(super) fn have_subtraces(self, db: &::salsa::Db) -> bool {
        // ad hoc, incorrect
        self.val_item_path.hir_defn(db).is_some()
    }

    pub(super) fn subtraces(&self, trace: Trace, db: &::salsa::Db) -> Vec<Trace> {
        let biological_parent_path = self.path;
        let biological_parent = trace;
        let val_item_path = self.val_item_path;
        let Some(ItemSynDefn {
            body,
            syn_expr_region,
        }) = item_syn_defn(db, val_item_path.into())
        else {
            return vec![];
        };
        let sema_expr_region = db.sema_expr_region(syn_expr_region);
        let sema_expr_region_data = sema_expr_region.data(db);
        let body: SemaExprIdx = sema_expr_region_data.syn_root_to_sema_expr_idx(body);
        let sema_expr_arena = sema_expr_region_data.sema_expr_arena();
        match sema_expr_region_contains_gn(db, sema_expr_region) {
            true => match body.data(sema_expr_arena) {
                &SemaExprData::Block { stmts } => Trace::new_lazy_stmts(
                    biological_parent_path,
                    biological_parent,
                    stmts,
                    sema_expr_region,
                    db,
                ),
                _ => todo!(),
            },
            false => match body.data(sema_expr_arena) {
                &SemaExprData::Block { stmts } => Trace::new_eager_stmts(
                    biological_parent_path,
                    biological_parent,
                    stmts,
                    sema_expr_region,
                    db,
                ),
                _ => todo!(),
            },
        }
    }

    pub(super) fn val_repr(&self, db: &::salsa::Db) -> ValRepr {
        ValRepr::new_val_item(self.val_item_path, db)
    }

    pub(super) fn val_repr_expansion(&self, trace_id: Trace, db: &::salsa::Db) -> ValReprExpansion {
        trace_id
            .val_repr(db)
            .expect("should be some")
            .expansion(db)
            .expect("should be some")
    }
}
