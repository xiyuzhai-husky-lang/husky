#[salsa::jar]
pub struct SemStaticVarDepsJar(
    crate::graph_dynamics::item_sem_static_var_deps_cycle_group_final_values,
    crate::graph_dynamics::item_sem_static_var_deps_initial_value,
    crate::region::SemStaticVarDepsRegion,
);
