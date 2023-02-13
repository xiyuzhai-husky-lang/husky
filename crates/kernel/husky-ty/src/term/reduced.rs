use super::*;

pub(crate) fn calc_reduced_term(db: &dyn TypeDb, term: Term) -> ReducedTerm {
    // ad hoc
    ReducedTerm(term)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReducedTerm(Term);

impl ReducedTerm {
    pub fn term(&self) -> Term {
        self.0
    }
}

impl<Db: TypeDb + ?Sized> salsa::DebugWithDb<Db> for ReducedTerm {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.0.fmt(f, db, include_all_fields)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReducedTermMenu<'a> {
    term_menu: &'a TermMenu,
}

impl<'a> ReducedTermMenu<'a> {
    pub(crate) fn new(term_menu: &'a TermMenu) -> Self {
        Self { term_menu }
    }

    pub fn never(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.never())
    }

    pub fn unit(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.unit())
    }

    pub fn bool(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.bool())
    }

    pub fn i32(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.i32())
    }

    pub fn r32(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.r32())
    }

    pub fn list(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.list())
    }

    pub fn static_str_ref(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.static_str_ref())
    }

    pub fn universe0(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.universe0().into())
    }

    pub fn universe1(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.universe1().into())
    }

    /// `Prop`
    pub fn prop(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.prop().into())
    }

    /// `Type`
    pub fn ty0(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.ty0().into())
    }

    // pub fn core(&self) -> ReducedTerm {
    //     self.term_menu.core
    // }

    // pub fn core_ops(&self) -> ReducedTerm {
    //     self.term_menu.core_ops
    // }

    // Add	The addition operator +.
    pub fn core_ops_add(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_add())
    }

    // AddAssign	The addition assignment operator +=.
    pub fn core_ops_add_assign(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_add_assign())
    }

    // BitAnd	The bitwise AND operator &.
    pub fn core_ops_bit_and(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_bit_and())
    }

    // BitAndAssign	The bitwise AND assignment operator &=.
    pub fn core_ops_bit_and_assign(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_bit_and_assign())
    }

    // BitOr	The bitwise OR operator |.
    pub fn core_ops_bit_or(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_bit_or())
    }

    // BitOrAssign	The bitwise OR assignment operator |=.
    pub fn core_ops_bit_or_assign(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_bit_or_assign())
    }
    // BitXor	The bitwise XOR operator ^.
    pub fn core_ops_bit_xor(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_bit_xor())
    }

    // BitXorAssign	The bitwise XOR assignment operator ^=.
    pub fn core_ops_bit_xor_assign(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_bit_or_assign())
    }

    // Div	The division operator /.
    pub fn core_ops_div(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_div())
    }

    // DivAssign	The division assignment operator /=.
    pub fn core_ops_div_assign(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_div_assign())
    }

    // Mul	The multiplication operator *.
    pub fn core_ops_mul(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_mul())
    }

    // MulAssign	The multiplication assignment operator *=.
    pub fn core_ops_mul_assign(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_mul_assign())
    }

    // Neg	The unary negation operator -.
    pub fn core_ops_neg(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_neg())
    }

    // Not	The unary logical negation operator !.
    pub fn core_ops_not(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.core_ops_not())
    }

    pub fn option_ty_path(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.option_ty_path())
    }

    pub fn slice_ty_path(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.slice_ty_path())
    }

    pub fn ref_ty_path(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.ref_ty_path())
    }

    pub fn trai(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.trai())
    }

    pub fn module(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.module())
    }

    pub fn i64(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.i64())
    }

    pub fn f32(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.f32())
    }

    pub fn f64(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.f64())
    }

    pub fn r64(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.r64())
    }

    pub fn eval_lifetime(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.eval_lifetime())
    }

    pub fn static_lifetime(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.static_lifetime())
    }

    pub fn lifetime_ty(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.lifetime_ty())
    }

    pub fn str_ty_path(&self) -> ReducedTerm {
        ReducedTerm(self.term_menu.str_ty_path())
    }

    pub fn term_menu(&self) -> &'a TermMenu {
        self.term_menu
    }
}
