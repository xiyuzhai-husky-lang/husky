use crate::*;

pub struct IdentMenu {
    std: Ident,
    core: Ident,
    unit: Ident,
    never: Ident,
    bool: Ident,
    i8: Ident,
    i16: Ident,
    i32: Ident,
    i64: Ident,
    i128: Ident,
    isize: Ident,
    u8: Ident,
    u16: Ident,
    u32: Ident,
    u64: Ident,
    u128: Ident,
    usize: Ident,
    r8: Ident,
    r16: Ident,
    r32: Ident,
    r64: Ident,
    r128: Ident,
    rsize: Ident,
    f32: Ident,
    f64: Ident,
    trai_ty: Ident,
    lifetime_ty: Ident,
    module: Ident,
    crate_word: Ident,
}

impl IdentMenu {
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

    pub fn core(&self) -> Ident {
        self.core
    }

    pub fn std(&self) -> Ident {
        self.std
    }

    pub fn i8(&self) -> Ident {
        self.i8
    }

    pub fn i16(&self) -> Ident {
        self.i16
    }

    pub fn i32(&self) -> Ident {
        self.i32
    }

    pub fn i64(&self) -> Ident {
        self.i64
    }

    pub fn i128(&self) -> Ident {
        self.i128
    }

    pub fn isize(&self) -> Ident {
        self.isize
    }

    pub fn r8(&self) -> Ident {
        self.r8
    }

    pub fn r16(&self) -> Ident {
        self.r16
    }

    pub fn r32(&self) -> Ident {
        self.r32
    }

    pub fn r64(&self) -> Ident {
        self.r64
    }

    pub fn r128(&self) -> Ident {
        self.r128
    }

    pub fn rsize(&self) -> Ident {
        self.rsize
    }

    pub fn f32(&self) -> Ident {
        self.f32
    }

    pub fn f64(&self) -> Ident {
        self.f64
    }

    pub fn unit(&self) -> Ident {
        self.unit
    }

    pub fn bool(&self) -> Ident {
        self.bool
    }

    pub fn u8(&self) -> Ident {
        self.u8
    }

    pub fn u16(&self) -> Ident {
        self.u16
    }

    pub fn u32(&self) -> Ident {
        self.u32
    }

    pub fn u64(&self) -> Ident {
        self.u64
    }

    pub fn u128(&self) -> Ident {
        self.u128
    }

    pub fn usize(&self) -> Ident {
        self.usize
    }

    pub fn trai_ty(&self) -> Ident {
        self.trai_ty
    }

    pub fn module(&self) -> Ident {
        self.module
    }

    pub fn crate_word(&self) -> Ident {
        self.crate_word
    }

    pub fn lifetime_ty(&self) -> Ident {
        self.lifetime_ty
    }

    pub fn never(&self) -> Ident {
        self.never
    }
}
