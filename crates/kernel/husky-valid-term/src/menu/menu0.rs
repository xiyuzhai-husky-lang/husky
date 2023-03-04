use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ValidTermMenu0 {
    universe0: ValidTermUniverse,
    universe1: ValidTermUniverse,
    prop: ValidTermCategory,
    ty0: ValidTermCategory,
    eval_lifetime: ValidTermLiteral,
    static_lifetime: ValidTermLiteral,
    unit: ValidTerm,
    never: ValidTerm,
    // core::ops::Add	The addition operator +.
    core_ops_add: ValidTerm,
    // core::ops::AddAssign	The addition assignment operator +=.
    core_ops_add_assign: ValidTerm,
    // core::ops::BitAnd	The bitwise AND operator &.
    core_ops_bit_and: ValidTerm,
    // core::ops::BitAndAssign	The bitwise AND assignment operator &=.
    core_ops_bit_and_assign: ValidTerm,
    // core::ops::BitOr	The bitwise OR operator |.
    core_ops_bit_or: ValidTerm,
    // core::ops::BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign: ValidTerm,
    // core::ops::BitXor	The bitwise XOR operator ^.
    core_ops_bit_xor: ValidTerm,
    // core::ops::BitXorAssign	The bitwise XOR assignment operator ^=.
    core_ops_bit_xor_assign: ValidTerm,
    // core::ops::Div	The division operator /.
    core_ops_div: ValidTerm,
    // core::ops::DivAssign	The division assignment operator /=.
    core_ops_div_assign: ValidTerm,
    // core::ops::Mul	The multiplication operator *.
    core_ops_mul: ValidTerm,
    // core::ops::MulAssign	The multiplication assignment operator *=.
    core_ops_mul_assign: ValidTerm,
    // core::ops::Neg	The unary negation operator -.
    core_ops_neg: ValidTerm,
    // Not	The unary logical negation operator !.
    core_ops_not: ValidTerm,
    option_ty_path: ValidTerm,
    slice_ty_path: ValidTerm,
    str_ty_path: ValidTerm,
    ref_ty_path: ValidTerm,
    list_ty: ValidTerm,
    i8: ValidTerm,
    i16: ValidTerm,
    i32: ValidTerm,
    i64: ValidTerm,
    f32: ValidTerm,
    f64: ValidTerm,
    r32: ValidTerm,
    r64: ValidTerm,
    bool: ValidTerm,
    lifetime_ty: ValidTerm,
    trai_ty: ValidTerm,
    module: ValidTerm,
}

