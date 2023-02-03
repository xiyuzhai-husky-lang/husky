use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu0 {
    universe0: TermUniverse,
    universe1: TermUniverse,
    prop: TermCategory,
    ty0: TermCategory,
    eval_lifetime: TermLiteral,
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
    core_option: Term,
    option_ty: Term,
    slice_ty: Term,
    ref_ty: Term,
    vec_ty: Term,
    std: Term,
    i32: Term,
    i64: Term,
    f32: Term,
    f64: Term,
    r32: Term,
    b64: Term,
    bool: Term,
    lifetime_ty: Term,
    trai_ty: Term,
    module: Term,
}

impl TermMenu0 {
    pub fn new(db: &dyn TermDb, toolchain: Toolchain) -> Self {
        // let sort = db.it_term(TermAtom::new_category(TermCategory::Sort).into());
        // let universe1 = db.it_term(TermAtom::new_universe(1).into());
        let vfs_path_menu = db.vfs_path_menu(toolchain).unwrap();
        let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
        let universe0 = TermUniverse::new(0);
        let universe1 = TermUniverse::new(1);
        TermMenu0 {
            eval_lifetime: TermLiteral::EvalLifetime,
            universe0,
            universe1,
            prop: TermCategory::new(universe0),
            ty0: TermCategory::new(universe1),
            core: Term::Entity(vfs_path_menu.core().into()),
            core_ops: Term::Entity(vfs_path_menu.core_ops().into()),
            core_ops_add: Term::Entity(entity_path_menu.core_ops_add().into()),
            // start here
            // Term::Entity(entity_path_menu.core_ops_())
            core_ops_add_assign: Term::Entity(entity_path_menu.core_ops_add_assign().into()),
            core_ops_bit_and: Term::Entity(entity_path_menu.core_ops_bit_and().into()),
            core_ops_bit_and_assign: Term::Entity(
                entity_path_menu.core_ops_bit_and_assign().into(),
            ),
            core_ops_bit_or: Term::Entity(entity_path_menu.core_ops_bit_or().into()),
            core_ops_bit_or_assign: Term::Entity(entity_path_menu.core_ops_bit_or_assign().into()),
            core_ops_bit_xor: Term::Entity(entity_path_menu.core_ops_bit_xor().into()),
            core_ops_bit_xor_assign: Term::Entity(
                entity_path_menu.core_ops_bit_xor_assign().into(),
            ),
            core_ops_div: Term::Entity(entity_path_menu.core_ops_div().into()),
            core_ops_div_assign: Term::Entity(entity_path_menu.core_ops_div_assign().into()),
            core_ops_mul: Term::Entity(entity_path_menu.core_ops_mul().into()),
            core_ops_mul_assign: Term::Entity(entity_path_menu.core_ops_mul_assign().into()),
            core_ops_neg: Term::Entity(entity_path_menu.core_ops_neg().into()),
            core_ops_not: Term::Entity(entity_path_menu.core_ops_not().into()),
            core_option: Term::Entity(vfs_path_menu.core_option().into()),
            option_ty: Term::Entity(entity_path_menu.option_ty().into()),
            slice_ty: Term::Entity(entity_path_menu.slice_ty().into()),
            ref_ty: Term::Entity(entity_path_menu.ref_ty().into()),
            vec_ty: Term::Entity(entity_path_menu.vec_ty().into()),
            std: Term::Entity(vfs_path_menu.std().into()),
            unit: Term::Entity(entity_path_menu.unit().into()),
            bool: Term::Entity(entity_path_menu.bool().into()),
            trai_ty: Term::Entity(entity_path_menu.trai_ty().into()),
            lifetime_ty: Term::Entity(entity_path_menu.lifetime_ty().into()),
            module: Term::Entity(entity_path_menu.module().into()),
            i32: Term::Entity(entity_path_menu.i32().into()),
            i64: Term::Entity(entity_path_menu.i64().into()),
            f32: Term::Entity(entity_path_menu.f32().into()),
            f64: Term::Entity(entity_path_menu.f64().into()),
            r32: Term::Entity(entity_path_menu.r32().into()),
            b64: Term::Entity(entity_path_menu.b64().into()),
        }
    }

    pub fn universe0(&self) -> TermUniverse {
        self.universe0
    }

    pub fn universe1(&self) -> TermUniverse {
        self.universe1
    }

    pub fn ty0(&self) -> TermCategory {
        self.ty0
    }

    // pub fn core(&self) -> Term {
    //     self.core
    // }

    // pub fn core_ops(&self) -> Term {
    //     self.core_ops
    // }

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

    pub fn option_ty(&self) -> Term {
        self.option_ty
    }

    pub fn slice_ty(&self) -> Term {
        self.slice_ty
    }

    pub fn ref_ty(&self) -> Term {
        self.ref_ty
    }

    pub fn vec_ty(&self) -> Term {
        self.vec_ty
    }

    pub fn unit(&self) -> Term {
        self.unit
    }

    pub fn trai(&self) -> Term {
        self.trai_ty
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

    pub fn r32(&self) -> Term {
        self.r32
    }

    pub fn b64(&self) -> Term {
        self.b64
    }

    pub fn eval_lifetime(&self) -> TermLiteral {
        self.eval_lifetime
    }

    pub fn lifetime_ty(&self) -> Term {
        self.lifetime_ty
    }
}
