use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclarativeTermMenu0 {
    universe0: TermUniverse,
    universe1: TermUniverse,
    prop: TermCategory,
    ty0: TermCategory,
    static_lifetime: DeclarativeTermLiteral,
    unit: DeclarativeTerm,
    never: DeclarativeTerm,
    // core::ops::Add	The addition operator +.
    core_ops_add: DeclarativeTerm,
    // core::ops::AddAssign	The addition assignment operator +=.
    core_ops_add_assign: DeclarativeTerm,
    // core::ops::BitAnd	The bitwise AND operator &.
    core_ops_bit_and: DeclarativeTerm,
    // core::ops::BitAndAssign	The bitwise AND assignment operator &=.
    core_ops_bit_and_assign: DeclarativeTerm,
    // core::ops::BitOr	The bitwise OR operator |.
    core_ops_bit_or: DeclarativeTerm,
    // core::ops::BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign: DeclarativeTerm,
    // core::ops::BitXor	The bitwise XOR operator ^.
    core_ops_bit_xor: DeclarativeTerm,
    // core::ops::BitXorAssign	The bitwise XOR assignment operator ^=.
    core_ops_bit_xor_assign: DeclarativeTerm,
    // core::ops::Div	The division operator /.
    core_ops_div: DeclarativeTerm,
    // core::ops::DivAssign	The division assignment operator /=.
    core_ops_div_assign: DeclarativeTerm,
    // core::ops::Mul	The multiplication operator *.
    core_ops_mul: DeclarativeTerm,
    // core::ops::MulAssign	The multiplication assignment operator *=.
    core_ops_mul_assign: DeclarativeTerm,
    // core::ops::Neg	The unary negation operator -.
    core_ops_neg: DeclarativeTerm,
    // Not	The unary logical negation operator !.
    core_ops_not: DeclarativeTerm,
    option_ty_path: DeclarativeTerm,
    slice_ty_path: DeclarativeTerm,
    str_ty_path: DeclarativeTerm,
    ref_ty_path: DeclarativeTerm,
    leash_ty_path: DeclarativeTermEntityPath,
    list_ty: DeclarativeTerm,
    i8: DeclarativeTerm,
    i16: DeclarativeTerm,
    i32: DeclarativeTerm,
    i64: DeclarativeTerm,
    f32: DeclarativeTerm,
    f64: DeclarativeTerm,
    r32: DeclarativeTerm,
    r64: DeclarativeTerm,
    bool: DeclarativeTerm,
    lifetime_ty: DeclarativeTerm,
    trai_ty: DeclarativeTerm,
    module: DeclarativeTerm,
}

