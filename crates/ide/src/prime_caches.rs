//! rust-analyzer is lazy and doesn't compute anything unless asked. This
//! sometimes is counter productive when, for example, the first goto definition
//! request takes longer to compute. This modules implemented prepopulation of
//! various caches, it's not really advanced at the moment.

use hir::db::DefDatabase;
use ide_db::base_db::{CrateGraph, CrateId, SourceDatabase, SourceDatabaseExt};
use rustc_hash::FxHashSet;

use crate::RootDatabase;

/// We started indexing a crate.
#[derive(Debug)]
pub struct PrimeCachesProgress {
    pub on_crate: String,
    pub n_done: usize,
    pub n_total: usize,
}

pub(crate) fn prime_caches(db: &RootDatabase, cb: &(dyn Fn(PrimeCachesProgress) + Sync)) {
    todo!()
}

fn toposort(graph: &CrateGraph, crates: &FxHashSet<CrateId>) -> Vec<CrateId> {
    // Just subset the full topologically sorted set for simplicity.

    let all = graph.crates_in_topological_order();
    let mut result = Vec::with_capacity(crates.len());
    for krate in all {
        if crates.contains(&krate) {
            result.push(krate);
        }
    }
    result
}
