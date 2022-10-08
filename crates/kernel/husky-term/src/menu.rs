use std::sync::Arc;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu {
    void: Ty,
    i32: Ty,
    i64: Ty,
    f32: Ty,
    f64: Ty,
    b32: Ty,
    b64: Ty,
    bool: Ty,
    i32_literal_0: TermPtr,
    i32_literal_1: TermPtr,
    i64_literal_0: TermPtr,
    i64_literal_1: TermPtr,
}

impl TermMenu {
    pub fn i32(&self) -> Ty {
        self.i32
    }
    pub fn void(&self) -> Ty {
        self.void
    }
    pub fn i64(&self) -> Ty {
        self.i64
    }
    pub fn f32(&self) -> Ty {
        self.f32
    }
    pub fn f64(&self) -> Ty {
        self.f64
    }
    pub fn b32(&self) -> Ty {
        self.b32
    }
    pub fn b64(&self) -> Ty {
        self.b64
    }
    pub fn bool(&self) -> Ty {
        self.bool
    }
}

pub(crate) fn term_menu(db: &dyn TermQuery) -> Arc<TermMenu> {
    let void = Ty::void(db);
    let i32 = Ty::i32(db);
    let i64 = Ty::i64(db);
    let f32 = Ty::f32(db);
    let f64 = Ty::f64(db);
    let b32 = Ty::b32(db);
    let b64 = Ty::b64(db);
    let bool = Ty::bool(db);
    Arc::new(TermMenu {
        void,
        i32,
        i64,
        f32,
        f64,
        b32,
        b64,
        bool,
        i32_literal_0: TermLiteral::i32_literal(db, 0, i32),
        i32_literal_1: TermLiteral::i32_literal(db, 1, i32),
        i64_literal_0: TermLiteral::i64_literal(db, 0, i64),
        i64_literal_1: TermLiteral::i64_literal(db, 1, i64),
    })
}
