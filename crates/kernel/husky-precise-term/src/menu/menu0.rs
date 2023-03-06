use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PreciseTermMenu0 {
    universe0: TermUniverse,
    universe1: TermUniverse,
    prop: TermCategory,
    ty0: TermCategory,
    eval_lifetime: TermLiteral,
    static_lifetime: TermLiteral,
    unit: PreciseTerm,
    never: PreciseTerm,
    // core::ops::Add	The addition operator +.
    core_ops_add: PreciseTerm,
    // core::ops::AddAssign	The addition assignment operator +=.
    core_ops_add_assign: PreciseTerm,
    // core::ops::BitAnd	The bitwise AND operator &.
    core_ops_bit_and: PreciseTerm,
    // core::ops::BitAndAssign	The bitwise AND assignment operator &=.
    core_ops_bit_and_assign: PreciseTerm,
    // core::ops::BitOr	The bitwise OR operator |.
    core_ops_bit_or: PreciseTerm,
    // core::ops::BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign: PreciseTerm,
    // core::ops::BitXor	The bitwise XOR operator ^.
    core_ops_bit_xor: PreciseTerm,
    // core::ops::BitXorAssign	The bitwise XOR assignment operator ^=.
    core_ops_bit_xor_assign: PreciseTerm,
    // core::ops::Div	The division operator /.
    core_ops_div: PreciseTerm,
    // core::ops::DivAssign	The division assignment operator /=.
    core_ops_div_assign: PreciseTerm,
    // core::ops::Mul	The multiplication operator *.
    core_ops_mul: PreciseTerm,
    // core::ops::MulAssign	The multiplication assignment operator *=.
    core_ops_mul_assign: PreciseTerm,
    // core::ops::Neg	The unary negation operator -.
    core_ops_neg: PreciseTerm,
    // Not	The unary logical negation operator !.
    core_ops_not: PreciseTerm,
    option_ty_path: PreciseTerm,
    slice_ty_path: PreciseTerm,
    str_ty_path: PreciseTerm,
    ref_ty_path: PreciseTerm,
    list_ty: PreciseTerm,
    i8: PreciseTerm,
    i16: PreciseTerm,
    i32: PreciseTerm,
    i64: PreciseTerm,
    f32: PreciseTerm,
    f64: PreciseTerm,
    r32: PreciseTerm,
    r64: PreciseTerm,
    bool: PreciseTerm,
    lifetime_ty: PreciseTerm,
    trai_ty: PreciseTerm,
    module: PreciseTerm,
}

