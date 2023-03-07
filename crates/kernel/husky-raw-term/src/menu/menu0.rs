use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct RawTermMenu0 {
    universe0: TermUniverse,
    universe1: TermUniverse,
    prop: TermCategory,
    ty0: TermCategory,
    eval_lifetime: RawTermLiteral,
    static_lifetime: RawTermLiteral,
    unit: RawTerm,
    never: RawTerm,
    // core::ops::Add	The addition operator +.
    core_ops_add: RawTerm,
    // core::ops::AddAssign	The addition assignment operator +=.
    core_ops_add_assign: RawTerm,
    // core::ops::BitAnd	The bitwise AND operator &.
    core_ops_bit_and: RawTerm,
    // core::ops::BitAndAssign	The bitwise AND assignment operator &=.
    core_ops_bit_and_assign: RawTerm,
    // core::ops::BitOr	The bitwise OR operator |.
    core_ops_bit_or: RawTerm,
    // core::ops::BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign: RawTerm,
    // core::ops::BitXor	The bitwise XOR operator ^.
    core_ops_bit_xor: RawTerm,
    // core::ops::BitXorAssign	The bitwise XOR assignment operator ^=.
    core_ops_bit_xor_assign: RawTerm,
    // core::ops::Div	The division operator /.
    core_ops_div: RawTerm,
    // core::ops::DivAssign	The division assignment operator /=.
    core_ops_div_assign: RawTerm,
    // core::ops::Mul	The multiplication operator *.
    core_ops_mul: RawTerm,
    // core::ops::MulAssign	The multiplication assignment operator *=.
    core_ops_mul_assign: RawTerm,
    // core::ops::Neg	The unary negation operator -.
    core_ops_neg: RawTerm,
    // Not	The unary logical negation operator !.
    core_ops_not: RawTerm,
    option_ty_path: RawTerm,
    slice_ty_path: RawTerm,
    str_ty_path: RawTerm,
    ref_ty_path: RawTerm,
    leash_ty_path: RawTermEntityPath,
    list_ty: RawTerm,
    i8: RawTerm,
    i16: RawTerm,
    i32: RawTerm,
    i64: RawTerm,
    f32: RawTerm,
    f64: RawTerm,
    r32: RawTerm,
    r64: RawTerm,
    bool: RawTerm,
    lifetime_ty: RawTerm,
    trai_ty: RawTerm,
    module: RawTerm,
}

