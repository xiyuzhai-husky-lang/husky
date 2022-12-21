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
    core: EntityPath,
    core_ops: EntityPath,
    // core::ops::Add	The addition operator +.
    core_ops_add: EntityPath,
    // core::ops::AddAssign	The addition assignment operator +=.
    core_ops_add_assign: EntityPath,
    // core::ops::BitAnd	The bitwise AND operator &.
    core_ops_bit_and: EntityPath,
    // core::ops::BitAndAssign	The bitwise AND assignment operator &=.
    core_ops_bit_and_assign: EntityPath,
    // core::ops::BitOr	The bitwise OR operator |.
    core_ops_bit_or: EntityPath,
    // core::ops::BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign: EntityPath,
    // core::ops::BitXor	The bitwise XOR operator ^.
    core_ops_bit_xor: EntityPath,
    // core::ops::BitXorAssign	The bitwise XOR assignment operator ^=.
    core_ops_bit_xor_assign: EntityPath,
    // core::ops::Div	The division operator /.
    core_ops_div: EntityPath,
    // core::ops::DivAssign	The division assignment operator /=.
    core_ops_div_assign: EntityPath,
    // core::ops::Mul	The multiplication operator *.
    core_ops_mul: EntityPath,
    // core::ops::MulAssign	The multiplication assignment operator *=.
    core_ops_mul_assign: EntityPath,
    // core::ops::Neg	The unary negation operator -.
    core_ops_neg: EntityPath,
    // Not	The unary logical negation operator !.
    core_ops_not: EntityPath,
    std: EntityPath,
    // prelude
    unit: EntityPath,
    bool: EntityPath,
    i32: EntityPath,
    i64: EntityPath,
    f32: EntityPath,
    f64: EntityPath,
    b32: EntityPath,
    b64: EntityPath,
    u32: EntityPath,
    u64: EntityPath,
    trai: EntityPath,
    module: EntityPath,
}

impl EntityPathMenu {
    pub(crate) fn new(db: &dyn EntityPathDb, toolchain: Toolchain) -> EntityPathResult<Self> {
        todo!()
        // let menu0 = EntityPathMenu0::new(db, toolchain)?;
        // let menu1 = EntityPathMenu1::new(db, toolchain, menu0);
        // let menu2 = EntityPathMenu2::new(db, toolchain, menu1);
        // let menu3 = EntityPathMenu3::new(db, toolchain, menu2);
        // Ok(Self { parent: menu3 })
    }

    pub fn i32(&self) -> EntityPath {
        self.i32
    }

    pub fn i64(&self) -> EntityPath {
        self.i64
    }

    pub fn f32(&self) -> EntityPath {
        self.f32
    }

    pub fn f64(&self) -> EntityPath {
        self.f64
    }

    pub fn b32(&self) -> EntityPath {
        self.b32
    }

    pub fn b64(&self) -> EntityPath {
        self.b64
    }

    pub fn u32(&self) -> EntityPath {
        self.u32
    }

    pub fn u64(&self) -> EntityPath {
        self.u64
    }

    pub fn core(&self) -> EntityPath {
        self.core
    }

    pub fn core_ops(&self) -> EntityPath {
        self.core_ops
    }

    pub fn core_ops_add(&self) -> EntityPath {
        self.core_ops_add
    }

    pub fn core_ops_add_assign(&self) -> EntityPath {
        self.core_ops_add_assign
    }

    pub fn core_ops_bit_and(&self) -> EntityPath {
        self.core_ops_bit_and
    }

    pub fn core_ops_bit_and_assign(&self) -> EntityPath {
        self.core_ops_bit_and_assign
    }

    pub fn core_ops_bit_or(&self) -> EntityPath {
        self.core_ops_bit_or
    }

    pub fn core_ops_bit_or_assign(&self) -> EntityPath {
        self.core_ops_bit_or_assign
    }

    pub fn core_ops_bit_xor(&self) -> EntityPath {
        self.core_ops_bit_xor
    }

    pub fn core_ops_bit_xor_assign(&self) -> EntityPath {
        self.core_ops_bit_xor_assign
    }

    pub fn core_ops_div(&self) -> EntityPath {
        self.core_ops_div
    }

    pub fn core_ops_div_assign(&self) -> EntityPath {
        self.core_ops_div_assign
    }

    pub fn core_ops_mul(&self) -> EntityPath {
        self.core_ops_mul
    }

    pub fn core_ops_mul_assign(&self) -> EntityPath {
        self.core_ops_mul_assign
    }

    pub fn core_ops_neg(&self) -> EntityPath {
        self.core_ops_neg
    }

    pub fn core_ops_not(&self) -> EntityPath {
        self.core_ops_not
    }

    pub fn std(&self) -> EntityPath {
        self.std
    }

    pub fn trai(&self) -> EntityPath {
        self.trai
    }

    pub fn module(&self) -> EntityPath {
        self.module
    }

    pub fn unit(&self) -> EntityPath {
        self.unit
    }

    pub fn bool(&self) -> EntityPath {
        self.bool
    }
}
