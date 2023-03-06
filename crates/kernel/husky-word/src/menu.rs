use crate::*;

pub struct WordMenu {
    std: Identifier,
    core: Identifier,
    unit: Identifier,
    never: Identifier,
    bool: Identifier,
    i8: Identifier,
    i16: Identifier,
    i32: Identifier,
    i64: Identifier,
    i128: Identifier,
    isize: Identifier,
    u8: Identifier,
    u16: Identifier,
    u32: Identifier,
    u64: Identifier,
    u128: Identifier,
    usize: Identifier,
    r8: Identifier,
    r16: Identifier,
    r32: Identifier,
    r64: Identifier,
    r128: Identifier,
    rsize: Identifier,
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
            never: db.it_ident_borrowed("never").unwrap(),
            bool: db.it_ident_borrowed("bool").unwrap(),
            i8: db.it_ident_borrowed("i8").unwrap(),
            i16: db.it_ident_borrowed("i16").unwrap(),
            i32: db.it_ident_borrowed("i32").unwrap(),
            i64: db.it_ident_borrowed("i64").unwrap(),
            i128: db.it_ident_borrowed("i128").unwrap(),
            isize: db.it_ident_borrowed("isize").unwrap(),
            u8: db.it_ident_borrowed("u8").unwrap(),
            u16: db.it_ident_borrowed("u16").unwrap(),
            u32: db.it_ident_borrowed("u32").unwrap(),
            u64: db.it_ident_borrowed("u64").unwrap(),
            u128: db.it_ident_borrowed("u128").unwrap(),
            usize: db.it_ident_borrowed("usize").unwrap(),
            r8: db.it_ident_borrowed("r8").unwrap(),
            r16: db.it_ident_borrowed("r16").unwrap(),
            r32: db.it_ident_borrowed("r32").unwrap(),
            r64: db.it_ident_borrowed("r64").unwrap(),
            r128: db.it_ident_borrowed("r128").unwrap(),
            rsize: db.it_ident_borrowed("rsize").unwrap(),
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

    pub fn i8(&self) -> Identifier {
        self.i8
    }

    pub fn i16(&self) -> Identifier {
        self.i16
    }

    pub fn i32(&self) -> Identifier {
        self.i32
    }

    pub fn i64(&self) -> Identifier {
        self.i64
    }

    pub fn i128(&self) -> Identifier {
        self.i128
    }

    pub fn isize(&self) -> Identifier {
        self.isize
    }

    pub fn r8(&self) -> Identifier {
        self.r8
    }

    pub fn r16(&self) -> Identifier {
        self.r16
    }

    pub fn r32(&self) -> Identifier {
        self.r32
    }

    pub fn r64(&self) -> Identifier {
        self.r64
    }

    pub fn r128(&self) -> Identifier {
        self.r128
    }

    pub fn rsize(&self) -> Identifier {
        self.rsize
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

    pub fn u8(&self) -> Identifier {
        self.u8
    }

    pub fn u16(&self) -> Identifier {
        self.u16
    }

    pub fn u32(&self) -> Identifier {
        self.u32
    }

    pub fn u64(&self) -> Identifier {
        self.u64
    }

    pub fn u128(&self) -> Identifier {
        self.u128
    }

    pub fn usize(&self) -> Identifier {
        self.usize
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

    pub fn never(&self) -> Identifier {
        self.never
    }
}
