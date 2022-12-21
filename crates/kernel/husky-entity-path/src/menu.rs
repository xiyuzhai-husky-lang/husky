use crate::*;

#[salsa::tracked(jar = EntityPathJar, return_ref)]
pub(crate) fn entity_path_menu(
    db: &dyn EntityPathDb,
    toolchain: Toolchain,
) -> EntityPathResult<EntityPathMenu> {
    EntityPathMenu::new(db, toolchain)
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu {
    // core::ops::Add	The addition operator +.
    core_ops_add: ModuleItemPath,
    // core::ops::AddAssign	The addition assignment operator +=.
    core_ops_add_assign: ModuleItemPath,
    // core::ops::BitAnd	The bitwise AND operator &.
    core_ops_bit_and: ModuleItemPath,
    // core::ops::BitAndAssign	The bitwise AND assignment operator &=.
    core_ops_bit_and_assign: ModuleItemPath,
    // core::ops::BitOr	The bitwise OR operator |.
    core_ops_bit_or: ModuleItemPath,
    // core::ops::BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign: ModuleItemPath,
    // core::ops::BitXor	The bitwise XOR operator ^.
    core_ops_bit_xor: ModuleItemPath,
    // core::ops::BitXorAssign	The bitwise XOR assignment operator ^=.
    core_ops_bit_xor_assign: ModuleItemPath,
    // core::ops::Div	The division operator /.
    core_ops_div: ModuleItemPath,
    // core::ops::DivAssign	The division assignment operator /=.
    core_ops_div_assign: ModuleItemPath,
    // core::ops::Mul	The multiplication operator *.
    core_ops_mul: ModuleItemPath,
    // core::ops::MulAssign	The multiplication assignment operator *=.
    core_ops_mul_assign: ModuleItemPath,
    // core::ops::Neg	The unary negation operator -.
    core_ops_neg: ModuleItemPath,
    // Not	The unary logical negation operator !.
    core_ops_not: ModuleItemPath,
    // prelude
    unit: ModuleItemPath,
    bool: ModuleItemPath,
    i32: ModuleItemPath,
    i64: ModuleItemPath,
    f32: ModuleItemPath,
    f64: ModuleItemPath,
    b32: ModuleItemPath,
    b64: ModuleItemPath,
    u32: ModuleItemPath,
    u64: ModuleItemPath,
    trai: ModuleItemPath,
    module: ModuleItemPath,
}

impl EntityPathMenu {
    pub(crate) fn new(db: &dyn EntityPathDb, toolchain: Toolchain) -> EntityPathResult<Self> {
        let word_menu = db.word_menu();
        let path_menu = db.path_menu(toolchain)?;
        let core_ops = path_menu.core_ops();
        let core_num = path_menu.core_num();
        let core_ops_add = ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("Add").unwrap());
        let core_ops_add_assign =
            ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("AddAssign").unwrap());
        let core_ops_bit_and =
            ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("BitAnd").unwrap());
        let core_ops_bit_and_assign =
            ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("BitAndAssign").unwrap());
        let core_ops_bit_or =
            ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("BitOr").unwrap());
        let core_ops_bit_or_assign =
            ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("BitOrAssign").unwrap());
        let core_ops_bit_xor =
            ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("BitXor").unwrap());
        let core_ops_bit_xor_assign =
            ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("BitXorAssign").unwrap());
        let core_ops_div = ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("Div").unwrap());
        let core_ops_div_assign =
            ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("DivAssign").unwrap());
        let core_ops_mul = ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("Mul").unwrap());
        let core_ops_mul_assign =
            ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("MulAssign").unwrap());
        let core_ops_neg = ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("Neg").unwrap());
        let core_ops_not = ModuleItemPath::new(db, core_ops, db.it_ident_borrowed("Not").unwrap());
        let i32 = ModuleItemPath::new(db, core_num, word_menu.i32());
        let i64 = ModuleItemPath::new(db, core_num, word_menu.i64());
        let unit = ModuleItemPath::new(db, core_num, word_menu.unit());
        let bool = ModuleItemPath::new(db, core_num, word_menu.bool());
        let f32 = ModuleItemPath::new(db, core_num, word_menu.f32());
        let f64 = ModuleItemPath::new(db, core_num, word_menu.f64());
        let b32 = ModuleItemPath::new(db, core_num, word_menu.b32());
        let b64 = ModuleItemPath::new(db, core_num, word_menu.b64());
        let u32 = ModuleItemPath::new(db, core_num, word_menu.u32());
        let u64 = ModuleItemPath::new(db, core_num, word_menu.u64());
        let trai = ModuleItemPath::new(db, core_num, word_menu.trai());
        let module = ModuleItemPath::new(db, core_num, word_menu.module());
        Ok(Self {
            core_ops_add,
            core_ops_add_assign,
            core_ops_bit_and,
            core_ops_bit_and_assign,
            core_ops_bit_or,
            core_ops_bit_or_assign,
            core_ops_bit_xor,
            core_ops_bit_xor_assign,
            core_ops_div,
            core_ops_div_assign,
            core_ops_mul,
            core_ops_mul_assign,
            core_ops_neg,
            core_ops_not,
            unit,
            bool,
            i32,
            i64,
            f32,
            f64,
            b32,
            b64,
            u32,
            u64,
            trai,
            module,
        })
    }

    pub fn i32(&self) -> ModuleItemPath {
        self.i32
    }

    pub fn i64(&self) -> ModuleItemPath {
        self.i64
    }

    pub fn f32(&self) -> ModuleItemPath {
        self.f32
    }

    pub fn f64(&self) -> ModuleItemPath {
        self.f64
    }

    pub fn b32(&self) -> ModuleItemPath {
        self.b32
    }

    pub fn b64(&self) -> ModuleItemPath {
        self.b64
    }

    pub fn u32(&self) -> ModuleItemPath {
        self.u32
    }

    pub fn u64(&self) -> ModuleItemPath {
        self.u64
    }

    pub fn core_ops_add(&self) -> ModuleItemPath {
        self.core_ops_add
    }

    pub fn core_ops_add_assign(&self) -> ModuleItemPath {
        self.core_ops_add_assign
    }

    pub fn core_ops_bit_and(&self) -> ModuleItemPath {
        self.core_ops_bit_and
    }

    pub fn core_ops_bit_and_assign(&self) -> ModuleItemPath {
        self.core_ops_bit_and_assign
    }

    pub fn core_ops_bit_or(&self) -> ModuleItemPath {
        self.core_ops_bit_or
    }

    pub fn core_ops_bit_or_assign(&self) -> ModuleItemPath {
        self.core_ops_bit_or_assign
    }

    pub fn core_ops_bit_xor(&self) -> ModuleItemPath {
        self.core_ops_bit_xor
    }

    pub fn core_ops_bit_xor_assign(&self) -> ModuleItemPath {
        self.core_ops_bit_xor_assign
    }

    pub fn core_ops_div(&self) -> ModuleItemPath {
        self.core_ops_div
    }

    pub fn core_ops_div_assign(&self) -> ModuleItemPath {
        self.core_ops_div_assign
    }

    pub fn core_ops_mul(&self) -> ModuleItemPath {
        self.core_ops_mul
    }

    pub fn core_ops_mul_assign(&self) -> ModuleItemPath {
        self.core_ops_mul_assign
    }

    pub fn core_ops_neg(&self) -> ModuleItemPath {
        self.core_ops_neg
    }

    pub fn core_ops_not(&self) -> ModuleItemPath {
        self.core_ops_not
    }

    pub fn trai(&self) -> ModuleItemPath {
        self.trai
    }

    pub fn module(&self) -> ModuleItemPath {
        self.module
    }

    pub fn unit(&self) -> ModuleItemPath {
        self.unit
    }

    pub fn bool(&self) -> ModuleItemPath {
        self.bool
    }
}
