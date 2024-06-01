use super::*;
use husky_entity_path::menu::item_path_menu;
use husky_term_prelude::literal::Literal;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu0 {
    universe0: Universe,
    universe1: Universe,
    prop: Sort,
    ty0: Sort,
    static_lifetime: Literal,
    unit: EthTerm,
    never: EthTerm,
    // core::ops::Add	The addition operator +.
    core_ops_add: EthTerm,
    // core::ops::AddAssign	The addition assignment operator +=.
    core_ops_add_assign: EthTerm,
    // core::ops::BitAnd	The bitwise AND operator &.
    core_ops_bit_and: EthTerm,
    // core::ops::BitAndAssign	The bitwise AND assignment operator &=.
    core_ops_bit_and_assign: EthTerm,
    // core::ops::BitOr	The bitwise OR operator |.
    core_ops_bit_or: EthTerm,
    // core::ops::BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign: EthTerm,
    // core::ops::BitXor	The bitwise XOR operator ^.
    core_ops_bit_xor: EthTerm,
    // core::ops::BitXorAssign	The bitwise XOR assignment operator ^=.
    core_ops_bit_xor_assign: EthTerm,
    // core::ops::Div	The division operator /.
    core_ops_div: EthTerm,
    // core::ops::DivAssign	The division assignment operator /=.
    core_ops_div_assign: EthTerm,
    // core::ops::Mul	The multiplication operator *.
    core_ops_mul: EthTerm,
    // core::ops::MulAssign	The multiplication assignment operator *=.
    core_ops_mul_assign: EthTerm,
    // core::ops::Neg	The unary negation operator -.
    core_ops_neg: EthTerm,
    // Not	The unary logical negation operator !.
    core_ops_not: EthTerm,
    option_ty_ontology: EthTerm,
    slice_ty_ontology: EthTerm,
    str_ty_ontology: EthTerm,
    at_ty_ontology: EthTerm,
    ref_ty_ontology: EthTerm,
    list_ty_ontology: EthTerm,
    array_ty_ontology: EthTerm,
    leash_ty_ontology: EthTerm,
    i8_ty_ontology: EthTerm,
    i16_ty_ontology: EthTerm,
    i32_ty_ontology: EthTerm,
    i64_ty_ontology: EthTerm,
    i128_ty_ontology: EthTerm,
    isize_ty_ontology: EthTerm,
    r8_ty_ontology: EthTerm,
    r16_ty_ontology: EthTerm,
    r32_ty_ontology: EthTerm,
    r64_ty_ontology: EthTerm,
    r128_ty_ontology: EthTerm,
    rsize_ty_ontology: EthTerm,
    u8_ty_ontology: EthTerm,
    u16_ty_ontology: EthTerm,
    u32_ty_ontology: EthTerm,
    u64_ty_ontology: EthTerm,
    u128_ty_ontology: EthTerm,
    usize_ty_ontology: EthTerm,
    f32_ty_ontology: EthTerm,
    f64_ty_ontology: EthTerm,
    bool_ty_ontology: EthTerm,
    html_ty_ontology: EthTerm,
    lifetime_ty_ontology: EthTerm,
    trai_ty_ontology: EthTerm,
    module_ty_ontology: EthTerm,
    clone_trai: EthTerm,
    copy_trai: EthTerm,
    default_trai: EthTerm,
}

