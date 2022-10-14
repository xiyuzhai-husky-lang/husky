use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu1 {
    parent: EntityPathMenu0,
    // modules
    core_marker: EntityPathPtr,
    // primitive types
    core_i32: EntityPathPtr,
    core_i64: EntityPathPtr,
    core_f32: EntityPathPtr,
    core_f64: EntityPathPtr,
    core_b32: EntityPathPtr,
    core_b64: EntityPathPtr,
    core_bool: EntityPathPtr,
    // primitive concepts
    core_trai: EntityPathPtr,
    core_ty: EntityPathPtr,
    core_prop: EntityPathPtr,
    core_module: EntityPathPtr,
}

impl EntityPathMenu1 {
    pub(crate) fn new(db: &dyn EntityPathDb, menu0: &EntityPathMenu0) -> Self {
        todo!()
    }
}

impl std::ops::Deref for EntityPathMenu1 {
    type Target = EntityPathMenu0;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
