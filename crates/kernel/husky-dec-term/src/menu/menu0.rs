use husky_entity_path::menu::item_path_menu;
use husky_term_prelude::literal::Literal;
use husky_vfs::VfsDb;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DecTermMenu0 {
    universe0: Universe,
    universe1: Universe,
    prop: Sort,
    ty0: Sort,
    static_lifetime: DecLiteral,
    unit: DecTerm,
    never: DecTerm,
    // core::ops::Add	The addition operator +.
    core_ops_add: DecTerm,
    // core::ops::AddAssign	The addition assignment operator +=.
    core_ops_add_assign: DecTerm,
    // core::ops::BitAnd	The bitwise AND operator &.
    core_ops_bit_and: DecTerm,
    // core::ops::BitAndAssign	The bitwise AND assignment operator &=.
    core_ops_bit_and_assign: DecTerm,
    // core::ops::BitOr	The bitwise OR operator |.
    core_ops_bit_or: DecTerm,
    // core::ops::BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign: DecTerm,
    // core::ops::BitXor	The bitwise XOR operator ^.
    core_ops_bit_xor: DecTerm,
    // core::ops::BitXorAssign	The bitwise XOR assignment operator ^=.
    core_ops_bit_xor_assign: DecTerm,
    // core::ops::Div	The division operator /.
    core_ops_div: DecTerm,
    // core::ops::DivAssign	The division assignment operator /=.
    core_ops_div_assign: DecTerm,
    // core::ops::Mul	The multiplication operator *.
    core_ops_mul: DecTerm,
    // core::ops::MulAssign	The multiplication assignment operator *=.
    core_ops_mul_assign: DecTerm,
    // core::ops::Neg	The unary negation operator -.
    core_ops_neg: DecTerm,
    // Not	The unary logical negation operator !.
    core_ops_not: DecTerm,
    option_ty_path: DecTerm,
    slice_ty_path: DecTerm,
    str_ty_path: DecTerm,
    ref_ty_path: DecTerm,
    at_ty_path: DecTerm,
    leash_ty_path: DecItemPath,
    vec_ty: DecTerm,
    i8: DecTerm,
    i16: DecTerm,
    i32: DecTerm,
    i64: DecTerm,
    f32: DecTerm,
    f64: DecTerm,
    r32: DecTerm,
    r64: DecTerm,
    bool: DecTerm,
    lifetime_ty: DecTerm,
    place_ty: DecTerm,
    trai_ty: DecTerm,
    module: DecTerm,
}

impl DecTermMenu0 {
    pub fn new(db: &::salsa::Db, toolchain: Toolchain) -> Self {
        // let sort = db.it_term(DecTermAtom::new_category(TermCategory::Sort).into());
        // let universe1 = db.it_term(DecTermAtom::new_universe(1).into());
        let _vfs_path_menu = db.vfs_path_menu(toolchain);
        let item_path_menu = item_path_menu(db, toolchain);
        let universe0 = Universe::new(0);
        let universe1 = Universe::new(1);
        DecTermMenu0 {
            static_lifetime: Literal::StaticLifetime.into(),
            universe0,
            universe1,
            prop: Sort::new(universe0),
            ty0: Sort::new(universe1),
            core_ops_add: DecTerm::EntityPath(item_path_menu.add_trai_path().into()),
            // start here
            // DecTerm::Entity(item_path_menu.core_ops_())
            core_ops_add_assign: DecTerm::EntityPath(item_path_menu.add_assign_trai_path().into()),
            core_ops_bit_and: DecTerm::EntityPath(item_path_menu.bit_and_trai_path().into()),
            core_ops_bit_and_assign: DecTerm::EntityPath(
                item_path_menu.bit_and_assign_trai_path().into(),
            ),
            core_ops_bit_or: DecTerm::EntityPath(item_path_menu.bit_or_trai_path().into()),
            core_ops_bit_or_assign: DecTerm::EntityPath(
                item_path_menu.bit_or_assign_trai_path().into(),
            ),
            core_ops_bit_xor: DecTerm::EntityPath(item_path_menu.bit_xor_trai_path().into()),
            core_ops_bit_xor_assign: DecTerm::EntityPath(
                item_path_menu.bit_xor_assign_trai_path().into(),
            ),
            core_ops_div: DecTerm::EntityPath(item_path_menu.div_trai_path().into()),
            core_ops_div_assign: DecTerm::EntityPath(item_path_menu.div_assign_trai_path().into()),
            core_ops_mul: DecTerm::EntityPath(item_path_menu.mul_trai_path().into()),
            core_ops_mul_assign: DecTerm::EntityPath(item_path_menu.mul_assign_trai_path().into()),
            core_ops_neg: DecTerm::EntityPath(item_path_menu.neg_trai_path().into()),
            core_ops_not: DecTerm::EntityPath(item_path_menu.not_trai_path().into()),
            option_ty_path: DecItemPath::Type(item_path_menu.option_ty_path()).into(),
            slice_ty_path: DecItemPath::Type(item_path_menu.slice_ty_path()).into(),
            str_ty_path: DecItemPath::Type(item_path_menu.str_ty_path()).into(),
            ref_ty_path: item_path_menu.ref_ty_path().into(),
            at_ty_path: item_path_menu.at_ty_path().into(),
            vec_ty: DecItemPath::Type(item_path_menu.vec_ty_path()).into(),
            unit: DecItemPath::Type(item_path_menu.unit_ty_path()).into(),
            never: DecItemPath::Type(item_path_menu.never_ty_path()).into(),
            bool: DecItemPath::Type(item_path_menu.bool_ty_path()).into(),
            trai_ty: DecItemPath::Type(item_path_menu.trai_ty_path()).into(),
            leash_ty_path: item_path_menu.leash_ty_path().into(),
            lifetime_ty: DecItemPath::Type(item_path_menu.lifetime_ty_path()).into(),
            place_ty: DecItemPath::Type(item_path_menu.place_ty_path()).into(),
            module: DecItemPath::Type(item_path_menu.module_ty_path()).into(),
            i8: DecItemPath::Type(item_path_menu.i8_ty_path()).into(),
            i16: DecItemPath::Type(item_path_menu.i16_ty_path()).into(),
            i32: DecItemPath::Type(item_path_menu.i32_ty_path()).into(),
            i64: DecItemPath::Type(item_path_menu.i64_ty_path()).into(),
            f32: DecItemPath::Type(item_path_menu.f32_ty_path()).into(),
            f64: DecItemPath::Type(item_path_menu.f64_ty_path()).into(),
            r32: DecItemPath::Type(item_path_menu.r32_ty_path()).into(),
            r64: DecItemPath::Type(item_path_menu.r64_ty_path()).into(),
        }
    }

