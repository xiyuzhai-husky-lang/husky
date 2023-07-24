use crate::*;

pub trait InterpreterQueryGroup {
    fn item_opt_instruction_sheet_by_uid(&self, uid: EntityUid) -> Option<Arc<InstructionSheet>>;
}

// impl InterpreterQueryGroup for husky-compilerompileTime {
//     fn item_opt_instruction_sheet_by_uid(
//         &self,
//         uid: husky_vm::EntityUid,
//     ) -> Option<Arc<husky_vm::InstructionSheet>> {
//         let item_path = self.item_route_by_uid(uid);
//         self.item_instruction_sheet(item_path)
//     }
// }
