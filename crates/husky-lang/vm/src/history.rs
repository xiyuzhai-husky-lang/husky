mod entry;

use crate::*;
use common::*;
pub use entry::HistoryEntry;

#[derive(Debug, Default, Clone)]
pub struct History {
    entries: HashMap<InstructionId, HistoryEntry>,
}

impl History {
    pub fn entry<T: InstructionSource>(&self, t: &T) -> HistoryEntry {
        self.entries.get(&t.instruction_id()).unwrap().clone()
    }

    pub fn write(&mut self, id: InstructionId, entry: HistoryEntry) {
        should!(self.entries.insert(id, entry).is_none());
    }
}
