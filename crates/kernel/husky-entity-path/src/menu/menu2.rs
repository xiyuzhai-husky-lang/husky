use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu2 {
    core_marker_sized: EntityPath,
    core_num_i32: EntityPath,
    core_num_i64: EntityPath,
    core_num_b32: EntityPath,
    core_num_b64: EntityPath,
    core_num_f32: EntityPath,
    core_num_f64: EntityPath,
    parent: EntityPathMenu1,
}

impl EntityPathMenu2 {
    pub(crate) fn new(db: &dyn EntityPathDb, menu1: EntityPathMenu1) -> Self {
        Self {
            core_marker_sized: menu1.core_marker().child(db, "Sized"),
            core_num_i32: menu1.core_num().child(db, "i32"),
            core_num_i64: menu1.core_num().child(db, "i64"),
            core_num_b32: menu1.core_num().child(db, "b32"),
            core_num_b64: menu1.core_num().child(db, "b64"),
            core_num_f32: menu1.core_num().child(db, "f32"),
            core_num_f64: menu1.core_num().child(db, "f64"),
            parent: menu1,
        }
    }

    pub fn i32(&self) -> EntityPath {
        self.core_num_i32
    }

    pub fn i64(&self) -> EntityPath {
        self.core_num_i64
    }

    pub fn b32(&self) -> EntityPath {
        self.core_num_b32
    }

    pub fn b64(&self) -> EntityPath {
        self.core_num_b64
    }

    pub fn f32(&self) -> EntityPath {
        self.core_num_f32
    }

    pub fn f64(&self) -> EntityPath {
        self.core_num_f64
    }
}

impl std::ops::Deref for EntityPathMenu2 {
    type Target = EntityPathMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
