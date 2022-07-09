mod entry;

use std::collections::HashMap;

use crate::*;

use check_utils::should;
pub use entry::HistoryEntry;
use print_utils::p;

#[derive(Debug, Default)]
pub struct History<'eval> {
    entries: HashMap<InstructionId, HistoryEntry<'eval>>,
}

impl<'eval> History<'eval> {
    pub fn write(&mut self, ins: &Instruction, entry: HistoryEntry<'eval>) {
        if let Some(old_value) = self.entries.insert(ins.ins_id(), entry) {
            p!(ins.src.file(), ins.src.text_range());
            p!(old_value);
            panic!()
        }
    }
}

impl History<'static> {
    pub fn value_result<T: InstructionSource>(&self, t: &T) -> __EvalResult {
        if let Some(entry) = self.entries.get(&t.instruction_id()) {
            entry.result()
        } else {
            Ok(__EvalValue::Undefined)
        }
    }

    pub fn get<T: InstructionSource>(&self, t: &T) -> Option<&HistoryEntry<'static>> {
        self.entries.get(&t.instruction_id())
    }

    pub fn contains<T: InstructionSource>(&self, t: &T) -> bool {
        self.entries.contains_key(&t.instruction_id())
    }
}
