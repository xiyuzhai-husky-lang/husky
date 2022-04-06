use crate::*;

pub trait InterpreterQueryGroup {
    fn entity_instruction_sheet_by_uid(&self, uid: EntityUid) -> Arc<InstructionSheet>;
}