impl PreciseTermMenu0 {
    pub fn new(db: &dyn PreciseTermDb, toolchain: Toolchain) -> Self {
        // let sort = db.it_precise_term(PreciseTermAtom::new_category(TermCategory::Sort).into());
        // let universe1 = db.it_precise_term(PreciseTermAtom::new_universe(1).into());
        let vfs_path_menu = db.vfs_path_menu(toolchain).unwrap();
        let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
        let universe0 = TermUniverse::new(0);
        let universe1 = TermUniverse::new(1);
        PreciseTermMenu0 {
            eval_lifetime: TermLiteral::EvalLifetime,
            static_lifetime: TermLiteral::StaticLifetime,
            universe0,
            universe1,
            prop: TermCategory::new(universe0),
            ty0: TermCategory::new(universe1),
            core_ops_add: PreciseTerm::EntityPath(entity_path_menu.core_ops_add().into()),
            // start here
            // PreciseTerm::Entity(entity_path_menu.core_ops_())
            core_ops_add_assign: PreciseTerm::EntityPath(
                entity_path_menu.core_ops_add_assign().into(),
            ),
            core_ops_bit_and: PreciseTerm::EntityPath(entity_path_menu.core_ops_bit_and().into()),
            core_ops_bit_and_assign: PreciseTerm::EntityPath(
                entity_path_menu.core_ops_bit_and_assign().into(),
            ),
            core_ops_bit_or: PreciseTerm::EntityPath(entity_path_menu.core_ops_bit_or().into()),
            core_ops_bit_or_assign: PreciseTerm::EntityPath(
                entity_path_menu.core_ops_bit_or_assign().into(),
            ),
            core_ops_bit_xor: PreciseTerm::EntityPath(entity_path_menu.core_ops_bit_xor().into()),
            core_ops_bit_xor_assign: PreciseTerm::EntityPath(
                entity_path_menu.core_ops_bit_xor_assign().into(),
            ),
            core_ops_div: PreciseTerm::EntityPath(entity_path_menu.core_ops_div().into()),
            core_ops_div_assign: PreciseTerm::EntityPath(
                entity_path_menu.core_ops_div_assign().into(),
            ),
            core_ops_mul: PreciseTerm::EntityPath(entity_path_menu.core_ops_mul().into()),
            core_ops_mul_assign: PreciseTerm::EntityPath(
                entity_path_menu.core_ops_mul_assign().into(),
            ),
            core_ops_neg: PreciseTerm::EntityPath(entity_path_menu.core_ops_neg().into()),
            core_ops_not: PreciseTerm::EntityPath(entity_path_menu.core_ops_not().into()),
            option_ty_path: TermEntityPath::TypeOntology(entity_path_menu.option_ty_path()).into(),
            slice_ty_path: TermEntityPath::TypeOntology(entity_path_menu.slice_ty_path()).into(),
            str_ty_path: TermEntityPath::TypeOntology(entity_path_menu.str_ty_path()).into(),
            ref_ty_path: TermEntityPath::TypeOntology(entity_path_menu.ref_ty_path()).into(),
            list_ty: TermEntityPath::TypeOntology(entity_path_menu.list_ty_path()).into(),
            unit: TermEntityPath::TypeOntology(entity_path_menu.unit()).into(),
            never: TermEntityPath::TypeOntology(entity_path_menu.never()).into(),
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

    // pub fn core(&self) -> PreciseTerm {
    //     self.core
    // }

    // pub fn core_ops(&self) -> PreciseTerm {
    //     self.core_ops
    // }

    // Add	The addition operator +.
    pub fn core_ops_add(&self) -> PreciseTerm {
        self.core_ops_add
    }

    // AddAssign	The addition assignment operator +=.
    pub fn core_ops_add_assign(&self) -> PreciseTerm {
        self.core_ops_add_assign
    }

    // BitAnd	The bitwise AND operator &.
    pub fn core_ops_bit_and(&self) -> PreciseTerm {
        self.core_ops_bit_and
    }

    // BitAndAssign	The bitwise AND assignment operator &=.
    pub fn core_ops_bit_and_assign(&self) -> PreciseTerm {
        self.core_ops_bit_and_assign
    }

    // BitOr	The bitwise OR operator |.
    pub fn core_ops_bit_or(&self) -> PreciseTerm {
        self.core_ops_bit_or
    }

    // BitOrAssign	The bitwise OR assignment operator |=.
    pub fn core_ops_bit_or_assign(&self) -> PreciseTerm {
        self.core_ops_bit_or_assign
    }
    // BitXor	The bitwise XOR operator ^.
    pub fn core_ops_bit_xor(&self) -> PreciseTerm {
        self.core_ops_bit_xor
    }

    // BitXorAssign	The bitwise XOR assignment operator ^=.
    pub fn core_ops_bit_xor_assign(&self) -> PreciseTerm {
        self.core_ops_bit_or_assign
    }

    // Div	The division operator /.
    pub fn core_ops_div(&self) -> PreciseTerm {
        self.core_ops_div
    }

    // DivAssign	The division assignment operator /=.
    pub fn core_ops_div_assign(&self) -> PreciseTerm {
        self.core_ops_div_assign
    }

    // Mul	The multiplication operator *.
    pub fn core_ops_mul(&self) -> PreciseTerm {
        self.core_ops_mul
    }

    // MulAssign	The multiplication assignment operator *=.
    pub fn core_ops_mul_assign(&self) -> PreciseTerm {
        self.core_ops_mul_assign
    }

    // Neg	The unary negation operator -.
    pub fn core_ops_neg(&self) -> PreciseTerm {
        self.core_ops_neg
    }

    // Not	The unary logical negation operator !.
    pub fn core_ops_not(&self) -> PreciseTerm {
        self.core_ops_not
    }

    pub fn option_ty_path(&self) -> PreciseTerm {
        self.option_ty_path
    }

    pub fn slice_ty_path(&self) -> PreciseTerm {
        self.slice_ty_path
    }

    pub fn ref_ty_path(&self) -> PreciseTerm {
        self.ref_ty_path
    }

    pub fn list(&self) -> PreciseTerm {
        self.list_ty
    }

    pub fn unit(&self) -> PreciseTerm {
        self.unit
    }

    pub fn never(&self) -> PreciseTerm {
        self.never
    }

    pub fn trai_ty(&self) -> PreciseTerm {
        self.trai_ty
    }

    pub fn module(&self) -> PreciseTerm {
        self.module
    }

    pub fn bool(&self) -> PreciseTerm {
        self.bool
    }

    pub fn i8(&self) -> PreciseTerm {
        self.i8
    }

    pub fn i16(&self) -> PreciseTerm {
        self.i16
    }

    pub fn i32(&self) -> PreciseTerm {
        self.i32
    }

    pub fn i64(&self) -> PreciseTerm {
        self.i64
    }

    pub fn f32(&self) -> PreciseTerm {
        self.f32
    }

    pub fn f64(&self) -> PreciseTerm {
        self.f64
    }

    pub fn r32(&self) -> PreciseTerm {
        self.r32
    }

    pub fn r64(&self) -> PreciseTerm {
        self.r64
    }

    pub fn eval_lifetime(&self) -> PreciseTerm {
        self.eval_lifetime.into()
    }

    pub fn static_lifetime(&self) -> PreciseTerm {
        self.static_lifetime.into()
    }

    /// Lifetime
    pub fn lifetime_ty(&self) -> PreciseTerm {
        self.lifetime_ty
    }

    pub fn str_ty_path(&self) -> PreciseTerm {
        self.str_ty_path
    }
}
