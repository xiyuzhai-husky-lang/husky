use husky_toolchain::Toolchain;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu0 {
    sort: Term,
    universe1: Term,
    unit: Term,
    core: Term,
    core_ops: Term,
    // core::ops::Add	The addition operator +.
    core_ops_add: Term,
    // core::ops::AddAssign	The addition assignment operator +=.
    core_ops_add_assign: Term,
    // core::ops::BitAnd	The bitwise AND operator &.
    core_ops_bit_and: Term,
    // core::ops::BitAndAssign	The bitwise AND assignment operator &=.
    core_ops_bit_and_assign: Term,
    // core::ops::BitOr	The bitwise OR operator |.
    core_ops_bit_or: Term,
    // core::ops::BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign: Term,
    // core::ops::BitXor	The bitwise XOR operator ^.
    core_ops_bit_xor: Term,
    // core::ops::BitXorAssign	The bitwise XOR assignment operator ^=.
    core_ops_bit_xor_assign: Term,
    // core::ops::Div	The division operator /.
    core_ops_div: Term,
    // core::ops::DivAssign	The division assignment operator /=.
    core_ops_div_assign: Term,
    // core::ops::Mul	The multiplication operator *.
    core_ops_mul: Term,
    // core::ops::MulAssign	The multiplication assignment operator *=.
    core_ops_mul_assign: Term,
    // core::ops::Neg	The unary negation operator -.
    core_ops_neg: Term,
    // Not	The unary logical negation operator !.
    core_ops_not: Term,
    std: Term,
    i32: Term,
    i64: Term,
    f32: Term,
    f64: Term,
    b32: Term,
    b64: Term,
    bool: Term,
    trai: Term,
    module: Term,
}

impl TermMenu0 {
    pub fn new(db: &dyn TermDb, toolchain: Toolchain) -> Self {
        let sort = db.it_term(TermAtom::new_category(TermCategory::Sort).into());
        let universe1 = db.it_term(TermAtom::new_universe(1).into());
        let entity_path_menu = db.entity_path_menu(toolchain);
        TermMenu0 {
            sort,
            universe1,
            core: db.it_entity_path_term(entity_path_menu.core()),
            core_ops: db.it_entity_path_term(entity_path_menu.core_ops()),
            core_ops_add: db.it_entity_path_term(entity_path_menu.core_ops_add()),
            // start here
            // db.it_entity_path_term(entity_path_menu.core_ops_())
            core_ops_add_assign: db.it_entity_path_term(entity_path_menu.core_ops_add_assign()),
            core_ops_bit_and: db.it_entity_path_term(entity_path_menu.core_ops_bit_and()),
            core_ops_bit_and_assign: db
                .it_entity_path_term(entity_path_menu.core_ops_bit_and_assign()),
            core_ops_bit_or: db.it_entity_path_term(entity_path_menu.core_ops_bit_or()),
            core_ops_bit_or_assign: db
                .it_entity_path_term(entity_path_menu.core_ops_bit_or_assign()),
            core_ops_bit_xor: db.it_entity_path_term(entity_path_menu.core_ops_bit_xor()),
            core_ops_bit_xor_assign: db
                .it_entity_path_term(entity_path_menu.core_ops_bit_xor_assign()),
            core_ops_div: db.it_entity_path_term(entity_path_menu.core_ops_div()),
            core_ops_div_assign: db.it_entity_path_term(entity_path_menu.core_ops_div_assign()),
            core_ops_mul: db.it_entity_path_term(entity_path_menu.core_ops_mul()),
            core_ops_mul_assign: db.it_entity_path_term(entity_path_menu.core_ops_mul_assign()),
            core_ops_neg: db.it_entity_path_term(entity_path_menu.core_ops_neg()),
            core_ops_not: db.it_entity_path_term(entity_path_menu.core_ops_not()),
            std: db.it_entity_path_term(entity_path_menu.std()),
            unit: db.it_entity_path_term(entity_path_menu.unit()),
            bool: db.it_entity_path_term(entity_path_menu.bool()),
            trai: db.it_entity_path_term(entity_path_menu.trai()),
            module: db.it_entity_path_term(entity_path_menu.module()),
            i32: db.it_entity_path_term(entity_path_menu.i32()),
            i64: db.it_entity_path_term(entity_path_menu.i64()),
            f32: db.it_entity_path_term(entity_path_menu.f32()),
            f64: db.it_entity_path_term(entity_path_menu.f64()),
            b32: db.it_entity_path_term(entity_path_menu.b32()),
            b64: db.it_entity_path_term(entity_path_menu.b64()),
        }
    }

    pub fn sort(&self) -> Term {
        self.sort
    }

    pub fn universe1(&self) -> Term {
        self.universe1
    }

    pub fn core(&self) -> Term {
        self.core
    }

    pub fn core_ops(&self) -> Term {
        self.core_ops
    }

    // Add	The addition operator +.
    pub fn core_ops_add(&self) -> Term {
        self.core_ops_add
    }

    // AddAssign	The addition assignment operator +=.
    pub fn core_ops_add_assign(&self) -> Term {
        self.core_ops_add_assign
    }

    // BitAnd	The bitwise AND operator &.
    pub fn core_ops_bit_and(&self) -> Term {
        self.core_ops_bit_and
    }

    // BitAndAssign	The bitwise AND assignment operator &=.
    pub fn core_ops_bit_and_assign(&self) -> Term {
        self.core_ops_bit_and_assign
    }

    // BitOr	The bitwise OR operator |.
    pub fn core_ops_bit_or(&self) -> Term {
        self.core_ops_bit_or
    }

    // BitOrAssign	The bitwise OR assignment operator |=.
    pub fn core_ops_bit_or_assign(&self) -> Term {
        self.core_ops_bit_or_assign
    }
    // BitXor	The bitwise XOR operator ^.
    pub fn core_ops_bit_xor(&self) -> Term {
        self.core_ops_bit_xor
    }

    // BitXorAssign	The bitwise XOR assignment operator ^=.
    pub fn core_ops_bit_xor_assign(&self) -> Term {
        self.core_ops_bit_or_assign
    }

    // Div	The division operator /.
    pub fn core_ops_div(&self) -> Term {
        self.core_ops_div
    }

    // DivAssign	The division assignment operator /=.
    pub fn core_ops_div_assign(&self) -> Term {
        self.core_ops_div_assign
    }

    // Mul	The multiplication operator *.
    pub fn core_ops_mul(&self) -> Term {
        self.core_ops_mul
    }

    // MulAssign	The multiplication assignment operator *=.
    pub fn core_ops_mul_assign(&self) -> Term {
        self.core_ops_mul_assign
    }

    // Neg	The unary negation operator -.
    pub fn core_ops_neg(&self) -> Term {
        self.core_ops_neg
    }

    // Not	The unary logical negation operator !.
    pub fn core_ops_not(&self) -> Term {
        self.core_ops_not
    }

    pub fn unit(&self) -> Term {
        self.unit
    }

    pub fn trai(&self) -> Term {
        self.trai
    }

    pub fn module(&self) -> Term {
        self.module
    }

    pub fn bool(&self) -> Term {
        self.bool
    }

    pub fn i32(&self) -> Term {
        self.i32
    }

    pub fn i64(&self) -> Term {
        self.i64
    }

    pub fn f32(&self) -> Term {
        self.f32
    }

    pub fn f64(&self) -> Term {
        self.f64
    }

    pub fn b32(&self) -> Term {
        self.b32
    }

    pub fn b64(&self) -> Term {
        self.b64
    }
}
