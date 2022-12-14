use crate::*;

pub struct WordMenu {
    std: Identifier,
    core: Identifier,
    i32: Identifier,
    i64: Identifier,
    b32: Identifier,
    b64: Identifier,
    f32: Identifier,
    f64: Identifier,
}

impl WordMenu {
    pub(crate) fn new(db: &dyn WordDb) -> Self {
        Self {
            core: db.it_ident_borrowed("core").unwrap(),
            std: db.it_ident_borrowed("std").unwrap(),
            i32: db.it_ident_borrowed("i32").unwrap(),
            i64: db.it_ident_borrowed("i64").unwrap(),
            b32: db.it_ident_borrowed("b32").unwrap(),
            b64: db.it_ident_borrowed("b64").unwrap(),
            f32: db.it_ident_borrowed("f32").unwrap(),
            f64: db.it_ident_borrowed("f64").unwrap(),
        }
    }

    pub fn core(&self) -> Identifier {
        self.core
    }

    pub fn std(&self) -> Identifier {
        self.std
    }

    pub fn i32(&self) -> Identifier {
        self.i32
    }

    pub fn i64(&self) -> Identifier {
        self.i64
    }

    pub fn b32(&self) -> Identifier {
        self.b32
    }

    pub fn b64(&self) -> Identifier {
        self.b64
    }

    pub fn f32(&self) -> Identifier {
        self.f32
    }

    pub fn f64(&self) -> Identifier {
        self.f64
    }
}
