use husky_item_path_interface::ItemPathIdInterface;
use husky_sem_expr::{SemExprIdx, SemExprMap, SemStmtIdx, SemStmtMap};
use husky_sem_var_deps::{
    region::ItemDefnSemVarDepsRegion,
    var_deps::{SemVarDep, SemVarDeps},
};
use husky_syn_expr::variable::VariableMap;
use vec_like::SmallVecSet;

#[salsa::tracked(constructor = new_inner)]
pub struct TraceVarDepsExpansion {
    #[return_ref]
    pub expr_value_var_deps_table: SemExprMap<TraceVarDeps>,
    #[return_ref]
    pub stmt_value_var_deps_table: SemStmtMap<TraceVarDeps>,
    #[return_ref]
    pub self_value_var_deps: TraceVarDeps,
    #[return_ref]
    pub variable_var_deps_table: VariableMap<TraceVarDeps>,
}

pub type TraceVarDeps = SmallVecSet<ItemPathIdInterface, 2>;

impl TraceVarDepsExpansion {
    pub(crate) fn new(region: ItemDefnSemVarDepsRegion, db: &::salsa::Db) -> Self {
        let f = |deps: &SemVarDeps| {
            deps.iter()
                .map(|&dep| match dep {
                    SemVarDep::Item(item_path) => item_path.into(),
                })
                .collect()
        };
        Self::new_inner(
            db,
            region.expr_value_var_deps_table(db).map(f),
            region.stmt_value_var_deps_table(db).map(f),
            f(region.self_value_var_deps(db)),
            region.variable_var_deps_table(db).map(f),
        )
    }
}

impl TraceVarDepsExpansion {
    pub fn expr_value_var_deps(self, expr: SemExprIdx, db: &::salsa::Db) -> &TraceVarDeps {
        &self.expr_value_var_deps_table(db)[expr]
    }

    pub fn stmt_value_var_deps(self, stmt: SemStmtIdx, db: &::salsa::Db) -> &TraceVarDeps {
        &self.stmt_value_var_deps_table(db)[stmt]
    }
}
