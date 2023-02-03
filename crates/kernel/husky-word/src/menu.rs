use crate::*;

pub struct WordMenu {
    std: Identifier,
    core: Identifier,
    unit: Identifier,
    bool: Identifier,
    i32: Identifier,
    i64: Identifier,
    u32: Identifier,
    u64: Identifier,
    r32: Identifier,
    b64: Identifier,
    f32: Identifier,
    f64: Identifier,
    trai_ty: Identifier,
    lifetime_ty: Identifier,
    module: Identifier,
    crate_word: Identifier,
}

impl WordMenu {
    pub(crate) fn new(db: &dyn WordDb) -> Self {
        Self {
            core: db.it_ident_borrowed("core").unwrap(),
            std: db.it_ident_borrowed("std").unwrap(),
            unit: db.it_ident_borrowed("unit").unwrap(),
            bool: db.it_ident_borrowed("bool").unwrap(),
            i32: db.it_ident_borrowed("i32").unwrap(),
            i64: db.it_ident_borrowed("i64").unwrap(),
            u32: db.it_ident_borrowed("r32").unwrap(),
            u64: db.it_ident_borrowed("b64").unwrap(),
            r32: db.it_ident_borrowed("r32").unwrap(),
            b64: db.it_ident_borrowed("b64").unwrap(),
            f32: db.it_ident_borrowed("f32").unwrap(),
            f64: db.it_ident_borrowed("f64").unwrap(),
            trai_ty: db.it_ident_borrowed("Trait").unwrap(),
            module: db.it_ident_borrowed("Module").unwrap(),
            crate_word: db.it_ident_borrowed("crate").unwrap(),
            lifetime_ty: db.it_ident_borrowed("Lifetime").unwrap(),
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

    pub fn r32(&self) -> Identifier {
        self.r32
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

    pub fn unit(&self) -> Identifier {
        self.unit
    }

    pub fn bool(&self) -> Identifier {
        self.bool
    }

    pub fn u32(&self) -> Identifier {
        self.u32
    }

    pub fn u64(&self) -> Identifier {
        self.u64
    }

    pub fn trai_ty(&self) -> Identifier {
        self.trai_ty
    }

    pub fn module(&self) -> Identifier {
        self.module
    }

    pub fn crate_word(&self) -> Identifier {
        self.crate_word
    }

    pub fn lifetime_ty(&self) -> Identifier {
        self.lifetime_ty
    }
}
