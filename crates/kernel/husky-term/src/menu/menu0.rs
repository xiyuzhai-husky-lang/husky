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
    str_ty_ontology: Term,
    ref_ty_path: Term,
    list_ty_ontology: Term,
    leash_ty_ontology: Term,
    i8_ty_ontology: Term,
    i16_ty_ontology: Term,
    i32_ty_ontology: Term,
    i64_ty_ontology: Term,
    i128_ty_ontology: Term,
    isize_ty_ontology: Term,
    r8_ty_ontology: Term,
    r16_ty_ontology: Term,
    r32_ty_ontology: Term,
    r64_ty_ontology: Term,
    r128_ty_ontology: Term,
    rsize_ty_ontology: Term,
    u8_ty_ontology: Term,
    u16_ty_ontology: Term,
    u32_ty_ontology: Term,
    u64_ty_ontology: Term,
    u128_ty_ontology: Term,
    usize_ty_ontology: Term,
    f32_ty_ontology: Term,
    f64_ty_ontology: Term,
    bool_ty_ontology: Term,
    lifetime_ty_ontology: Term,
    trai_ty_ontology: Term,
    module_ty_ontology: Term,
    default_trai: Term,
}

impl TermMenu0 {
    pub fn new(db: &dyn TermDb, toolchain: Toolchain) -> Self {
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
            str_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.str_ty_path()).into(),
            ref_ty_path: TermEntityPath::TypeOntology(entity_path_menu.ref_ty_path()).into(),
            list_ty_ontology: TermEntityPath::TypeOntology(entity_path_menu.list_ty_path()).into(),
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

    pub fn list_ty_ontology(&self) -> Term {
        self.list_ty_ontology
    }

    pub fn unit(&self) -> Term {
        self.unit
    }

    pub fn never(&self) -> Term {
        self.never
    }

    pub fn trai_ty_ontology(&self) -> Term {
        self.trai_ty_ontology
    }

    pub fn module_ty_ontology(&self) -> Term {
        self.module_ty_ontology
    }

    pub fn bool_ty_ontology(&self) -> Term {
        self.bool_ty_ontology
    }

    pub fn i8_ty_ontology(&self) -> Term {
        self.i8_ty_ontology
    }

    pub fn i16_ty_ontology(&self) -> Term {
        self.i16_ty_ontology
    }

    pub fn i32_ty_ontology(&self) -> Term {
        self.i32_ty_ontology
    }

    pub fn i64_ty_ontology(&self) -> Term {
        self.i64_ty_ontology
    }

    pub fn i128_ty_ontology(&self) -> Term {
        self.i128_ty_ontology
    }

    pub fn isize_ty_ontology(&self) -> Term {
        self.isize_ty_ontology
    }

    pub fn u8_ty_ontology(&self) -> Term {
        self.u8_ty_ontology
    }

    pub fn u16_ty_ontology(&self) -> Term {
        self.u16_ty_ontology
    }

    pub fn u32_ty_ontology(&self) -> Term {
        self.u32_ty_ontology
    }

    pub fn u64_ty_ontology(&self) -> Term {
        self.u64_ty_ontology
    }

    pub fn u128_ty_ontology(&self) -> Term {
        self.u128_ty_ontology
    }

    pub fn usize_ty_ontology(&self) -> Term {
        self.usize_ty_ontology
    }

    pub fn r8_ty_ontology(&self) -> Term {
        self.r8_ty_ontology
    }

    pub fn r16_ty_ontology(&self) -> Term {
        self.r16_ty_ontology
    }

    pub fn r128_ty_ontology(&self) -> Term {
        self.r128_ty_ontology
    }

    pub fn rsize_ty_ontology(&self) -> Term {
        self.rsize_ty_ontology
    }

    pub fn f32_ty_ontology(&self) -> Term {
        self.f32_ty_ontology
    }

    pub fn f64_ty_ontology(&self) -> Term {
        self.f64_ty_ontology
    }

    pub fn r32_ty_ontology(&self) -> Term {
        self.r32_ty_ontology
    }

    pub fn r64_ty_ontology(&self) -> Term {
        self.r64_ty_ontology
    }

    pub fn eval_lifetime(&self) -> Term {
        self.eval_lifetime.into()
    }

    pub fn static_lifetime(&self) -> Term {
        self.static_lifetime.into()
    }

    /// Lifetime
    pub fn lifetime_ty(&self) -> Term {
        self.lifetime_ty_ontology
    }

    pub fn str_ty_ontology(&self) -> Term {
        self.str_ty_ontology
    }

    pub fn leash_ty_ontology(&self) -> Term {
        self.leash_ty_ontology
    }

    pub fn default_trai(&self) -> Term {
        self.default_trai
    }
}
