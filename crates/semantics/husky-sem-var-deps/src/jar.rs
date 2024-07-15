#[salsa::jar]
pub struct SemVarDepsJar(
    crate::graph_dynamics::item_sem_var_deps_cycle_group_final_values,
    crate::graph_dynamics::item_sem_var_deps_initial_value,
    crate::region::SemStaticVarDepsRegion,
);
