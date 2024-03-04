use husky_entity_path::ItemPath;
use husky_linkage::linkage::Linkage;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VmirRegion {
    // Block(Vmirs),
}

fn item_instruction_region(_db: &::salsa::Db, _item_path: ItemPath) -> Option<VmirRegion> {
    todo!()
}

fn method_linkage_instruction_region(
    _db: &::salsa::Db,
    _member_route: Linkage,
) -> Option<VmirRegion> {
    todo!()
}

fn dataset_config_instruction_region(_db: &::salsa::Db, _target_entrance: ItemPath) -> VmirRegion {
    todo!()
}

fn enum_literal_to_i32(_db: &::salsa::Db, _item_path: ItemPath) -> i32 {
    todo!()
}
