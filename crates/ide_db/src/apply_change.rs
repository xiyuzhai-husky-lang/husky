//! Applies changes to the IDE state transactionally.

use std::sync::Arc;

use base_db::{
    salsa::{Database, Durability},
    Change, SourceRootId,
};
use profile::{memory_usage, Bytes};
use rustc_hash::FxHashSet;

use crate::{symbol_index::SymbolsDatabase, IdeDatabase};

impl IdeDatabase {
    pub fn request_cancellation(&mut self) {
        let _p = profile::span("RootDatabase::request_cancellation");
        self.salsa_runtime_mut().synthetic_write(Durability::LOW);
    }

    pub fn apply_change(&mut self, change: Change) {
        let _p = profile::span("RootDatabase::apply_change");
        self.request_cancellation();
        tracing::info!("apply_change {:?}", change);
        if let Some(roots) = &change.roots {
            let mut local_roots = FxHashSet::default();
            let mut library_roots = FxHashSet::default();
            for (idx, root) in roots.iter().enumerate() {
                let root_id = SourceRootId(idx as u32);
                if root.is_library {
                    library_roots.insert(root_id);
                } else {
                    local_roots.insert(root_id);
                }
            }
            self.set_local_roots_with_durability(Arc::new(local_roots), Durability::HIGH);
            self.set_library_roots_with_durability(Arc::new(library_roots), Durability::HIGH);
        }
        change.apply(self);
    }
}
