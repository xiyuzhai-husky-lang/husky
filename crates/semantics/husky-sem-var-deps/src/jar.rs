#[salsa::jar]
pub struct SemVarDepsJar(
    crate::graph_dynamics::item_sem_var_deps_cycle_group_final_values,
    crate::graph_dynamics::item_sem_var_deps_initial_value,
    crate::region::ItemDefnSemVarDepsRegion,
    crate::region::item_defn_sem_var_deps_region,
    crate::helpers::history::item_history_sem_var_deps,
);
