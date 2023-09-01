mod entry;

use std::collections::HashMap;

use crate::*;

pub use entry::HistoryEntry;
use husky_print_utils::p;

#[derive(Debug, Default)]
pub struct History {
    entries: HashMap<InstructionId, HistoryEntry>,
}

impl History {
    pub fn write(&mut self, ins: &Instruction, entry: HistoryEntry) {
        if let Some(old_value) = self.entries.insert(ins.ins_id(), entry) {
            todo!()
            // p!(ins.src.source(), ins.src.text_range(), ins.src);
            // p!(old_value);
            // panic!()
        }
    }
}

impl History {
    pub fn register_result<T: Into<InstructionSource>>(
        &self,
        t: &T,
    ) -> Option<VMResult<RegularValue>> {
        todo!()
        // self.entries
        //     .get(&t.instruction_id())
        //     .map(|entry| entry.result())
    }

    pub fn get<T: Into<InstructionSource>>(&self, t: T) -> Option<&HistoryEntry> {
        todo!()
        // self.entries.get(&t.instruction_id())
    }

    pub fn contains<T: Into<InstructionSource>>(&self, t: T) -> bool {
        todo!()
        // self.entries.contains_key(&t.instruction_id())
    }
}
