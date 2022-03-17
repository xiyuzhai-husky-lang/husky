mod entry;

use crate::*;
use common::*;
pub use entry::HistoryEntry;

#[derive(Debug, Default, Clone)]
pub struct History<'eval> {
    entries: HashMap<InstructionId, HistoryEntry<'eval>>,
}

impl<'eval> History<'eval> {
    pub fn entry<T: InstructionSource>(&self, t: &T) -> HistoryEntry<'eval> {
        self.entries.get(&t.instruction_id()).unwrap().clone()
    }

    pub fn write(&mut self, ins: &Instruction, entry: HistoryEntry<'eval>) {
        should!(self.entries.insert(ins.id(), entry).is_none());
    }
}
