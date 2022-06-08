use crate::*;
use entity_route::EntityRoutePtr;
use husky_debugger_gui::protocol::VisualProps;

pub trait InterpreterQueryGroup {
    fn entity_opt_instruction_sheet_by_uid(&self, uid: EntityUid) -> Option<Arc<InstructionSheet>>;
    fn visualize<'temp, 'eval>(
        &self,
        ty: EntityRoutePtr,
        value: &(dyn AnyValueDyn<'eval> + 'temp),
    ) -> VisualProps;
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
