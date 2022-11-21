use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu5 {
    // Add	The addition operator +.
    core_ops_add: Term,
    // AddAssign	The addition assignment operator +=.
    core_ops_add_assign: Term,
    // BitAnd	The bitwise AND operator &.
    core_ops_bit_and: Term,
    // BitAndAssign	The bitwise AND assignment operator &=.
    core_ops_bit_and_assign: Term,
    // BitOr	The bitwise OR operator |.
    core_ops_bit_or: Term,
    // BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign: Term,
    // BitXor	The bitwise XOR operator ^.
    core_ops_bit_xor: Term,
    // BitXorAssign	The bitwise XOR assignment operator ^=.
    core_ops_bit_xor_assign: Term,
    // Div	The division operator /.
    core_ops_div: Term,
    // DivAssign	The division assignment operator /=.
    core_ops_div_assign: Term,
    // Mul	The multiplication operator *.
    core_ops_mul: Term,
    // MulAssign	The multiplication assignment operator *=.
    core_ops_mul_assign: Term,
    // Neg	The unary negation operator -.
    core_ops_neg: Term,
    // Not	The unary logical negation operator !.
    core_ops_not: Term,
    parent: TermMenu4,
}

impl std::ops::Deref for TermMenu5 {
    type Target = TermMenu4;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu5 {
    pub(super) fn new(db: &dyn TermDb, menu4: TermMenu4) -> Self {
        let core_ops = menu4.core_ops();
        Self {
            core_ops_add: TermSubentity::new(db, core_ops, "Add"),
            core_ops_add_assign: TermSubentity::new(db, core_ops, "AddAssign"),
            core_ops_bit_and: TermSubentity::new(db, core_ops, "BitAnd"),
            core_ops_bit_and_assign: TermSubentity::new(db, core_ops, "BitAndAssign"),
            core_ops_bit_or: TermSubentity::new(db, core_ops, "BitOr"),
            core_ops_bit_or_assign: TermSubentity::new(db, core_ops, "BitOrAssign"),
            core_ops_bit_xor: TermSubentity::new(db, core_ops, "BitXor"),
            core_ops_bit_xor_assign: TermSubentity::new(db, core_ops, "BitXorAssign"),
            core_ops_div: TermSubentity::new(db, core_ops, "Div"),
            core_ops_div_assign: TermSubentity::new(db, core_ops, "DivAssign"),
            core_ops_mul: TermSubentity::new(db, core_ops, "Mul"),
            core_ops_mul_assign: TermSubentity::new(db, core_ops, "MulAssign"),
            core_ops_neg: TermSubentity::new(db, core_ops, "Neg"),
            core_ops_not: TermSubentity::new(db, core_ops, "Not"),
            parent: menu4,
        }
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
}
