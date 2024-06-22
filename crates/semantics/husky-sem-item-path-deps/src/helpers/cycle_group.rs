use graph_dynamics::{cycle_group::CycleGroup, deps::IsGraphDepsScheme};
use husky_entity_path::path::ItemPathId;

#[salsa::interned]
pub struct SemItemPathCyclceGroupItd {
    pub cycle_group: CycleGroup<SemItemPathGraphDepsScheme>,
}

pub struct SemItemPathGraphDepsScheme;

impl IsGraphDepsScheme for SemItemPathGraphDepsScheme {
    type Node = ItemPathId;

    const CYCLE_GROUP_N: usize = 4;

    type CycleGroupItd = SemItemPathCyclceGroupItd;
}