    pub fn universe0(&self) -> Universe {
        self.universe0
    }

    pub fn universe1(&self) -> Universe {
        self.universe1
    }

    /// `Prop`
    pub fn prop(&self) -> Sort {
        self.prop
    }

    /// `Type`
    pub fn ty0(&self) -> Sort {
        self.ty0
    }

    // pub fn core(&self) -> DecTerm {
    //     self.core
    // }

    // pub fn core_ops(&self) -> DecTerm {
    //     self.core_ops
    // }

    // Add	The addition operator +.
    pub fn core_ops_add(&self) -> DecTerm {
        self.core_ops_add
    }

    // AddAssign	The addition assignment operator +=.
    pub fn core_ops_add_assign(&self) -> DecTerm {
        self.core_ops_add_assign
    }

    // BitAnd	The bitwise AND operator &.
    pub fn core_ops_bit_and(&self) -> DecTerm {
        self.core_ops_bit_and
    }

    // BitAndAssign	The bitwise AND assignment operator &=.
    pub fn core_ops_bit_and_assign(&self) -> DecTerm {
        self.core_ops_bit_and_assign
    }

    // BitOr	The bitwise OR operator |.
    pub fn core_ops_bit_or(&self) -> DecTerm {
        self.core_ops_bit_or
    }

    // BitOrAssign	The bitwise OR assignment operator |=.
    pub fn core_ops_bit_or_assign(&self) -> DecTerm {
        self.core_ops_bit_or_assign
    }
    // BitXor	The bitwise XOR operator ^.
    pub fn core_ops_bit_xor(&self) -> DecTerm {
        self.core_ops_bit_xor
    }

    // BitXorAssign	The bitwise XOR assignment operator ^=.
    pub fn core_ops_bit_xor_assign(&self) -> DecTerm {
        self.core_ops_bit_or_assign
    }

    // Div	The division operator /.
    pub fn core_ops_div(&self) -> DecTerm {
        self.core_ops_div
    }

    // DivAssign	The division assignment operator /=.
    pub fn core_ops_div_assign(&self) -> DecTerm {
        self.core_ops_div_assign
    }

    // Mul	The multiplication operator *.
    pub fn core_ops_mul(&self) -> DecTerm {
        self.core_ops_mul
    }

    // MulAssign	The multiplication assignment operator *=.
    pub fn core_ops_mul_assign(&self) -> DecTerm {
        self.core_ops_mul_assign
    }

    // Neg	The unary negation operator -.
    pub fn core_ops_neg(&self) -> DecTerm {
        self.core_ops_neg
    }

    // Not	The unary logical negation operator !.
    pub fn core_ops_not(&self) -> DecTerm {
        self.core_ops_not
    }

    pub fn option_ty_path(&self) -> DecTerm {
        self.option_ty_path
    }

    pub fn slice_ty_path(&self) -> DecTerm {
        self.slice_ty_path
    }

    pub fn ref_ty_path(&self) -> DecTerm {
        self.ref_ty_path
    }

    pub fn leash_ty_path(&self) -> DecItemPath {
        self.leash_ty_path
    }

    pub fn list(&self) -> DecTerm {
        self.vec_ty
    }

    pub fn unit(&self) -> DecTerm {
        self.unit
    }

    pub fn never(&self) -> DecTerm {
        self.never
    }

    pub fn trai_ty(&self) -> DecTerm {
        self.trai_ty
    }

    pub fn module(&self) -> DecTerm {
        self.module
    }

    pub fn bool(&self) -> DecTerm {
        self.bool
    }

    pub fn i8(&self) -> DecTerm {
        self.i8
    }

    pub fn i16(&self) -> DecTerm {
        self.i16
    }

    pub fn i32(&self) -> DecTerm {
        self.i32
    }

    pub fn i64(&self) -> DecTerm {
        self.i64
    }

    pub fn f32(&self) -> DecTerm {
        self.f32
    }

    pub fn f64(&self) -> DecTerm {
        self.f64
    }

    pub fn r32(&self) -> DecTerm {
        self.r32
    }

    pub fn r64(&self) -> DecTerm {
        self.r64
    }

    pub fn static_lifetime(&self) -> DecTerm {
        self.static_lifetime.into()
    }

    /// Lifetime
    pub fn lifetime_ty(&self) -> DecTerm {
        self.lifetime_ty
    }

    /// Place
    pub fn place_ty(&self) -> DecTerm {
        self.place_ty
    }

    pub fn str_ty_path(&self) -> DecTerm {
        self.str_ty_path
    }

    pub fn at_ty_path(&self) -> DecTerm {
        self.at_ty_path
    }
}
