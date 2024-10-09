use husky_item_path_interface::ItemPathIdInterface;
use husky_sem_expr::{SemExprIdx, SemExprMap, SemStmtIdx, SemStmtMap};
use husky_sem_var_deps::{
    region::ItemDefnSemVarDepsRegion,
    var_deps::{control_flow::SemControlFlowVarDeps, value::SemValueVarDeps, SemVarDep},
};
use husky_syn_expr::variable::VariableMap;
use vec_like::SmallVecSet;

#[salsa::tracked(constructor = new_inner)]
pub struct TraceVarDepsExpansion {
    #[return_ref]
    pub expr_control_flow_var_deps_table: SemExprMap<TraceVarDeps>,
    #[return_ref]
    pub stmt_control_flow_var_deps_table: SemStmtMap<TraceVarDeps>,
    #[return_ref]
    pub self_value_var_deps: TraceVarDeps,
    #[return_ref]
    pub variable_var_deps_table: VariableMap<TraceVarDeps>,
}

pub type TraceVarDeps = SmallVecSet<ItemPathIdInterface, 2>;

impl TraceVarDepsExpansion {
    pub(crate) fn new(region: ItemDefnSemVarDepsRegion, db: &::salsa::Db) -> Self {
        let f = |deps: &SemControlFlowVarDeps| {
            deps.iter()
                .map(|&dep| match dep {
                    SemVarDep::Item(item_path) => item_path.into(),
                })
                .collect()
        };
        let g = |deps: &SemValueVarDeps| {
            deps.iter()
                .map(|&dep| match dep {
                    SemVarDep::Item(item_path) => item_path.into(),
                })
                .collect()
        };
        Self::new_inner(
            db,
            region.expr_control_flow_var_deps_table(db).map(f),
            region.stmt_control_flow_var_deps_table(db).map(f),
            g(region.self_value_var_deps(db)),
            region.variable_var_deps_table(db).map(g),
        )
    }
}

impl TraceVarDepsExpansion {
    #[track_caller]
    pub fn expr_control_flow_var_deps(self, expr: SemExprIdx, db: &::salsa::Db) -> &TraceVarDeps {
        &self.expr_control_flow_var_deps_table(db)[expr]
    }

    pub fn stmt_control_flow_var_deps(self, stmt: SemStmtIdx, db: &::salsa::Db) -> &TraceVarDeps {
        &self.stmt_control_flow_var_deps_table(db)[stmt]
    }
}
