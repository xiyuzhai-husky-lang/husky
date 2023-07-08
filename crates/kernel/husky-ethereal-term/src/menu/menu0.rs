use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu0 {
    universe0: TermUniverse,
    universe1: TermUniverse,
    prop: TermCategory,
    ty0: TermCategory,
    eval_lifetime: TermLiteral,
    static_lifetime: TermLiteral,
    unit: EtherealTerm,
    never: EtherealTerm,
    // core::ops::Add	The addition operator +.
    core_ops_add: EtherealTerm,
    // core::ops::AddAssign	The addition assignment operator +=.
    core_ops_add_assign: EtherealTerm,
    // core::ops::BitAnd	The bitwise AND operator &.
    core_ops_bit_and: EtherealTerm,
    // core::ops::BitAndAssign	The bitwise AND assignment operator &=.
    core_ops_bit_and_assign: EtherealTerm,
    // core::ops::BitOr	The bitwise OR operator |.
    core_ops_bit_or: EtherealTerm,
    // core::ops::BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign: EtherealTerm,
    // core::ops::BitXor	The bitwise XOR operator ^.
    core_ops_bit_xor: EtherealTerm,
    // core::ops::BitXorAssign	The bitwise XOR assignment operator ^=.
    core_ops_bit_xor_assign: EtherealTerm,
    // core::ops::Div	The division operator /.
    core_ops_div: EtherealTerm,
    // core::ops::DivAssign	The division assignment operator /=.
    core_ops_div_assign: EtherealTerm,
    // core::ops::Mul	The multiplication operator *.
    core_ops_mul: EtherealTerm,
    // core::ops::MulAssign	The multiplication assignment operator *=.
    core_ops_mul_assign: EtherealTerm,
    // core::ops::Neg	The unary negation operator -.
    core_ops_neg: EtherealTerm,
    // Not	The unary logical negation operator !.
    core_ops_not: EtherealTerm,
    option_ty_path: EtherealTerm,
    slice_ty_path: EtherealTerm,
    str_ty_ontology: EtherealTerm,
    ref_ty_path: EtherealTerm,
    list_ty_ontology: EtherealTerm,
    array_ty_ontology: EtherealTerm,
    leash_ty_ontology: EtherealTerm,
    i8_ty_ontology: EtherealTerm,
    i16_ty_ontology: EtherealTerm,
    i32_ty_ontology: EtherealTerm,
    i64_ty_ontology: EtherealTerm,
    i128_ty_ontology: EtherealTerm,
    isize_ty_ontology: EtherealTerm,
    r8_ty_ontology: EtherealTerm,
    r16_ty_ontology: EtherealTerm,
    r32_ty_ontology: EtherealTerm,
    r64_ty_ontology: EtherealTerm,
    r128_ty_ontology: EtherealTerm,
    rsize_ty_ontology: EtherealTerm,
    u8_ty_ontology: EtherealTerm,
    u16_ty_ontology: EtherealTerm,
    u32_ty_ontology: EtherealTerm,
    u64_ty_ontology: EtherealTerm,
    u128_ty_ontology: EtherealTerm,
    usize_ty_ontology: EtherealTerm,
    f32_ty_ontology: EtherealTerm,
    f64_ty_ontology: EtherealTerm,
    bool_ty_ontology: EtherealTerm,
    html_ty_ontology: EtherealTerm,
    lifetime_ty_ontology: EtherealTerm,
    trai_ty_ontology: EtherealTerm,
    module_ty_ontology: EtherealTerm,
    clone_trai: EtherealTerm,
    copy_trai: EtherealTerm,
    default_trai: EtherealTerm,
}

