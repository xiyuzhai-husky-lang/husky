use crate::*;
use husky_entity_route::EntityRoutePtr;

pub trait InterpreterQueryGroup {
    fn entity_opt_instruction_sheet_by_uid(&self, uid: EntityUid) -> Option<Arc<InstructionSheet>>;
}

// impl InterpreterQueryGroup for HuskyCompileTime {
//     fn entity_opt_instruction_sheet_by_uid(
//         &self,
//         uid: vm::EntityUid,
//     ) -> Option<Arc<vm::InstructionSheet>> {
//         let entity_route = self.entity_route_by_uid(uid);
//         self.entity_instruction_sheet(entity_route)
//     }
// }
