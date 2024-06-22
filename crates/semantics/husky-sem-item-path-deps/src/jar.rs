#[salsa::jar]
pub struct SemItemPathDepsJar(
    crate::item_path_deps::item_item_path_deps,
    crate::item_path_deps::item_item_path_deps_cropped,
    crate::helpers::cycle_group::SemItemPathCyclceGroupItd,
);
