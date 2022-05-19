use crate::*;
use entity_route::EntityRoutePtr;

pub trait InterpreterQueryGroup {
    fn entity_opt_instruction_sheet_by_uid(&self, uid: EntityUid) -> Option<Arc<InstructionSheet>>;
}
