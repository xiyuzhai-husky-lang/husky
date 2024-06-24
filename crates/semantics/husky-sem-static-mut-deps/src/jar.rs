#[salsa::jar]
pub struct SemStaticMutDepsJar(
    crate::region::SemStaticMutDepsRegion,
    crate::graph_dynamics::item_sem_static_mut_deps_initial_value,
    crate::graph_dynamics::item_sem_static_mut_deps_cycle_group_final_values,
);
