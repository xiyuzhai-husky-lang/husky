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
    core_ops_add: ConnectedModuleItemPath,
    // core::ops::AddAssign	The addition assignment operator +=.
    core_ops_add_assign: ConnectedModuleItemPath,
    // core::ops::BitAnd	The bitwise AND operator &.
    core_ops_bit_and: ConnectedModuleItemPath,
    // core::ops::BitAndAssign	The bitwise AND assignment operator &=.
    core_ops_bit_and_assign: ConnectedModuleItemPath,
    // core::ops::BitOr	The bitwise OR operator |.
    core_ops_bit_or: ConnectedModuleItemPath,
    // core::ops::BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign: ConnectedModuleItemPath,
    // core::ops::BitXor	The bitwise XOR operator ^.
    core_ops_bit_xor: ConnectedModuleItemPath,
    // core::ops::BitXorAssign	The bitwise XOR assignment operator ^=.
    core_ops_bit_xor_assign: ConnectedModuleItemPath,
    // core::ops::Div	The division operator /.
    core_ops_div: ConnectedModuleItemPath,
    // core::ops::DivAssign	The division assignment operator /=.
    core_ops_div_assign: ConnectedModuleItemPath,
    // core::ops::Mul	The multiplication operator *.
    core_ops_mul: ConnectedModuleItemPath,
    // core::ops::MulAssign	The multiplication assignment operator *=.
    core_ops_mul_assign: ConnectedModuleItemPath,
    // core::ops::Neg	The unary negation operator -.
    core_ops_neg: ConnectedModuleItemPath,
    // Not	The unary logical negation operator !.
    core_ops_not: ConnectedModuleItemPath,
    // prelude
    unit: ConnectedModuleItemPath,
    bool: ConnectedModuleItemPath,
    i32: ConnectedModuleItemPath,
    i64: ConnectedModuleItemPath,
    f32: ConnectedModuleItemPath,
    f64: ConnectedModuleItemPath,
    b32: ConnectedModuleItemPath,
    b64: ConnectedModuleItemPath,
    u32: ConnectedModuleItemPath,
    u64: ConnectedModuleItemPath,
    trai: ConnectedModuleItemPath,
    module: ConnectedModuleItemPath,
}

