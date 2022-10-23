use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu0 {
    // library
    core: EntityPathItd,
    std: EntityPathItd,
    i32: EntityPathItd,
    i64: EntityPathItd,
    b32: EntityPathItd,
    b64: EntityPathItd,
    f32: EntityPathItd,
    f64: EntityPathItd,
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

    pub fn core(&self) -> EntityPathItd {
        self.core
    }

    pub fn std(&self) -> EntityPathItd {
        self.std
    }

    pub fn i32(&self) -> EntityPathItd {
        self.i32
    }

    pub fn i64(&self) -> EntityPathItd {
        self.i64
    }

    pub fn b32(&self) -> EntityPathItd {
        self.b32
    }

    pub fn b64(&self) -> EntityPathItd {
        self.b64
    }

    pub fn f32(&self) -> EntityPathItd {
        self.f32
    }

    pub fn f64(&self) -> EntityPathItd {
        self.f64
    }
}
