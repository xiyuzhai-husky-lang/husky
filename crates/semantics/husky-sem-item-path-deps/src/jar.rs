#[salsa::jar]
pub struct SemItemPathDepsJar(
    crate::item_path_deps::item_sem_item_path_deps,
    crate::item_path_deps::item_sem_item_path_deps_cropped,
    crate::helpers::graph_dynamics::SemItemPathDepsCyclceGroupItd,
    crate::helpers::graph_dynamics::item_sem_item_path_full_deps_cropped,
    crate::helpers::graph_dynamics::item_sem_item_path_cycle_group_itd,
);
