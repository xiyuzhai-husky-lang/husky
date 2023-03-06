use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu0 {
    universe0: TermUniverse,
    universe1: TermUniverse,
    prop: TermCategory,
    ty0: TermCategory,
    eval_lifetime: TermLiteral,
    static_lifetime: TermLiteral,
    unit: Term,
    never: Term,
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
    option_ty_path: Term,
    slice_ty_path: Term,
    str_ty_path: Term,
    ref_ty_path: Term,
    list_ty: Term,
    i8: Term,
    i16: Term,
    i32: Term,
    i64: Term,
    f32: Term,
    f64: Term,
    r32: Term,
    r64: Term,
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
            static_lifetime: TermLiteral::StaticLifetime,
            universe0,
            universe1,
            prop: TermCategory::new(universe0),
            ty0: TermCategory::new(universe1),
            core_ops_add: Term::EntityPath(entity_path_menu.core_ops_add().into()),
            // start here
            // Term::Entity(entity_path_menu.core_ops_())
            core_ops_add_assign: Term::EntityPath(entity_path_menu.core_ops_add_assign().into()),
            core_ops_bit_and: Term::EntityPath(entity_path_menu.core_ops_bit_and().into()),
            core_ops_bit_and_assign: Term::EntityPath(
                entity_path_menu.core_ops_bit_and_assign().into(),
            ),
            core_ops_bit_or: Term::EntityPath(entity_path_menu.core_ops_bit_or().into()),
            core_ops_bit_or_assign: Term::EntityPath(
                entity_path_menu.core_ops_bit_or_assign().into(),
            ),
            core_ops_bit_xor: Term::EntityPath(entity_path_menu.core_ops_bit_xor().into()),
            core_ops_bit_xor_assign: Term::EntityPath(
                entity_path_menu.core_ops_bit_xor_assign().into(),
            ),
            core_ops_div: Term::EntityPath(entity_path_menu.core_ops_div().into()),
            core_ops_div_assign: Term::EntityPath(entity_path_menu.core_ops_div_assign().into()),
            core_ops_mul: Term::EntityPath(entity_path_menu.core_ops_mul().into()),
            core_ops_mul_assign: Term::EntityPath(entity_path_menu.core_ops_mul_assign().into()),
            core_ops_neg: Term::EntityPath(entity_path_menu.core_ops_neg().into()),
            core_ops_not: Term::EntityPath(entity_path_menu.core_ops_not().into()),
            option_ty_path: TermEntityPath::TypeOntology(entity_path_menu.option_ty_path()).into(),
            slice_ty_path: TermEntityPath::TypeOntology(entity_path_menu.slice_ty_path()).into(),
            str_ty_path: TermEntityPath::TypeOntology(entity_path_menu.str_ty_path()).into(),
            ref_ty_path: TermEntityPath::TypeOntology(entity_path_menu.ref_ty_path()).into(),
            list_ty: TermEntityPath::TypeOntology(entity_path_menu.list_ty_path()).into(),
            unit: TermEntityPath::TypeOntology(entity_path_menu.unit_ty_path()).into(),
            never: TermEntityPath::TypeOntology(entity_path_menu.never_ty_path()).into(),
            bool: TermEntityPath::TypeOntology(entity_path_menu.bool_ty_path()).into(),
            trai_ty: TermEntityPath::TypeOntology(entity_path_menu.trai_ty_path()).into(),
            lifetime_ty: TermEntityPath::TypeOntology(entity_path_menu.lifetime_ty_path()).into(),
            module: TermEntityPath::TypeOntology(entity_path_menu.module_ty_path()).into(),
            i8: TermEntityPath::TypeOntology(entity_path_menu.i8_ty_path()).into(),
            i16: TermEntityPath::TypeOntology(entity_path_menu.i16_ty_path()).into(),
            i32: TermEntityPath::TypeOntology(entity_path_menu.i32_ty_path()).into(),
            i64: TermEntityPath::TypeOntology(entity_path_menu.i64_ty_path()).into(),
            f32: TermEntityPath::TypeOntology(entity_path_menu.f32_ty_path()).into(),
            f64: TermEntityPath::TypeOntology(entity_path_menu.f64_ty_path()).into(),
            r32: TermEntityPath::TypeOntology(entity_path_menu.r32_ty_path()).into(),
            r64: TermEntityPath::TypeOntology(entity_path_menu.r64_ty_path()).into(),
        }
    }

    pub fn universe0(&self) -> TermUniverse {
        self.universe0
    }

    pub fn universe1(&self) -> TermUniverse {
        self.universe1
    }

    /// `Prop`
    pub fn prop(&self) -> TermCategory {
        self.prop
    }

    /// `Type`
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

    pub fn option_ty_path(&self) -> Term {
        self.option_ty_path
    }

    pub fn slice_ty_path(&self) -> Term {
        self.slice_ty_path
    }

    pub fn ref_ty_path(&self) -> Term {
        self.ref_ty_path
    }

    pub fn list(&self) -> Term {
        self.list_ty
    }

    pub fn unit(&self) -> Term {
        self.unit
    }

    pub fn never(&self) -> Term {
        self.never
    }

    pub fn trai_ty(&self) -> Term {
        self.trai_ty
    }

    pub fn module(&self) -> Term {
        self.module
    }

    pub fn bool(&self) -> Term {
        self.bool
    }

    pub fn i8(&self) -> Term {
        self.i8
    }

    pub fn i16(&self) -> Term {
        self.i16
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

    pub fn r64(&self) -> Term {
        self.r64
    }

    pub fn eval_lifetime(&self) -> Term {
        self.eval_lifetime.into()
    }

    pub fn static_lifetime(&self) -> Term {
        self.static_lifetime.into()
    }

    /// Lifetime
    pub fn lifetime_ty(&self) -> Term {
        self.lifetime_ty
    }

    pub fn str_ty_path(&self) -> Term {
        self.str_ty_path
    }
}
