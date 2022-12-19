use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu2 {
    parent: EntityPathMenu1,
    // core::marker::Sized
    core_marker_sized: EntityPath,
    // core::basic::unit
    core_basic_unit: EntityPath,
    // core::basic::bool
    core_basic_bool: EntityPath,
    // core::basic::Trait
    core_basic_trai: EntityPath,
    // core::basic::Module
    core_basic_module: EntityPath,
    // core::num::i32
    core_num_i32: EntityPath,
    // core::num::i64
    core_num_i64: EntityPath,
    // core::num::b32
    core_num_b32: EntityPath,
    // core::num::b64
    core_num_b64: EntityPath,
    // core::num::f32
    core_num_f32: EntityPath,
    // core::num::f64
    core_num_f64: EntityPath,
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
}

impl EntityPathMenu2 {
    pub(crate) fn new(db: &dyn EntityPathDb, toolchain: Toolchain, menu1: EntityPathMenu1) -> Self {
        Self {
            core_marker_sized: menu1.core_marker().child(db, "Sized").unwrap(),
            core_basic_unit: menu1.core_basic().child(db, "unit").unwrap(),
            core_basic_bool: menu1.core_basic().child(db, "bool").unwrap(),
            core_basic_trai: menu1.core_basic().child(db, "Trait").unwrap(),
            core_basic_module: menu1.core_basic().child(db, "Module").unwrap(),
            core_num_i32: menu1.core_num().child(db, "i32").unwrap(),
            core_num_i64: menu1.core_num().child(db, "i64").unwrap(),
            core_num_b32: menu1.core_num().child(db, "b32").unwrap(),
            core_num_b64: menu1.core_num().child(db, "b64").unwrap(),
            core_num_f32: menu1.core_num().child(db, "f32").unwrap(),
            core_num_f64: menu1.core_num().child(db, "f64").unwrap(),
            core_ops_add: menu1.core_ops().child(db, "Add").unwrap(),
            core_ops_add_assign: menu1.core_ops().child(db, "AddAssign").unwrap(),
            core_ops_bit_and: menu1.core_ops().child(db, "BitAnd").unwrap(),
            core_ops_bit_and_assign: menu1.core_ops().child(db, "BitAndAssign").unwrap(),
            core_ops_bit_or: menu1.core_ops().child(db, "BitOr").unwrap(),
            core_ops_bit_or_assign: menu1.core_ops().child(db, "BitOrAssign").unwrap(),
            core_ops_bit_xor: menu1.core_ops().child(db, "BitXor").unwrap(),
            core_ops_bit_xor_assign: menu1.core_ops().child(db, "BitXorAssign").unwrap(),
            core_ops_div: menu1.core_ops().child(db, "Div").unwrap(),
            core_ops_div_assign: menu1.core_ops().child(db, "DivAssign").unwrap(),
            core_ops_mul: menu1.core_ops().child(db, "Mul").unwrap(),
            core_ops_mul_assign: menu1.core_ops().child(db, "MulAssign").unwrap(),
            core_ops_neg: menu1.core_ops().child(db, "Neg").unwrap(),
            core_ops_not: menu1.core_ops().child(db, "Not").unwrap(),
            parent: menu1,
        }
    }

    // Add	The addition operator +.
    pub fn core_ops_add(&self) -> EntityPath {
        self.core_ops_add
    }

    // AddAssign	The addition assignment operator +=.
    pub fn core_ops_add_assign(&self) -> EntityPath {
        self.core_ops_add_assign
    }

    // BitAnd	The bitwise AND operator &.
    pub fn core_ops_bit_and(&self) -> EntityPath {
        self.core_ops_bit_and
    }

    // BitAndAssign	The bitwise AND assignment operator &=.
    pub fn core_ops_bit_and_assign(&self) -> EntityPath {
        self.core_ops_bit_and_assign
    }

    // BitOr	The bitwise OR operator |.
    pub fn core_ops_bit_or(&self) -> EntityPath {
        self.core_ops_bit_or
    }

    // BitOrAssign	The bitwise OR assignment operator |=.
    pub fn core_ops_bit_or_assign(&self) -> EntityPath {
        self.core_ops_bit_or_assign
    }
    // BitXor	The bitwise XOR operator ^.
    pub fn core_ops_bit_xor(&self) -> EntityPath {
        self.core_ops_bit_xor
    }

    // BitXorAssign	The bitwise XOR assignment operator ^=.
    pub fn core_ops_bit_xor_assign(&self) -> EntityPath {
        self.core_ops_bit_or_assign
    }

    // Div	The division operator /.
    pub fn core_ops_div(&self) -> EntityPath {
        self.core_ops_div
    }

    // DivAssign	The division assignment operator /=.
    pub fn core_ops_div_assign(&self) -> EntityPath {
        self.core_ops_div_assign
    }

    // Mul	The multiplication operator *.
    pub fn core_ops_mul(&self) -> EntityPath {
        self.core_ops_mul
    }

    // MulAssign	The multiplication assignment operator *=.
    pub fn core_ops_mul_assign(&self) -> EntityPath {
        self.core_ops_mul_assign
    }

    // Neg	The unary negation operator -.
    pub fn core_ops_neg(&self) -> EntityPath {
        self.core_ops_neg
    }

    // Not	The unary logical negation operator !.
    pub fn core_ops_not(&self) -> EntityPath {
        self.core_ops_not
    }

    pub fn unit(&self) -> EntityPath {
        self.core_basic_unit
    }

    pub fn bool(&self) -> EntityPath {
        self.core_basic_bool
    }

    // core::basic::Trait
    pub fn trai(&self) -> EntityPath {
        self.core_basic_trai
    }

    // core::basic::Module
    pub fn module(&self) -> EntityPath {
        self.core_basic_module
    }

    pub fn i32(&self) -> EntityPath {
        self.core_num_i32
    }

    pub fn i64(&self) -> EntityPath {
        self.core_num_i64
    }

    pub fn b32(&self) -> EntityPath {
        self.core_num_b32
    }

    pub fn b64(&self) -> EntityPath {
        self.core_num_b64
    }

    pub fn f32(&self) -> EntityPath {
        self.core_num_f32
    }

    pub fn f64(&self) -> EntityPath {
        self.core_num_f64
    }
}

impl std::ops::Deref for EntityPathMenu2 {
    type Target = EntityPathMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
