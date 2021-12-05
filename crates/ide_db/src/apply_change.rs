//! Applies changes to the IDE state transactionally.

use std::sync::Arc;

use base_db::{
    salsa::{Database, Durability},
    Change, SourceRootId,
};
use profile::{memory_usage, Bytes};
use rustc_hash::FxHashSet;

use crate::{symbol_index::SymbolsDatabase, RootDatabase};

impl RootDatabase {
    pub fn request_cancellation(&mut self) {
        let _p = profile::span("RootDatabase::request_cancellation");
        self.salsa_runtime_mut().synthetic_write(Durability::LOW);
    }

    pub fn apply_change(&mut self, change: Change) {
        todo!()
    }
}
