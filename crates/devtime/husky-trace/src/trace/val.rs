use super::*;
use crate::registry::assoc_trace::VoidAssocTraceRegistry;
use husky_hir_defn::defn::HasHirDefn;
use husky_sem_expr::{helpers::analysis::sem_expr_region_requires_lazy, SemExprData, SemExprDb};
use husky_syn_defn::{item_syn_defn, ItemSynDefn};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValTracePathData {
    // todo: more general paths
    val_path: MajorFormPath,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValTraceData {
    path: TracePath,
    // todo: more general paths
    val_path: MajorFormPath,
}

impl Trace {
    pub fn from_major_val_form_path(val_path: MajorFormPath, db: &::salsa::Db) -> Self {
        debug_assert_eq!(val_path.kind(db), MajorFormKind::Val);
        let path = TracePath::new(ValTracePathData { val_path }, db);
        Trace::new(path, ValTraceData { path, val_path }.into(), db)
    }
}

impl ValTraceData {
    pub(super) fn view_lines(&self, db: &::salsa::Db) -> TraceViewLines {
        use husky_entity_tree::node::HasSynNodePath;
        let val_path = self.val_path;
        let token_idx_range = val_path
            .syn_node_path(db)
            .decl_tokra_region_token_idx_range(db);
        TraceViewLines::new(
            val_path.module_path(db),
            token_idx_range,
            VoidAssocTraceRegistry,
            db,
        )
    }

    pub(super) fn have_subtraces(self, db: &::salsa::Db) -> bool {
        // ad hoc, incorrect
        self.val_path.hir_defn(db).is_some()
    }

    pub(super) fn subtraces(&self, trace: Trace, db: &::salsa::Db) -> Vec<Trace> {
        let biological_parent_path = self.path;
        let biological_parent = trace;
        let val_path = self.val_path;
        let Some(ItemSynDefn {
            body,
            syn_expr_region,
        }) = item_syn_defn(db, val_path.into())
        else {
            return vec![];
        };
        let sem_expr_region = db.sem_expr_region(syn_expr_region);
        let sem_expr_region_data = sem_expr_region.data(db);
        let body: SemExprIdx = sem_expr_region_data.syn_root_to_sem_expr_idx(body);
        let sem_expr_arena = sem_expr_region_data.sem_expr_arena();
        match sem_expr_region_requires_lazy(db, sem_expr_region) {
            true => match body.data(sem_expr_arena) {
                &SemExprData::Block { stmts } => Trace::new_lazy_stmts(
                    biological_parent_path,
                    biological_parent,
                    stmts,
                    sem_expr_region,
                    db,
                ),
                _ => todo!(),
            },
            false => match body.data(sem_expr_arena) {
                &SemExprData::Block { stmts } => Trace::new_eager_stmts(
                    biological_parent_path,
                    biological_parent,
                    stmts,
                    sem_expr_region,
                    db,
                ),
                _ => todo!(),
            },
        }
    }

    pub(super) fn ki_repr(&self, db: &::salsa::Db) -> KiRepr {
        KiRepr::new_val(self.val_path, db)
    }

    pub(super) fn ki_repr_expansion(&self, trace_id: Trace, db: &::salsa::Db) -> KiReprExpansion {
        trace_id
            .ki_repr(db)
            .expect("should be some")
            .expansion(db)
            .expect("should be some")
    }
}
