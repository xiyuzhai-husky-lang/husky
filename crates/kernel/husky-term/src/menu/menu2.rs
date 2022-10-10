use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu2 {
    parent: TermMenu1,
    void: Ty,
    i32: Ty,
    i64: Ty,
    f32: Ty,
    f64: Ty,
    b32: Ty,
    b64: Ty,
    bool: Ty,
}

impl std::ops::Deref for TermMenu2 {
    type Target = TermMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu2 {
    pub(crate) fn new(db: &dyn TermQuery, menu1: TermMenu1) -> Self {
        let void = Ty::void(db, &menu1);
        let i32 = Ty::i32(db, &menu1);
        let i64 = Ty::i64(db, &menu1);
        let f32 = Ty::f32(db, &menu1);
        let f64 = Ty::f64(db, &menu1);
        let b32 = Ty::b32(db, &menu1);
        let b64 = Ty::b64(db, &menu1);
        let bool = Ty::bool(db, &menu1);
        TermMenu2 {
            parent: menu1,
            void,
            i32,
            i64,
            f32,
            f64,
            b32,
            b64,
            bool,
        }
    }

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
