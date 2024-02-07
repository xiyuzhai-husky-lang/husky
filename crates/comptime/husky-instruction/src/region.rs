use crate::*;
use husky_entity_path::ItemPath;
use husky_linkage::linkage::Linkage;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InstructionRegion {
    // Block(Instructions),
}

fn item_instruction_region(_db: &::salsa::Db, _item_path: ItemPath) -> Option<InstructionRegion> {
    todo!()
}

fn method_linkage_instruction_region(
    _db: &::salsa::Db,
    _member_route: Linkage,
) -> Option<InstructionRegion> {
    todo!()
}

fn dataset_config_instruction_region(
    _db: &::salsa::Db,
    _target_entrance: ItemPath,
) -> InstructionRegion {
    todo!()
}

fn enum_literal_to_i32(_db: &::salsa::Db, _item_path: ItemPath) -> i32 {
    todo!()
}
