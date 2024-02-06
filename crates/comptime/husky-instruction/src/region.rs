use crate::*;

#[derive(Debug)]
pub struct InstructionRegion {
    arena: InstructionArena,
}

fn item_instruction_sheet(_db: &::salsa::Db, _item_path: ItemPath) -> Option<InstructionRegion> {
    todo!()
}

fn method_opt_instruction_sheet(
    _db: &::salsa::Db,
    _member_route: EthTerm,
) -> Option<InstructionRegion> {
    todo!()
}

fn dataset_config_instruction_sheet(
    _db: &::salsa::Db,
    _target_entrance: ItemPath,
) -> InstructionRegion {
    todo!()
}

fn enum_literal_to_i32(_db: &::salsa::Db, _item_path: ItemPath) -> i32 {
    todo!()
}