impl ValidTermMenu0 {
    pub fn new(db: &dyn ValidTermDb, toolchain: Toolchain) -> Self {
        // let sort = db.it_valid_term(ValidTermAtom::new_category(ValidTermCategory::Sort).into());
        // let universe1 = db.it_valid_term(ValidTermAtom::new_universe(1).into());
        let vfs_path_menu = db.vfs_path_menu(toolchain).unwrap();
        let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
        let universe0 = ValidTermUniverse::new(0);
        let universe1 = ValidTermUniverse::new(1);
        ValidTermMenu0 {
            eval_lifetime: ValidTermLiteral::EvalLifetime,
            static_lifetime: ValidTermLiteral::StaticLifetime,
            universe0,
            universe1,
            prop: ValidTermCategory::new(universe0),
            ty0: ValidTermCategory::new(universe1),
            core_ops_add: ValidTerm::EntityPath(entity_path_menu.core_ops_add().into()),
            // start here
            // ValidTerm::Entity(entity_path_menu.core_ops_())
            core_ops_add_assign: ValidTerm::EntityPath(
                entity_path_menu.core_ops_add_assign().into(),
            ),
            core_ops_bit_and: ValidTerm::EntityPath(entity_path_menu.core_ops_bit_and().into()),
            core_ops_bit_and_assign: ValidTerm::EntityPath(
                entity_path_menu.core_ops_bit_and_assign().into(),
            ),
            core_ops_bit_or: ValidTerm::EntityPath(entity_path_menu.core_ops_bit_or().into()),
            core_ops_bit_or_assign: ValidTerm::EntityPath(
                entity_path_menu.core_ops_bit_or_assign().into(),
            ),
            core_ops_bit_xor: ValidTerm::EntityPath(entity_path_menu.core_ops_bit_xor().into()),
            core_ops_bit_xor_assign: ValidTerm::EntityPath(
                entity_path_menu.core_ops_bit_xor_assign().into(),
            ),
            core_ops_div: ValidTerm::EntityPath(entity_path_menu.core_ops_div().into()),
            core_ops_div_assign: ValidTerm::EntityPath(
                entity_path_menu.core_ops_div_assign().into(),
            ),
            core_ops_mul: ValidTerm::EntityPath(entity_path_menu.core_ops_mul().into()),
            core_ops_mul_assign: ValidTerm::EntityPath(
                entity_path_menu.core_ops_mul_assign().into(),
            ),
            core_ops_neg: ValidTerm::EntityPath(entity_path_menu.core_ops_neg().into()),
            core_ops_not: ValidTerm::EntityPath(entity_path_menu.core_ops_not().into()),
            option_ty_path: ValidTermEntityPath::TypeOntology(entity_path_menu.option_ty_path())
                .into(),
            slice_ty_path: ValidTermEntityPath::TypeOntology(entity_path_menu.slice_ty_path())
                .into(),
            str_ty_path: ValidTermEntityPath::TypeOntology(entity_path_menu.str_ty_path()).into(),
            ref_ty_path: ValidTermEntityPath::TypeOntology(entity_path_menu.ref_ty_path()).into(),
            list_ty: ValidTermEntityPath::TypeOntology(entity_path_menu.list_ty_path()).into(),
            unit: ValidTermEntityPath::TypeOntology(entity_path_menu.unit()).into(),
            never: ValidTermEntityPath::TypeOntology(entity_path_menu.never()).into(),
            bool: ValidTermEntityPath::TypeOntology(entity_path_menu.bool_ty_path()).into(),
            trai_ty: ValidTermEntityPath::TypeOntology(entity_path_menu.trai_ty_path()).into(),
            lifetime_ty: ValidTermEntityPath::TypeOntology(entity_path_menu.lifetime_ty_path())
                .into(),
            module: ValidTermEntityPath::TypeOntology(entity_path_menu.module_ty_path()).into(),
            i8: ValidTermEntityPath::TypeOntology(entity_path_menu.i8_ty_path()).into(),
            i16: ValidTermEntityPath::TypeOntology(entity_path_menu.i16_ty_path()).into(),
            i32: ValidTermEntityPath::TypeOntology(entity_path_menu.i32_ty_path()).into(),
            i64: ValidTermEntityPath::TypeOntology(entity_path_menu.i64_ty_path()).into(),
            f32: ValidTermEntityPath::TypeOntology(entity_path_menu.f32_ty_path()).into(),
            f64: ValidTermEntityPath::TypeOntology(entity_path_menu.f64_ty_path()).into(),
            r32: ValidTermEntityPath::TypeOntology(entity_path_menu.r32_ty_path()).into(),
            r64: ValidTermEntityPath::TypeOntology(entity_path_menu.r64_ty_path()).into(),
        }
    }

    pub fn universe0(&self) -> ValidTermUniverse {
        self.universe0
    }

    pub fn universe1(&self) -> ValidTermUniverse {
        self.universe1
    }

    /// `Prop`
    pub fn prop(&self) -> ValidTermCategory {
        self.prop
    }

    /// `Type`
    pub fn ty0(&self) -> ValidTermCategory {
        self.ty0
    }

    // pub fn core(&self) -> ValidTerm {
    //     self.core
    // }

    // pub fn core_ops(&self) -> ValidTerm {
    //     self.core_ops
    // }

    // Add	The addition operator +.
    pub fn core_ops_add(&self) -> ValidTerm {
        self.core_ops_add
    }