impl TermMenu0 {
    pub fn new(db: &dyn EtherealTermDb, toolchain: Toolchain) -> Self {
        // let sort = db.it_term(TermAtom::new_category(TermCategory::Sort).into());
        // let universe1 = db.it_term(TermAtom::new_universe(1).into());
        let vfs_path_menu = db.vfs_path_menu(toolchain);
        let entity_path_menu = db.entity_path_menu(toolchain);
        let universe0 = TermUniverse::new(0);
        let universe1 = TermUniverse::new(1);
        TermMenu0 {
            eval_lifetime: TermLiteral::EvalLifetime,
            static_lifetime: TermLiteral::StaticLifetime,
            universe0,
            universe1,
            prop: TermCategory::new(universe0),
            ty0: TermCategory::new(universe1),
            core_ops_add: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_add_trai_path().into(),
            ),
            // start here
            // EtherealTerm::Entity(entity_path_menu.core_ops_())
            core_ops_add_assign: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_add_assign_trai_path().into(),
            ),
            core_ops_bit_and: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_bit_and_trai_path().into(),
            ),
            core_ops_bit_and_assign: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_bit_and_assign_trai_path().into(),
            ),
            core_ops_bit_or: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_bit_or_trai_path().into(),
            ),
            core_ops_bit_or_assign: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_bit_or_assign_trai_path().into(),
            ),
            core_ops_bit_xor: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_bit_xor_trai_path().into(),
            ),
            core_ops_bit_xor_assign: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_bit_xor_assign_trai_path().into(),
            ),
            core_ops_div: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_div_trai_path().into(),
            ),
            core_ops_div_assign: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_div_assign_trai_path().into(),
            ),
            core_ops_mul: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_mul_trai_path().into(),
            ),
            core_ops_mul_assign: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_mul_assign_trai_path().into(),
            ),
            core_ops_neg: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_neg_trai_path().into(),
            ),
            core_ops_not: EtherealTerm::EntityPath(
                entity_path_menu.core_ops_not_trai_path().into(),
            ),
            option_ty_path: TermEntityPath::TypeOntology(entity_path_menu.option_ty_path()).into(),
            slice_ty_path: TermEntityPath::TypeOntology(entity_path_menu.slice_ty_path()).into(),
            str_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.str_ty_path()).into(),
            ref_ty_path: TermEntityPath::TypeOntology(entity_path_menu.ref_ty_path()).into(),
            list_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.list_ty_path()).into(),
            array_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.array_ty_path())
                .into(),
            leash_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.leash_ty_path())
                .into(),
            unit: TermEntityPath::TypeOntology(entity_path_menu.unit_ty_path()).into(),
            never: TermEntityPath::TypeOntology(entity_path_menu.never_ty_path()).into(),
            bool_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.bool_ty_path()).into(),
            trai_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.trai_ty_path()).into(),
            lifetime_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.lifetime_ty_path())
                .into(),
            module_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.module_ty_path())
                .into(),
            i8_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.i8_ty_path()).into(),
            i16_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.i16_ty_path()).into(),
            i32_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.i32_ty_path()).into(),
            i64_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.i64_ty_path()).into(),
            i128_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.i128_ty_path()).into(),
            isize_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.isize_ty_path())
                .into(),
            u8_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.u8_ty_path()).into(),
            u16_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.u16_ty_path()).into(),
            u32_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.u32_ty_path()).into(),
            u64_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.u64_ty_path()).into(),
            u128_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.u128_ty_path()).into(),
            usize_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.usize_ty_path())
                .into(),
            f32_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.f32_ty_path()).into(),
            f64_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.f64_ty_path()).into(),
            r8_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.r8_ty_path()).into(),
            r16_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.r16_ty_path()).into(),
            r32_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.r32_ty_path()).into(),
            r64_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.r64_ty_path()).into(),
            r128_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.r128_ty_path()).into(),
            rsize_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.rsize_ty_path())
                .into(),
            html_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.html_ty_path()).into(),
            clone_trai: TermEntityPath::Trait(entity_path_menu.clone_trai_path()).into(),
            copy_trai: TermEntityPath::Trait(entity_path_menu.copy_trai_path()).into(),
            default_trai: TermEntityPath::Trait(entity_path_menu.default_trai_path()).into(),
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

    // pub fn core(&self) -> EtherealTerm {
    //     self.core
    // }

    // pub fn core_ops(&self) -> EtherealTerm {
    //     self.core_ops
    // }

    // Add	The addition operator +.
    pub fn core_ops_add(&self) -> EtherealTerm {
        self.core_ops_add
    }

    // AddAssign	The addition assignment operator +=.
    pub fn core_ops_add_assign(&self) -> EtherealTerm {
        self.core_ops_add_assign
    }

    // BitAnd	The bitwise AND operator &.
    pub fn core_ops_bit_and(&self) -> EtherealTerm {
        self.core_ops_bit_and
    }

    // BitAndAssign	The bitwise AND assignment operator &=.
    pub fn core_ops_bit_and_assign(&self) -> EtherealTerm {
        self.core_ops_bit_and_assign
    }

    // BitOr	The bitwise OR operator |.
    pub fn core_ops_bit_or(&self) -> EtherealTerm {
        self.core_ops_bit_or
    }

    // BitOrAssign	The bitwise OR assignment operator |=.
    pub fn core_ops_bit_or_assign(&self) -> EtherealTerm {
        self.core_ops_bit_or_assign
    }
    // BitXor	The bitwise XOR operator ^.
    pub fn core_ops_bit_xor(&self) -> EtherealTerm {
        self.core_ops_bit_xor
    }

    // BitXorAssign	The bitwise XOR assignment operator ^=.
    pub fn core_ops_bit_xor_assign(&self) -> EtherealTerm {
        self.core_ops_bit_or_assign
    }

    // Div	The division operator /.
    pub fn core_ops_div(&self) -> EtherealTerm {
        self.core_ops_div
    }

    // DivAssign	The division assignment operator /=.
    pub fn core_ops_div_assign(&self) -> EtherealTerm {
        self.core_ops_div_assign
    }

    // Mul	The multiplication operator *.
    pub fn core_ops_mul(&self) -> EtherealTerm {
        self.core_ops_mul
    }

    // MulAssign	The multiplication assignment operator *=.
    pub fn core_ops_mul_assign(&self) -> EtherealTerm {
        self.core_ops_mul_assign
    }

    // Neg	The unary negation operator -.
    pub fn core_ops_neg(&self) -> EtherealTerm {
        self.core_ops_neg
    }

    // Not	The unary logical negation operator !.
    pub fn core_ops_not(&self) -> EtherealTerm {
        self.core_ops_not
    }

    pub fn option_ty_path(&self) -> EtherealTerm {
        self.option_ty_path
    }

    pub fn slice_ty_path(&self) -> EtherealTerm {
        self.slice_ty_path
    }

    pub fn ref_ty_path(&self) -> EtherealTerm {
        self.ref_ty_path
    }

    pub fn list_ty_ontology(&self) -> EtherealTerm {
        self.list_ty_ontology
    }

    pub fn array_ty_ontology(&self) -> EtherealTerm {
        self.array_ty_ontology
    }

    pub fn unit_ty_ontology(&self) -> EtherealTerm {
        self.unit
    }

    pub fn never(&self) -> EtherealTerm {
        self.never
    }

    pub fn trai_ty_ontology(&self) -> EtherealTerm {
        self.trai_ty_ontology
    }

    pub fn module_ty_ontology(&self) -> EtherealTerm {
        self.module_ty_ontology
    }

    pub fn bool_ty_ontology(&self) -> EtherealTerm {
        self.bool_ty_ontology
    }

    pub fn i8_ty_ontology(&self) -> EtherealTerm {
        self.i8_ty_ontology
    }

    pub fn i16_ty_ontology(&self) -> EtherealTerm {
        self.i16_ty_ontology
    }

    pub fn i32_ty_ontology(&self) -> EtherealTerm {
        self.i32_ty_ontology
    }

    pub fn i64_ty_ontology(&self) -> EtherealTerm {
        self.i64_ty_ontology
    }

    pub fn i128_ty_ontology(&self) -> EtherealTerm {
        self.i128_ty_ontology
    }

    pub fn isize_ty_ontology(&self) -> EtherealTerm {
        self.isize_ty_ontology
    }

    pub fn u8_ty_ontology(&self) -> EtherealTerm {
        self.u8_ty_ontology
    }

    pub fn u16_ty_ontology(&self) -> EtherealTerm {
        self.u16_ty_ontology
    }

    pub fn u32_ty_ontology(&self) -> EtherealTerm {
        self.u32_ty_ontology
    }

    pub fn u64_ty_ontology(&self) -> EtherealTerm {
        self.u64_ty_ontology
    }

    pub fn u128_ty_ontology(&self) -> EtherealTerm {
        self.u128_ty_ontology
    }

    pub fn usize_ty_ontology(&self) -> EtherealTerm {
        self.usize_ty_ontology
    }

    pub fn r8_ty_ontology(&self) -> EtherealTerm {
        self.r8_ty_ontology
    }

    pub fn r16_ty_ontology(&self) -> EtherealTerm {
        self.r16_ty_ontology
    }

    pub fn r128_ty_ontology(&self) -> EtherealTerm {
        self.r128_ty_ontology
    }

    pub fn rsize_ty_ontology(&self) -> EtherealTerm {
        self.rsize_ty_ontology
    }

    pub fn f32_ty_ontology(&self) -> EtherealTerm {
        self.f32_ty_ontology
    }

    pub fn f64_ty_ontology(&self) -> EtherealTerm {
        self.f64_ty_ontology
    }

    pub fn r32_ty_ontology(&self) -> EtherealTerm {
        self.r32_ty_ontology
    }

    pub fn r64_ty_ontology(&self) -> EtherealTerm {
        self.r64_ty_ontology
    }

    pub fn eval_lifetime(&self) -> EtherealTerm {
        self.eval_lifetime.into()
    }

    pub fn static_lifetime(&self) -> EtherealTerm {
        self.static_lifetime.into()
    }

    /// Lifetime
    pub fn lifetime_ty(&self) -> EtherealTerm {
        self.lifetime_ty_ontology
    }

    pub fn str_ty_ontology(&self) -> EtherealTerm {
        self.str_ty_ontology
    }

    pub fn leash_ty_ontology(&self) -> EtherealTerm {
        self.leash_ty_ontology
    }

    pub fn clone_trai(&self) -> EtherealTerm {
        self.clone_trai
    }

    pub fn copy_trai(&self) -> EtherealTerm {
        self.copy_trai
    }

    pub fn default_trai(&self) -> EtherealTerm {
        self.default_trai
    }

    pub fn html_ty_ontology(&self) -> EtherealTerm {
        self.html_ty_ontology
    }
}
