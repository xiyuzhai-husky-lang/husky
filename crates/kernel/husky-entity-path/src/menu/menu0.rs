use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu0 {
    // library
    core: EntityPath,
    std: EntityPath,
    i32: EntityPath,
    i64: EntityPath,
    b32: EntityPath,
    b64: EntityPath,
    f32: EntityPath,
    f64: EntityPath,
}

impl EntityPathMenu0 {
    pub(crate) fn new(db: &dyn EntityPathDb) -> Self {
        Self {
            core: db.it_root_entity_path("core"),
            std: db.it_root_entity_path("std"),
            i32: db.it_root_entity_path("i32"),
            i64: db.it_root_entity_path("i64"),
            b32: db.it_root_entity_path("b32"),
            b64: db.it_root_entity_path("b64"),
            f32: db.it_root_entity_path("f32"),
            f64: db.it_root_entity_path("f64"),
        }
    }

    pub fn core(&self) -> EntityPath {
        self.core
    }

    pub fn std(&self) -> EntityPath {
        self.std
    }

    pub fn i32(&self) -> EntityPath {
        self.i32
    }

    pub fn i64(&self) -> EntityPath {
        self.i64
    }

    pub fn b32(&self) -> EntityPath {
        self.b32
    }

    pub fn b64(&self) -> EntityPath {
        self.b64
    }

    pub fn f32(&self) -> EntityPath {
        self.f32
    }

    pub fn f64(&self) -> EntityPath {
        self.f64
    }
}