impl DeclarativeTermMenu0 {
    pub fn new(db: &dyn DeclarativeTermDb, toolchain: Toolchain) -> Self {
        // let sort = db.it_term(DeclarativeTermAtom::new_category(TermCategory::Sort).into());
        // let universe1 = db.it_term(DeclarativeTermAtom::new_universe(1).into());
        let _vfs_path_menu = db.vfs_path_menu(toolchain);
        let item_path_menu = db.item_path_menu(toolchain);
        let universe0 = TermUniverse::new(0);
        let universe1 = TermUniverse::new(1);
        DeclarativeTermMenu0 {
            static_lifetime: TermLiteral::StaticLifetime.into(),
            universe0,
            universe1,
            prop: TermCategory::new(universe0),
            ty0: TermCategory::new(universe1),
            core_ops_add: DeclarativeTerm::EntityPath(item_path_menu.add_trai_path().into()),
            // start here
            // DeclarativeTerm::Entity(item_path_menu.core_ops_())
            core_ops_add_assign: DeclarativeTerm::EntityPath(
                item_path_menu.add_assign_trai_path().into(),
            ),
            core_ops_bit_and: DeclarativeTerm::EntityPath(
                item_path_menu.bit_and_trai_path().into(),
            ),
            core_ops_bit_and_assign: DeclarativeTerm::EntityPath(
                item_path_menu.bit_and_assign_trai_path().into(),
            ),
            core_ops_bit_or: DeclarativeTerm::EntityPath(item_path_menu.bit_or_trai_path().into()),
            core_ops_bit_or_assign: DeclarativeTerm::EntityPath(
                item_path_menu.core_ops_bit_or_assign_trai_path().into(),
            ),
            core_ops_bit_xor: DeclarativeTerm::EntityPath(
                item_path_menu.bit_xor_trai_path().into(),
            ),
            core_ops_bit_xor_assign: DeclarativeTerm::EntityPath(
                item_path_menu.bit_xor_assign_trai_path().into(),
            ),
            core_ops_div: DeclarativeTerm::EntityPath(item_path_menu.div_trai_path().into()),
            core_ops_div_assign: DeclarativeTerm::EntityPath(
                item_path_menu.div_assign_trai_path().into(),
            ),
            core_ops_mul: DeclarativeTerm::EntityPath(item_path_menu.mul_trai_path().into()),
            core_ops_mul_assign: DeclarativeTerm::EntityPath(
                item_path_menu.mul_assign_trai_path().into(),
            ),
            core_ops_neg: DeclarativeTerm::EntityPath(item_path_menu.neg_trai_path().into()),
            core_ops_not: DeclarativeTerm::EntityPath(item_path_menu.not_trai_path().into()),
            option_ty_path: DeclarativeTermEntityPath::Type(item_path_menu.option_ty_path()).into(),
            slice_ty_path: DeclarativeTermEntityPath::Type(item_path_menu.slice_ty_path()).into(),
            str_ty_path: DeclarativeTermEntityPath::Type(item_path_menu.str_ty_path()).into(),
            ref_ty_path: DeclarativeTermEntityPath::Type(item_path_menu.ref_ty_path()).into(),
            list_ty: DeclarativeTermEntityPath::Type(item_path_menu.list_ty_path()).into(),
            unit: DeclarativeTermEntityPath::Type(item_path_menu.unit_ty_path()).into(),
            never: DeclarativeTermEntityPath::Type(item_path_menu.never_ty_path()).into(),
            bool: DeclarativeTermEntityPath::Type(item_path_menu.bool_ty_path()).into(),
            trai_ty: DeclarativeTermEntityPath::Type(item_path_menu.trai_ty_path()).into(),
            leash_ty_path: item_path_menu.leash_ty_path().into(),
            lifetime_ty: DeclarativeTermEntityPath::Type(item_path_menu.lifetime_ty_path()).into(),
            module: DeclarativeTermEntityPath::Type(item_path_menu.module_ty_path()).into(),
            i8: DeclarativeTermEntityPath::Type(item_path_menu.i8_ty_path()).into(),
            i16: DeclarativeTermEntityPath::Type(item_path_menu.i16_ty_path()).into(),
            i32: DeclarativeTermEntityPath::Type(item_path_menu.i32_ty_path()).into(),
            i64: DeclarativeTermEntityPath::Type(item_path_menu.i64_ty_path()).into(),
            f32: DeclarativeTermEntityPath::Type(item_path_menu.f32_ty_path()).into(),
            f64: DeclarativeTermEntityPath::Type(item_path_menu.f64_ty_path()).into(),
            r32: DeclarativeTermEntityPath::Type(item_path_menu.r32_ty_path()).into(),
            r64: DeclarativeTermEntityPath::Type(item_path_menu.r64_ty_path()).into(),
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

    // pub fn core(&self) -> DeclarativeTerm {
    //     self.core
    // }

    // pub fn core_ops(&self) -> DeclarativeTerm {
    //     self.core_ops
    // }

    // Add	The addition operator +.
    pub fn core_ops_add(&self) -> DeclarativeTerm {
        self.core_ops_add
    }

    // AddAssign	The addition assignment operator +=.
    pub fn core_ops_add_assign(&self) -> DeclarativeTerm {
        self.core_ops_add_assign
    }

    // BitAnd	The bitwise AND operator &.
    pub fn core_ops_bit_and(&self) -> DeclarativeTerm {
        self.core_ops_bit_and
    }

    // BitAndAssign	The bitwise AND assignment operator &=.
    pub fn core_ops_bit_and_assign(&self) -> DeclarativeTerm {
        self.core_ops_bit_and_assign
    }

    // BitOr	The bitwise OR operator |.
    pub fn core_ops_bit_or(&self) -> DeclarativeTerm {
        self.core_ops_bit_or
    }

    // BitOrAssign	The bitwise OR assignment operator |=.
    pub fn core_ops_bit_or_assign(&self) -> DeclarativeTerm {
        self.core_ops_bit_or_assign
    }
    // BitXor	The bitwise XOR operator ^.
    pub fn core_ops_bit_xor(&self) -> DeclarativeTerm {
        self.core_ops_bit_xor
    }

    // BitXorAssign	The bitwise XOR assignment operator ^=.
    pub fn core_ops_bit_xor_assign(&self) -> DeclarativeTerm {
        self.core_ops_bit_or_assign
    }

    // Div	The division operator /.
    pub fn core_ops_div(&self) -> DeclarativeTerm {
        self.core_ops_div
    }

    // DivAssign	The division assignment operator /=.
    pub fn core_ops_div_assign(&self) -> DeclarativeTerm {
        self.core_ops_div_assign
    }

    // Mul	The multiplication operator *.
    pub fn core_ops_mul(&self) -> DeclarativeTerm {
        self.core_ops_mul
    }

    // MulAssign	The multiplication assignment operator *=.
    pub fn core_ops_mul_assign(&self) -> DeclarativeTerm {
        self.core_ops_mul_assign
    }

    // Neg	The unary negation operator -.
    pub fn core_ops_neg(&self) -> DeclarativeTerm {
        self.core_ops_neg
    }

    // Not	The unary logical negation operator !.
    pub fn core_ops_not(&self) -> DeclarativeTerm {
        self.core_ops_not
    }

    pub fn option_ty_path(&self) -> DeclarativeTerm {
        self.option_ty_path
    }

    pub fn slice_ty_path(&self) -> DeclarativeTerm {
        self.slice_ty_path
    }

    pub fn ref_ty_path(&self) -> DeclarativeTerm {
        self.ref_ty_path
    }

    pub fn leash_ty_path(&self) -> DeclarativeTermEntityPath {
        self.leash_ty_path
    }

    pub fn list(&self) -> DeclarativeTerm {
        self.list_ty
    }

    pub fn unit(&self) -> DeclarativeTerm {
        self.unit
    }

    pub fn never(&self) -> DeclarativeTerm {
        self.never
    }

    pub fn trai_ty(&self) -> DeclarativeTerm {
        self.trai_ty
    }

    pub fn module(&self) -> DeclarativeTerm {
        self.module
    }

    pub fn bool(&self) -> DeclarativeTerm {
        self.bool
    }

    pub fn i8(&self) -> DeclarativeTerm {
        self.i8
    }

    pub fn i16(&self) -> DeclarativeTerm {
        self.i16
    }

    pub fn i32(&self) -> DeclarativeTerm {
        self.i32
    }

    pub fn i64(&self) -> DeclarativeTerm {
        self.i64
    }

    pub fn f32(&self) -> DeclarativeTerm {
        self.f32
    }

    pub fn f64(&self) -> DeclarativeTerm {
        self.f64
    }

    pub fn r32(&self) -> DeclarativeTerm {
        self.r32
    }

    pub fn r64(&self) -> DeclarativeTerm {
        self.r64
    }

    pub fn static_lifetime(&self) -> DeclarativeTerm {
        self.static_lifetime.into()
    }

    /// Lifetime
    pub fn lifetime_ty(&self) -> DeclarativeTerm {
        self.lifetime_ty
    }

    pub fn str_ty_path(&self) -> DeclarativeTerm {
        self.str_ty_path
    }
}
