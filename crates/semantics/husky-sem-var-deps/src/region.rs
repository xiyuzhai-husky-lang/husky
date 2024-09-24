use crate::{
    builder::SemVarDepsBuilder,
    item_sem_var_deps,
    var_deps::{SemControlTransferVarDeps, SemVarDeps},
};
use husky_entity_path::{path::ItemPathId, region::RegionPath};
use husky_sem_expr::{SemExprMap, SemStmtMap};
use husky_syn_expr::variable::VariableMap;
use vec_like::OrderedSmallVecSet;

#[salsa::tracked]
pub struct ItemDefnSemVarDepsRegion {
    #[return_ref]
    pub expr_value_var_deps_table: SemExprMap<SemVarDeps>,
    #[return_ref]
    pub expr_control_flow_var_deps_table: SemExprMap<SemControlTransferVarDeps>,
    #[return_ref]
    pub stmt_value_var_deps_table: SemStmtMap<SemVarDeps>,
    #[return_ref]
    pub stmt_control_flow_var_deps_table: SemStmtMap<SemControlTransferVarDeps>,
    #[return_ref]
    pub self_value_var_deps: SemVarDeps,
    #[return_ref]
    pub variable_var_deps_table: VariableMap<SemVarDeps>,
}

#[salsa::tracked]
pub fn item_defn_sem_var_deps_region(
    db: &::salsa::Db,
    item_path_id: ItemPathId,
) -> Option<ItemDefnSemVarDepsRegion> {
    let item_path = item_path_id.item_path(db);
    let mut builder = SemVarDepsBuilder::new(db, RegionPath::ItemDefn(item_path), |item_path| {
        item_sem_var_deps(item_path, db)
    })?;
    builder.calc_root();
    Some(builder.finish())
}

impl ItemDefnSemVarDepsRegion {
    pub fn total_var_deps(self, db: &::salsa::Db) -> SemVarDeps {
        let mut var_deps = SemVarDeps::default();
        for (_, var_deps1) in self.expr_control_flow_var_deps_table(db) {
            var_deps.merge(var_deps1)
        }
        for (_, var_deps1) in self.expr_value_var_deps_table(db) {
            var_deps.merge(var_deps1)
        }
        for (_, var_deps1) in self.stmt_control_flow_var_deps_table(db) {
            var_deps.merge(var_deps1)
        }
        for (_, var_deps1) in self.stmt_value_var_deps_table(db) {
            var_deps.merge(var_deps1)
        }
        var_deps
    }
}