impl RawTermMenu0 {
    pub fn new(db: &dyn RawTermDb, toolchain: Toolchain) -> Self {
        // let sort = db.it_term(RawTermAtom::new_category(TermCategory::Sort).into());
        // let universe1 = db.it_term(RawTermAtom::new_universe(1).into());
        let vfs_path_menu = db.vfs_path_menu(toolchain).unwrap();
        let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
        let universe0 = TermUniverse::new(0);
        let universe1 = TermUniverse::new(1);
        RawTermMenu0 {
            eval_lifetime: TermLiteral::EvalLifetime.into(),
            static_lifetime: TermLiteral::StaticLifetime.into(),
            universe0,
            universe1,
            prop: TermCategory::new(universe0),
            ty0: TermCategory::new(universe1),
            core_ops_add: RawTerm::EntityPath(entity_path_menu.core_ops_add().into()),
            // start here
            // RawTerm::Entity(entity_path_menu.core_ops_())
            core_ops_add_assign: RawTerm::EntityPath(entity_path_menu.core_ops_add_assign().into()),
            core_ops_bit_and: RawTerm::EntityPath(entity_path_menu.core_ops_bit_and().into()),
            core_ops_bit_and_assign: RawTerm::EntityPath(
                entity_path_menu.core_ops_bit_and_assign().into(),
            ),
            core_ops_bit_or: RawTerm::EntityPath(entity_path_menu.core_ops_bit_or().into()),
            core_ops_bit_or_assign: RawTerm::EntityPath(
                entity_path_menu.core_ops_bit_or_assign().into(),
            ),
            core_ops_bit_xor: RawTerm::EntityPath(entity_path_menu.core_ops_bit_xor().into()),
            core_ops_bit_xor_assign: RawTerm::EntityPath(
                entity_path_menu.core_ops_bit_xor_assign().into(),
            ),
            core_ops_div: RawTerm::EntityPath(entity_path_menu.core_ops_div().into()),
            core_ops_div_assign: RawTerm::EntityPath(entity_path_menu.core_ops_div_assign().into()),
            core_ops_mul: RawTerm::EntityPath(entity_path_menu.core_ops_mul().into()),
            core_ops_mul_assign: RawTerm::EntityPath(entity_path_menu.core_ops_mul_assign().into()),
            core_ops_neg: RawTerm::EntityPath(entity_path_menu.core_ops_neg().into()),
            core_ops_not: RawTerm::EntityPath(entity_path_menu.core_ops_not().into()),
            option_ty_path: RawTermEntityPath::Type(entity_path_menu.option_ty_path()).into(),
            slice_ty_path: RawTermEntityPath::Type(entity_path_menu.slice_ty_path()).into(),
            str_ty_path: RawTermEntityPath::Type(entity_path_menu.str_ty_path()).into(),
            ref_ty_path: RawTermEntityPath::Type(entity_path_menu.ref_ty_path()).into(),
            list_ty: RawTermEntityPath::Type(entity_path_menu.list_ty_path()).into(),
            unit: RawTermEntityPath::Type(entity_path_menu.unit_ty_path()).into(),
            never: RawTermEntityPath::Type(entity_path_menu.never_ty_path()).into(),
            bool: RawTermEntityPath::Type(entity_path_menu.bool_ty_path()).into(),
            trai_ty: RawTermEntityPath::Type(entity_path_menu.trai_ty_path()).into(),
            leash_ty_path: entity_path_menu.leash_ty_path().into(),
            lifetime_ty: RawTermEntityPath::Type(entity_path_menu.lifetime_ty_path()).into(),
            module: RawTermEntityPath::Type(entity_path_menu.module_ty_path()).into(),
            i8: RawTermEntityPath::Type(entity_path_menu.i8_ty_path()).into(),
            i16: RawTermEntityPath::Type(entity_path_menu.i16_ty_path()).into(),
            i32: RawTermEntityPath::Type(entity_path_menu.i32_ty_path()).into(),
            i64: RawTermEntityPath::Type(entity_path_menu.i64_ty_path()).into(),
            f32: RawTermEntityPath::Type(entity_path_menu.f32_ty_path()).into(),
            f64: RawTermEntityPath::Type(entity_path_menu.f64_ty_path()).into(),
            r32: RawTermEntityPath::Type(entity_path_menu.r32_ty_path()).into(),
            r64: RawTermEntityPath::Type(entity_path_menu.r64_ty_path()).into(),
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

    // pub fn core(&self) -> RawTerm {
    //     self.core
    // }

    // pub fn core_ops(&self) -> RawTerm {
    //     self.core_ops
    // }

    // Add	The addition operator +.
    pub fn core_ops_add(&self) -> RawTerm {
        self.core_ops_add
    }

    // AddAssign	The addition assignment operator +=.
    pub fn core_ops_add_assign(&self) -> RawTerm {
        self.core_ops_add_assign
    }

    // BitAnd	The bitwise AND operator &.
    pub fn core_ops_bit_and(&self) -> RawTerm {
        self.core_ops_bit_and
    }

    // BitAndAssign	The bitwise AND assignment operator &=.
    pub fn core_ops_bit_and_assign(&self) -> RawTerm {
        self.core_ops_bit_and_assign
    }

    // BitOr	The bitwise OR operator |.
    pub fn core_ops_bit_or(&self) -> RawTerm {
        self.core_ops_bit_or
    }

    // BitOrAssign	The bitwise OR assignment operator |=.
    pub fn core_ops_bit_or_assign(&self) -> RawTerm {
        self.core_ops_bit_or_assign
    }
    // BitXor	The bitwise XOR operator ^.
    pub fn core_ops_bit_xor(&self) -> RawTerm {
        self.core_ops_bit_xor
    }

    // BitXorAssign	The bitwise XOR assignment operator ^=.
    pub fn core_ops_bit_xor_assign(&self) -> RawTerm {
        self.core_ops_bit_or_assign
    }

    // Div	The division operator /.
    pub fn core_ops_div(&self) -> RawTerm {
        self.core_ops_div
    }

    // DivAssign	The division assignment operator /=.
    pub fn core_ops_div_assign(&self) -> RawTerm {
        self.core_ops_div_assign
    }

    // Mul	The multiplication operator *.
    pub fn core_ops_mul(&self) -> RawTerm {
        self.core_ops_mul
    }

    // MulAssign	The multiplication assignment operator *=.
    pub fn core_ops_mul_assign(&self) -> RawTerm {
        self.core_ops_mul_assign
    }

    // Neg	The unary negation operator -.
    pub fn core_ops_neg(&self) -> RawTerm {
        self.core_ops_neg
    }

    // Not	The unary logical negation operator !.
    pub fn core_ops_not(&self) -> RawTerm {
        self.core_ops_not
    }

    pub fn option_ty_path(&self) -> RawTerm {
        self.option_ty_path
    }

    pub fn slice_ty_path(&self) -> RawTerm {
        self.slice_ty_path
    }

    pub fn ref_ty_path(&self) -> RawTerm {
        self.ref_ty_path
    }

    pub fn leash_ty_path(&self) -> RawTermEntityPath {
        self.leash_ty_path
    }

    pub fn list(&self) -> RawTerm {
        self.list_ty
    }

    pub fn unit(&self) -> RawTerm {
        self.unit
    }

    pub fn never(&self) -> RawTerm {
        self.never
    }

    pub fn trai_ty(&self) -> RawTerm {
        self.trai_ty
    }

    pub fn module(&self) -> RawTerm {
        self.module
    }

    pub fn bool(&self) -> RawTerm {
        self.bool
    }

    pub fn i8(&self) -> RawTerm {
        self.i8
    }

    pub fn i16(&self) -> RawTerm {
        self.i16
    }

    pub fn i32(&self) -> RawTerm {
        self.i32
    }

    pub fn i64(&self) -> RawTerm {
        self.i64
    }

    pub fn f32(&self) -> RawTerm {
        self.f32
    }

    pub fn f64(&self) -> RawTerm {
        self.f64
    }

    pub fn r32(&self) -> RawTerm {
        self.r32
    }

    pub fn r64(&self) -> RawTerm {
        self.r64
    }

    pub fn eval_lifetime(&self) -> RawTerm {
        self.eval_lifetime.into()
    }

    pub fn static_lifetime(&self) -> RawTerm {
        self.static_lifetime.into()
    }

    /// Lifetime
    pub fn lifetime_ty(&self) -> RawTerm {
        self.lifetime_ty
    }

    pub fn str_ty_path(&self) -> RawTerm {
        self.str_ty_path
    }
}
