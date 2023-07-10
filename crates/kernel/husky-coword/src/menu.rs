use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CowordMenu {
    std_name: Name,
    std_ident: Ident,
    core_name: Name,
    core_ident: Ident,
    unit_ident: Ident,
    never_ident: Ident,
    bool_ident: Ident,
    derive_ident: Ident,
    i8_ident: Ident,
    i16_ident: Ident,
    i32_ident: Ident,
    i64_ident: Ident,
    i128_ident: Ident,
    isize_ident: Ident,
    u8_ident: Ident,
    u16_ident: Ident,
    u32_ident: Ident,
    u64_ident: Ident,
    u128_ident: Ident,
    usize_ident: Ident,
    r8_ident: Ident,
    r16_ident: Ident,
    r32_ident: Ident,
    r64_ident: Ident,
    r128_ident: Ident,
    rsize_ident: Ident,
    f32_ident: Ident,
    f64_ident: Ident,
    trai_ty_ident: Ident,
    lifetime_ty_ident: Ident,
    module_ident: Ident,
    crate_ident: Ident,
}

impl CowordMenu {
    pub(crate) fn new(db: &dyn CowordDb) -> Self {
        Self {
            core_name: Name::from_ref(db, "core").unwrap(),
            core_ident: Ident::from_borrowed(db, "core").unwrap(),
            std_name: Name::from_ref(db, "std").unwrap(),
            std_ident: Ident::from_borrowed(db, "std").unwrap(),
            unit_ident: db.it_ident_borrowed("unit").unwrap(),
            never_ident: db.it_ident_borrowed("never").unwrap(),
            bool_ident: db.it_ident_borrowed("bool").unwrap(),
            derive_ident: db.it_ident_borrowed("derive").unwrap(),
            i8_ident: db.it_ident_borrowed("i8").unwrap(),
            i16_ident: db.it_ident_borrowed("i16").unwrap(),
            i32_ident: db.it_ident_borrowed("i32").unwrap(),
            i64_ident: db.it_ident_borrowed("i64").unwrap(),
            i128_ident: db.it_ident_borrowed("i128").unwrap(),
            isize_ident: db.it_ident_borrowed("isize").unwrap(),
            u8_ident: db.it_ident_borrowed("u8").unwrap(),
            u16_ident: db.it_ident_borrowed("u16").unwrap(),
            u32_ident: db.it_ident_borrowed("u32").unwrap(),
            u64_ident: db.it_ident_borrowed("u64").unwrap(),
            u128_ident: db.it_ident_borrowed("u128").unwrap(),
            usize_ident: db.it_ident_borrowed("usize").unwrap(),
            r8_ident: db.it_ident_borrowed("r8").unwrap(),
            r16_ident: db.it_ident_borrowed("r16").unwrap(),
            r32_ident: db.it_ident_borrowed("r32").unwrap(),
            r64_ident: db.it_ident_borrowed("r64").unwrap(),
            r128_ident: db.it_ident_borrowed("r128").unwrap(),
            rsize_ident: db.it_ident_borrowed("rsize").unwrap(),
            f32_ident: db.it_ident_borrowed("f32").unwrap(),
            f64_ident: db.it_ident_borrowed("f64").unwrap(),
            trai_ty_ident: db.it_ident_borrowed("Trait").unwrap(),
            module_ident: db.it_ident_borrowed("Module").unwrap(),
            crate_ident: db.it_ident_borrowed("crate").unwrap(),
            lifetime_ty_ident: db.it_ident_borrowed("Lifetime").unwrap(),
        }
    }

    pub fn core_name(&self) -> Name {
        self.core_name
    }

    pub fn core_ident(&self) -> Ident {
        self.core_ident
    }

    pub fn std_name(&self) -> Name {
        self.std_name
    }

    pub fn std_ident(&self) -> Ident {
        self.std_ident
    }

    pub fn i8_ident(&self) -> Ident {
        self.i8_ident
    }

    pub fn i16_ident(&self) -> Ident {
        self.i16_ident
    }

    pub fn i32_ident(&self) -> Ident {
        self.i32_ident
    }

    pub fn i64_ident(&self) -> Ident {
        self.i64_ident
    }

    pub fn i128_ident(&self) -> Ident {
        self.i128_ident
    }

    pub fn isize_ident(&self) -> Ident {
        self.isize_ident
    }

    pub fn r8_ident(&self) -> Ident {
        self.r8_ident
    }

    pub fn r16_ident(&self) -> Ident {
        self.r16_ident
    }

    pub fn r32_ident(&self) -> Ident {
        self.r32_ident
    }

    pub fn r64_ident(&self) -> Ident {
        self.r64_ident
    }

    pub fn r128_ident(&self) -> Ident {
        self.r128_ident
    }

    pub fn rsize_ident(&self) -> Ident {
        self.rsize_ident
    }

    pub fn f32_ident(&self) -> Ident {
        self.f32_ident
    }

    pub fn f64_ident(&self) -> Ident {
        self.f64_ident
    }

    pub fn unit_ident(&self) -> Ident {
        self.unit_ident
    }

    pub fn bool_ident(&self) -> Ident {
        self.bool_ident
    }

    pub fn u8_ident(&self) -> Ident {
        self.u8_ident
    }

    pub fn u16_ident(&self) -> Ident {
        self.u16_ident
    }

    pub fn u32_ident(&self) -> Ident {
        self.u32_ident
    }

    pub fn u64_ident(&self) -> Ident {
        self.u64_ident
    }

    pub fn u128_ident(&self) -> Ident {
        self.u128_ident
    }

    pub fn usize_ident(&self) -> Ident {
        self.usize_ident
    }

    pub fn trai_ty_ident(&self) -> Ident {
        self.trai_ty_ident
    }

    pub fn module_ident(&self) -> Ident {
        self.module_ident
    }

    pub fn crate_ident(&self) -> Ident {
        self.crate_ident
    }

    pub fn lifetime_ty_ident(&self) -> Ident {
        self.lifetime_ty_ident
    }

    pub fn never_ident(&self) -> Ident {
        self.never_ident
    }

    pub fn derive_ident(&self) -> Ident {
        self.derive_ident
    }
}

#[salsa::tracked(jar = CowordJar, return_ref)]
pub(crate) fn ident_menu(db: &dyn CowordDb) -> CowordMenu {
    CowordMenu::new(db)
}
