use super::*;
use crate::registry::assoc_trace::VoidAssocTraceRegistry;
use husky_hir_defn::defn::HasHirDefn;
use husky_sem_expr::{helpers::analysis::sem_expr_region_requires_lazy, SemExprData, SemExprDb};
use husky_syn_defn::{item_syn_defn, ItemSynDefn};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaticVarTracePathData {
    // todo: more general paths
    static_var_item_path: MajorFormPath,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StaticVarTraceData {
    path: TracePath,
    // todo: more general paths
    static_var_item_path: MajorFormPath,
}

impl StaticVarTraceData {
    pub(super) fn ki_repr(&self, db: &::salsa::Db) -> KiRepr {
        KiRepr::new_static_var_item(self.static_var_item_path, db)
    }

    pub(super) fn view_lines(&self, db: &::salsa::Db) -> TraceViewLines {
        use husky_entity_tree::node::HasSynNodePath;
        let static_var_item_path = self.static_var_item_path;
        let token_idx_range = static_var_item_path
            .syn_node_path(db)
            .decl_tokra_region_token_idx_range(db);
        TraceViewLines::new(
            static_var_item_path.module_path(db),
            token_idx_range,
            VoidAssocTraceRegistry,
            db,
        )
    }

    #[inline(always)]
    pub(super) fn have_subtraces(self, db: &::salsa::Db) -> bool {
        false
    }

    pub(super) fn subtraces(&self, trace: Trace, db: &::salsa::Db) -> Vec<Trace> {
        vec![]
    }

    pub(super) fn var_deps(&self, db: &::salsa::Db) -> Vec<ItemPathIdInterface> {
        todo!()
    }
}

impl Trace {
    pub fn from_major_static_var_form_path(
        static_var_item_path: MajorFormPath,
        db: &::salsa::Db,
    ) -> Self {
        debug_assert_eq!(static_var_item_path.kind(db), MajorFormKind::StaticVar);
        let path = TracePath::new(
            StaticVarTracePathData {
                static_var_item_path,
            },
            db,
        );
        Trace::new(
            path,
            StaticVarTraceData {
                path,
                static_var_item_path,
            }
            .into(),
            db,
        )
    }
}
