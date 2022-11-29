use crate::*;

pub struct WordMenu {
    std: Word,
    core: Word,
    i32: Word,
    i64: Word,
    b32: Word,
    b64: Word,
    f32: Word,
    f64: Word,
}

impl WordMenu {
    pub(crate) fn new(db: &dyn WordDb) -> Self {
        Self {
            core: db.it_word_borrowed("core"),
            std: db.it_word_borrowed("std"),
            i32: db.it_word_borrowed("i32"),
            i64: db.it_word_borrowed("i64"),
            b32: db.it_word_borrowed("b32"),
            b64: db.it_word_borrowed("b64"),
            f32: db.it_word_borrowed("f32"),
            f64: db.it_word_borrowed("f64"),
        }
    }

    pub fn core(&self) -> Word {
        self.core
    }

    pub fn std(&self) -> Word {
        self.std
    }

    pub fn i32(&self) -> Word {
        self.i32
    }

    pub fn i64(&self) -> Word {
        self.i64
    }

    pub fn b32(&self) -> Word {
        self.b32
    }

    pub fn b64(&self) -> Word {
        self.b64
    }

    pub fn f32(&self) -> Word {
        self.f32
    }

    pub fn f64(&self) -> Word {
        self.f64
    }
}