impl EntityPathMenu {
    pub(crate) fn new(db: &dyn EntityPathDb, toolchain: Toolchain) -> EntityPathResult<Self> {
        let word_menu = db.word_menu();
        let path_menu = db.dev_path_menu()?;
        let core_ops = path_menu.core_ops();
        let core_num = path_menu.core_num();
        let core_ops_add =
            ConnectedModuleItemPath::new(db, core_ops, db.it_ident_borrowed("Add").unwrap());
        let core_ops_add_assign =
            ConnectedModuleItemPath::new(db, core_ops, db.it_ident_borrowed("AddAssign").unwrap());
        let core_ops_bit_and =
            ConnectedModuleItemPath::new(db, core_ops, db.it_ident_borrowed("BitAnd").unwrap());
        let core_ops_bit_and_assign = ConnectedModuleItemPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitAndAssign").unwrap(),
        );
        let core_ops_bit_or =
            ConnectedModuleItemPath::new(db, core_ops, db.it_ident_borrowed("BitOr").unwrap());
        let core_ops_bit_or_assign = ConnectedModuleItemPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitOrAssign").unwrap(),
        );
        let core_ops_bit_xor =
            ConnectedModuleItemPath::new(db, core_ops, db.it_ident_borrowed("BitXor").unwrap());
        let core_ops_bit_xor_assign = ConnectedModuleItemPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitXorAssign").unwrap(),
        );
        let core_ops_div =
            ConnectedModuleItemPath::new(db, core_ops, db.it_ident_borrowed("Div").unwrap());
        let core_ops_div_assign =
            ConnectedModuleItemPath::new(db, core_ops, db.it_ident_borrowed("DivAssign").unwrap());
        let core_ops_mul =
            ConnectedModuleItemPath::new(db, core_ops, db.it_ident_borrowed("Mul").unwrap());
        let core_ops_mul_assign =
            ConnectedModuleItemPath::new(db, core_ops, db.it_ident_borrowed("MulAssign").unwrap());
        let core_ops_neg =
            ConnectedModuleItemPath::new(db, core_ops, db.it_ident_borrowed("Neg").unwrap());
        let core_ops_not =
            ConnectedModuleItemPath::new(db, core_ops, db.it_ident_borrowed("Not").unwrap());
        let i32 = ConnectedModuleItemPath::new(db, core_num, word_menu.i32());
        let i64 = ConnectedModuleItemPath::new(db, core_num, word_menu.i64());
        let unit = ConnectedModuleItemPath::new(db, core_num, word_menu.unit());
        let bool = ConnectedModuleItemPath::new(db, core_num, word_menu.bool());
        let f32 = ConnectedModuleItemPath::new(db, core_num, word_menu.f32());
        let f64 = ConnectedModuleItemPath::new(db, core_num, word_menu.f64());
        let b32 = ConnectedModuleItemPath::new(db, core_num, word_menu.b32());
        let b64 = ConnectedModuleItemPath::new(db, core_num, word_menu.b64());
        let u32 = ConnectedModuleItemPath::new(db, core_num, word_menu.u32());
        let u64 = ConnectedModuleItemPath::new(db, core_num, word_menu.u64());
        let trai = ConnectedModuleItemPath::new(db, core_num, word_menu.trai());
        let module = ConnectedModuleItemPath::new(db, core_num, word_menu.module());
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

    pub fn i32(&self) -> ConnectedModuleItemPath {
        self.i32
    }

    pub fn i64(&self) -> ConnectedModuleItemPath {
        self.i64
    }

    pub fn f32(&self) -> ConnectedModuleItemPath {
        self.f32
    }

    pub fn f64(&self) -> ConnectedModuleItemPath {
        self.f64
    }

    pub fn b32(&self) -> ConnectedModuleItemPath {
        self.b32
    }

    pub fn b64(&self) -> ConnectedModuleItemPath {
        self.b64
    }

    pub fn u32(&self) -> ConnectedModuleItemPath {
        self.u32
    }

    pub fn u64(&self) -> ConnectedModuleItemPath {
        self.u64
    }

    pub fn core_ops_add(&self) -> ConnectedModuleItemPath {
        self.core_ops_add
    }

    pub fn core_ops_add_assign(&self) -> ConnectedModuleItemPath {
        self.core_ops_add_assign
    }

    pub fn core_ops_bit_and(&self) -> ConnectedModuleItemPath {
        self.core_ops_bit_and
    }

    pub fn core_ops_bit_and_assign(&self) -> ConnectedModuleItemPath {
        self.core_ops_bit_and_assign
    }

    pub fn core_ops_bit_or(&self) -> ConnectedModuleItemPath {
        self.core_ops_bit_or
    }

    pub fn core_ops_bit_or_assign(&self) -> ConnectedModuleItemPath {
        self.core_ops_bit_or_assign
    }

    pub fn core_ops_bit_xor(&self) -> ConnectedModuleItemPath {
        self.core_ops_bit_xor
    }

    pub fn core_ops_bit_xor_assign(&self) -> ConnectedModuleItemPath {
        self.core_ops_bit_xor_assign
    }

    pub fn core_ops_div(&self) -> ConnectedModuleItemPath {
        self.core_ops_div
    }

    pub fn core_ops_div_assign(&self) -> ConnectedModuleItemPath {
        self.core_ops_div_assign
    }

    pub fn core_ops_mul(&self) -> ConnectedModuleItemPath {
        self.core_ops_mul
    }

    pub fn core_ops_mul_assign(&self) -> ConnectedModuleItemPath {
        self.core_ops_mul_assign
    }

    pub fn core_ops_neg(&self) -> ConnectedModuleItemPath {
        self.core_ops_neg
    }

    pub fn core_ops_not(&self) -> ConnectedModuleItemPath {
        self.core_ops_not
    }

    pub fn trai(&self) -> ConnectedModuleItemPath {
        self.trai
    }

    pub fn module(&self) -> ConnectedModuleItemPath {
        self.module
    }

    pub fn unit(&self) -> ConnectedModuleItemPath {
        self.unit
    }

    pub fn bool(&self) -> ConnectedModuleItemPath {
        self.bool
    }
}