    // AddAssign	The addition assignment operator +=.
    pub fn core_ops_add_assign(&self) -> ValidTerm {
        self.core_ops_add_assign
    }

    // BitAnd	The bitwise AND operator &.
    pub fn core_ops_bit_and(&self) -> ValidTerm {
        self.core_ops_bit_and
    }

    // BitAndAssign	The bitwise AND assignment operator &=.
    pub fn core_ops_bit_and_assign(&self) -> ValidTerm {
        self.core_ops_bit_and_assign
    }

    // BitOr	The bitwise OR operator |.
    pub fn core_ops_bit_or(&self) -> ValidTerm {
        self.core_ops_bit_or
    }

    // BitOrAssign	The bitwise OR assignment operator |=.
    pub fn core_ops_bit_or_assign(&self) -> ValidTerm {
        self.core_ops_bit_or_assign
    }
    // BitXor	The bitwise XOR operator ^.
    pub fn core_ops_bit_xor(&self) -> ValidTerm {
        self.core_ops_bit_xor
    }

    // BitXorAssign	The bitwise XOR assignment operator ^=.
    pub fn core_ops_bit_xor_assign(&self) -> ValidTerm {
        self.core_ops_bit_or_assign
    }

    // Div	The division operator /.
    pub fn core_ops_div(&self) -> ValidTerm {
        self.core_ops_div
    }

    // DivAssign	The division assignment operator /=.
    pub fn core_ops_div_assign(&self) -> ValidTerm {
        self.core_ops_div_assign
    }

    // Mul	The multiplication operator *.
    pub fn core_ops_mul(&self) -> ValidTerm {
        self.core_ops_mul
    }

    // MulAssign	The multiplication assignment operator *=.
    pub fn core_ops_mul_assign(&self) -> ValidTerm {
        self.core_ops_mul_assign
    }

    // Neg	The unary negation operator -.
    pub fn core_ops_neg(&self) -> ValidTerm {
        self.core_ops_neg
    }

    // Not	The unary logical negation operator !.
    pub fn core_ops_not(&self) -> ValidTerm {
        self.core_ops_not
    }

    pub fn option_ty_path(&self) -> ValidTerm {
        self.option_ty_path
    }

    pub fn slice_ty_path(&self) -> ValidTerm {
        self.slice_ty_path
    }

    pub fn ref_ty_path(&self) -> ValidTerm {
        self.ref_ty_path
    }

    pub fn list(&self) -> ValidTerm {
        self.list_ty
    }

    pub fn unit(&self) -> ValidTerm {
        self.unit
    }

    pub fn never(&self) -> ValidTerm {
        self.never
    }

    pub fn trai_ty(&self) -> ValidTerm {
        self.trai_ty
    }

    pub fn module(&self) -> ValidTerm {
        self.module
    }

    pub fn bool(&self) -> ValidTerm {
        self.bool
    }

    pub fn i8(&self) -> ValidTerm {
        self.i8
    }

    pub fn i16(&self) -> ValidTerm {
        self.i16
    }

    pub fn i32(&self) -> ValidTerm {
        self.i32
    }

    pub fn i64(&self) -> ValidTerm {
        self.i64
    }

    pub fn f32(&self) -> ValidTerm {
        self.f32
    }

    pub fn f64(&self) -> ValidTerm {
        self.f64
    }

    pub fn r32(&self) -> ValidTerm {
        self.r32
    }

    pub fn r64(&self) -> ValidTerm {
        self.r64
    }

    pub fn eval_lifetime(&self) -> ValidTerm {
        self.eval_lifetime.into()
    }

    pub fn static_lifetime(&self) -> ValidTerm {
        self.static_lifetime.into()
    }

    /// Lifetime
    pub fn lifetime_ty(&self) -> ValidTerm {
        self.lifetime_ty
    }

    pub fn str_ty_path(&self) -> ValidTerm {
        self.str_ty_path
    }
}