impl TermMenu0 {
    pub fn new(db: &::salsa::Db, toolchain: Toolchain) -> Self {
        // let sort = db.it_term(TermAtom::new_category(TermCategory::Sort).into());
        // let universe1 = db.it_term(TermAtom::new_universe(1).into());
        let _vfs_path_menu = db.vfs_path_menu(toolchain);
        let item_path_menu = item_path_menu(db, toolchain);
        let universe0 = Universe::new(0);
        let universe1 = Universe::new(1);
        TermMenu0 {
            static_lifetime: Literal::StaticLifetime,
            universe0,
            universe1,
            prop: Sort::new(universe0),
            ty0: Sort::new(universe1),
            core_ops_add: EthTerm::EntityPath(item_path_menu.add_trai_path().into()),
            // start here
            // EthTerm::Entity(item_path_menu.core_ops_())
            core_ops_add_assign: EthTerm::EntityPath(item_path_menu.add_assign_trai_path().into()),
            core_ops_bit_and: EthTerm::EntityPath(item_path_menu.bit_and_trai_path().into()),
            core_ops_bit_and_assign: EthTerm::EntityPath(
                item_path_menu.bit_and_assign_trai_path().into(),
            ),
            core_ops_bit_or: EthTerm::EntityPath(item_path_menu.bit_or_trai_path().into()),
            core_ops_bit_or_assign: EthTerm::EntityPath(
                item_path_menu.bit_or_assign_trai_path().into(),
            ),
            core_ops_bit_xor: EthTerm::EntityPath(item_path_menu.bit_xor_trai_path().into()),
            core_ops_bit_xor_assign: EthTerm::EntityPath(
                item_path_menu.bit_xor_assign_trai_path().into(),
            ),
            core_ops_div: EthTerm::EntityPath(item_path_menu.div_trai_path().into()),
            core_ops_div_assign: EthTerm::EntityPath(item_path_menu.div_assign_trai_path().into()),
            core_ops_mul: EthTerm::EntityPath(item_path_menu.mul_trai_path().into()),
            core_ops_mul_assign: EthTerm::EntityPath(item_path_menu.mul_assign_trai_path().into()),
            core_ops_neg: EthTerm::EntityPath(item_path_menu.neg_trai_path().into()),
            core_ops_not: EthTerm::EntityPath(item_path_menu.not_trai_path().into()),
            option_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.option_ty_path()).into(),
            slice_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.slice_ty_path()).into(),
            str_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.str_ty_path()).into(),
            at_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.at_ty_path()).into(),
            ref_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.ref_ty_path()).into(),
            list_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.vec_ty_path()).into(),
            array_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.array_ty_path()).into(),
            leash_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.leash_ty_path()).into(),
            unit: ItemPathTerm::TypeOntology(item_path_menu.unit_ty_path()).into(),
            never: ItemPathTerm::TypeOntology(item_path_menu.never_ty_path()).into(),
            bool_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.bool_ty_path()).into(),
            trai_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.trai_ty_path()).into(),
            lifetime_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.lifetime_ty_path())
                .into(),
            module_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.module_ty_path()).into(),
            i8_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.i8_ty_path()).into(),
            i16_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.i16_ty_path()).into(),
            i32_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.i32_ty_path()).into(),
            i64_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.i64_ty_path()).into(),
            i128_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.i128_ty_path()).into(),
            isize_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.isize_ty_path()).into(),
            u8_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.u8_ty_path()).into(),
            u16_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.u16_ty_path()).into(),
            u32_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.u32_ty_path()).into(),
            u64_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.u64_ty_path()).into(),
            u128_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.u128_ty_path()).into(),
            usize_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.usize_ty_path()).into(),
            f32_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.f32_ty_path()).into(),
            f64_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.f64_ty_path()).into(),
            r8_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.r8_ty_path()).into(),
            r16_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.r16_ty_path()).into(),
            r32_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.r32_ty_path()).into(),
            r64_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.r64_ty_path()).into(),
            r128_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.r128_ty_path()).into(),
            rsize_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.rsize_ty_path()).into(),
            html_ty_ontology: ItemPathTerm::TypeOntology(item_path_menu.visual_ty_path()).into(),
            clone_trai: ItemPathTerm::Trait(item_path_menu.clone_trai_path()).into(),
            copy_trai: ItemPathTerm::Trait(item_path_menu.copy_trai_path()).into(),
            default_trai: ItemPathTerm::Trait(item_path_menu.default_trai_path()).into(),
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

    // pub fn core(&self) -> EthTerm {
    //     self.core
    // }

    // pub fn core_ops(&self) -> EthTerm {
    //     self.core_ops
    // }

    // Add	The addition operator +.
    pub fn core_ops_add(&self) -> EthTerm {
        self.core_ops_add
    }

    // AddAssign	The addition assignment operator +=.
    pub fn core_ops_add_assign(&self) -> EthTerm {
        self.core_ops_add_assign
    }

    // BitAnd	The bitwise AND operator &.
    pub fn core_ops_bit_and(&self) -> EthTerm {
        self.core_ops_bit_and
    }

    // BitAndAssign	The bitwise AND assignment operator &=.
    pub fn core_ops_bit_and_assign(&self) -> EthTerm {
        self.core_ops_bit_and_assign
    }

    // BitOr	The bitwise OR operator |.
    pub fn core_ops_bit_or(&self) -> EthTerm {
        self.core_ops_bit_or
    }

    // BitOrAssign	The bitwise OR assignment operator |=.
    pub fn core_ops_bit_or_assign(&self) -> EthTerm {
        self.core_ops_bit_or_assign
    }
    // BitXor	The bitwise XOR operator ^.
    pub fn core_ops_bit_xor(&self) -> EthTerm {
        self.core_ops_bit_xor
    }

    // BitXorAssign	The bitwise XOR assignment operator ^=.
    pub fn core_ops_bit_xor_assign(&self) -> EthTerm {
        self.core_ops_bit_or_assign
    }

    // Div	The division operator /.
    pub fn core_ops_div(&self) -> EthTerm {
        self.core_ops_div
    }

    // DivAssign	The division assignment operator /=.
    pub fn core_ops_div_assign(&self) -> EthTerm {
        self.core_ops_div_assign
    }

    // Mul	The multiplication operator *.
    pub fn core_ops_mul(&self) -> EthTerm {
        self.core_ops_mul
    }

    // MulAssign	The multiplication assignment operator *=.
    pub fn core_ops_mul_assign(&self) -> EthTerm {
        self.core_ops_mul_assign
    }

    // Neg	The unary negation operator -.
    pub fn core_ops_neg(&self) -> EthTerm {
        self.core_ops_neg
    }

    // Not	The unary logical negation operator !.
    pub fn core_ops_not(&self) -> EthTerm {
        self.core_ops_not
    }

    pub fn option_ty_ontology(&self) -> EthTerm {
        self.option_ty_ontology
    }

    pub fn slice_ty_ontology(&self) -> EthTerm {
        self.slice_ty_ontology
    }

    pub fn at_ty_ontology(&self) -> EthTerm {
        self.at_ty_ontology
    }

    pub fn ref_ty_path(&self) -> EthTerm {
        self.ref_ty_ontology
    }

    pub fn list_ty_ontology(&self) -> EthTerm {
        self.list_ty_ontology
    }

    pub fn array_ty_ontology(&self) -> EthTerm {
        self.array_ty_ontology
    }

    pub fn unit_ty_ontology(&self) -> EthTerm {
        self.unit
    }

    pub fn never(&self) -> EthTerm {
        self.never
    }

    pub fn trai_ty_ontology(&self) -> EthTerm {
        self.trai_ty_ontology
    }

    pub fn module_ty_ontology(&self) -> EthTerm {
        self.module_ty_ontology
    }

    pub fn bool_ty_ontology(&self) -> EthTerm {
        self.bool_ty_ontology
    }

    pub fn i8_ty_ontology(&self) -> EthTerm {
        self.i8_ty_ontology
    }

    pub fn i16_ty_ontology(&self) -> EthTerm {
        self.i16_ty_ontology
    }

    pub fn i32_ty_ontology(&self) -> EthTerm {
        self.i32_ty_ontology
    }

    pub fn i64_ty_ontology(&self) -> EthTerm {
        self.i64_ty_ontology
    }

    pub fn i128_ty_ontology(&self) -> EthTerm {
        self.i128_ty_ontology
    }

    pub fn isize_ty_ontology(&self) -> EthTerm {
        self.isize_ty_ontology
    }

    pub fn u8_ty_ontology(&self) -> EthTerm {
        self.u8_ty_ontology
    }

    pub fn u16_ty_ontology(&self) -> EthTerm {
        self.u16_ty_ontology
    }

    pub fn u32_ty_ontology(&self) -> EthTerm {
        self.u32_ty_ontology
    }

    pub fn u64_ty_ontology(&self) -> EthTerm {
        self.u64_ty_ontology
    }

    pub fn u128_ty_ontology(&self) -> EthTerm {
        self.u128_ty_ontology
    }

    pub fn usize_ty_ontology(&self) -> EthTerm {
        self.usize_ty_ontology
    }

    pub fn r8_ty_ontology(&self) -> EthTerm {
        self.r8_ty_ontology
    }

    pub fn r16_ty_ontology(&self) -> EthTerm {
        self.r16_ty_ontology
    }

    pub fn r128_ty_ontology(&self) -> EthTerm {
        self.r128_ty_ontology
    }

    pub fn rsize_ty_ontology(&self) -> EthTerm {
        self.rsize_ty_ontology
    }

    pub fn f32_ty_ontology(&self) -> EthTerm {
        self.f32_ty_ontology
    }

    pub fn f64_ty_ontology(&self) -> EthTerm {
        self.f64_ty_ontology
    }

    pub fn r32_ty_ontology(&self) -> EthTerm {
        self.r32_ty_ontology
    }

    pub fn r64_ty_ontology(&self) -> EthTerm {
        self.r64_ty_ontology
    }

    pub fn static_lifetime(&self) -> EthTerm {
        self.static_lifetime.into()
    }

    /// Lifetime
    pub fn lifetime_ty(&self) -> EthTerm {
        self.lifetime_ty_ontology
    }

    pub fn str_ty_ontology(&self) -> EthTerm {
        self.str_ty_ontology
    }

    pub fn leash_ty_ontology(&self) -> EthTerm {
        self.leash_ty_ontology
    }

    pub fn clone_trai(&self) -> EthTerm {
        self.clone_trai
    }

    pub fn copy_trai(&self) -> EthTerm {
        self.copy_trai
    }

    pub fn default_trai(&self) -> EthTerm {
        self.default_trai
    }

    pub fn html_ty_ontology(&self) -> EthTerm {
        self.html_ty_ontology
    }
}
