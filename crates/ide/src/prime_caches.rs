//! husky-lang-server is lazy and doesn't compute anything unless asked. This
//! sometimes is counter productive when, for example, the first goto definition
//! request takes longer to compute. This modules implemented prepopulation of
//! various caches, it's not really advanced at the moment.

use hir::db::DefDatabase;
use ide_db::base_db::{CrateId, SourceDatabase, SourceDatabaseExt};
use rustc_hash::FxHashSet;

use crate::IdeDatabase;

/// We started indexing a crate.
#[derive(Debug)]
pub struct PrimeCachesProgress {
    pub on_crate: String,
    pub n_done: usize,
    pub n_total: usize,
}

pub(crate) fn prime_caches(db: &IdeDatabase, cb: &(dyn Fn(PrimeCachesProgress) + Sync)) {
    todo!()
}
