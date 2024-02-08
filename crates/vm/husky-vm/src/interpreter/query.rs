use crate::*;

pub trait InterpreterQueryGroup {
    fn item_opt_instruction_region_by_uid(&self, uid: EntityUid) -> Option<Vmirs>;
}

// impl InterpreterQueryGroup for husky-compilerompileTime {
//     fn item_opt_instruction_region_by_uid(
//         &self,
//         uid: husky_vm::EntityUid,
//     ) -> Option<Arc<husky_vm::Vmirs>> {
//         let item_path = self.item_route_by_uid(uid);
//         self.item_instruction_region(item_path)
//     }
// }
