use crate::*;

#[salsa::tracked(jar = EntityPathJar, return_ref)]
pub(crate) fn entity_path_menu(db: &dyn EntityPathDb, toolchain: Toolchain) -> EntityPathMenu {
    EntityPathMenu::new(db, toolchain)
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu {
    // core::ops::Add	The addition operator +.
    core_ops_add: TraitPath,
    // core::ops::AddAssign	The addition assignment operator +=.
    core_ops_add_assign: TraitPath,
    // core::ops::BitAnd	The bitwise AND operator &.
    core_ops_bit_and: TraitPath,
    // core::ops::BitAndAssign	The bitwise AND assignment operator &=.
    core_ops_bit_and_assign: TraitPath,
    // core::ops::BitOr	The bitwise OR operator |.
    core_ops_bit_or: TraitPath,
    // core::ops::BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign: TraitPath,
    // core::ops::BitXor	The bitwise XOR operator ^.
    core_ops_bit_xor: TraitPath,
    // core::ops::BitXorAssign	The bitwise XOR assignment operator ^=.
    core_ops_bit_xor_assign: TraitPath,
    // core::ops::Div	The division operator /.
    core_ops_div: TraitPath,
    // core::ops::DivAssign	The division assignment operator /=.
    core_ops_div_assign: TraitPath,
    // core::ops::Mul	The multiplication operator *.
    core_ops_mul: TraitPath,
    // core::ops::MulAssign	The multiplication assignment operator *=.
    core_ops_mul_assign: TraitPath,
    // core::ops::Neg	The unary negation operator -.
    core_ops_neg: TraitPath,
    // Not	The unary logical negation operator !.
    core_ops_not: TraitPath,
    default_trai_path: TraitPath,
    option_ty_path: TypePath,
    slice_ty_path: TypePath,
    string_literal_ty_path: TypePath,
    str_ty_path: TypePath,
    ref_ty_path: TypePath,
    ref_mut_ty_path: TypePath,
    list_ty_path: TypePath,
    leash_ty_path: TypePath,
    // prelude
    unit_ty_path: TypePath,
    never_ty_path: TypePath,
    bool_ty_path: TypePath,
    i8_ty_path: TypePath,
    i16_ty_path: TypePath,
    i32_ty_path: TypePath,
    i64_ty_path: TypePath,
    i128_ty_path: TypePath,
    isize_ty_path: TypePath,
    f32_ty_path: TypePath,
    f64_ty_path: TypePath,
    r8_ty_path: TypePath,
    r16_ty_path: TypePath,
    r32_ty_path: TypePath,
    r64_ty_path: TypePath,
    r128_ty_path: TypePath,
    rsize_ty_path: TypePath,
    u8_ty_path: TypePath,
    u16_ty_path: TypePath,
    u32_ty_path: TypePath,
    u64_ty_path: TypePath,
    u128_ty_path: TypePath,
    usize_ty_path: TypePath,
    trai_ty_path: TypePath,
    lifetime_ty_path: TypePath,
    module_ty_path: TypePath,
}

impl EntityPathMenu {
    pub(crate) fn new(db: &dyn EntityPathDb, toolchain: Toolchain) -> Self {
        let ident_menu = db.word_menu();
        let path_menu = db.vfs_path_menu(toolchain);
        let core_ops = path_menu.core_ops();
        let core_option = path_menu.core_option_ty_path();
        let core_slice = path_menu.core_slice();
        let core_str = path_menu.core_str();
        let core_basic = path_menu.core_basic();
        let core_default = path_menu.core_default();
        let core_num = path_menu.core_num();
        let core_raw_bits = path_menu.core_raw_bits();
        let core_mem = path_menu.core_mem();
        let core_list = path_menu.core_list();
        let option_ty_path = TypePath::new(
            db,
            core_option,
            db.it_ident_borrowed("Option").unwrap(),
            ModuleItemConnection::Connected,
            TypeKind::Enum,
        );
        let unit_ty_path = TypePath::new(
            db,
            core_basic,
            ident_menu.unit_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let never_ty_path = TypePath::new(
            db,
            core_basic,
            ident_menu.never_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let bool_ty_path = TypePath::new(
            db,
            core_basic,
            ident_menu.bool_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let i8_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.i8_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let i16_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.i16_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let i32_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.i32_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let i64_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.i64_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let i128_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.i128_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let isize_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.isize_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let u8_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.u8_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let u16_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.u16_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let u32_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.u32_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let u64_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.u64_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let u128_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.u128_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let usize_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.usize_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let r8_ty_path = TypePath::new(
            db,
            core_raw_bits,
            ident_menu.r8_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let r16_ty_path = TypePath::new(
            db,
            core_raw_bits,
            ident_menu.r16_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let r32_ty_path = TypePath::new(
            db,
            core_raw_bits,
            ident_menu.r32_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let r64_ty_path = TypePath::new(
            db,
            core_raw_bits,
            ident_menu.r64_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let r128_ty_path = TypePath::new(
            db,
            core_raw_bits,
            ident_menu.r128_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let rsize_ty_path = TypePath::new(
            db,
            core_raw_bits,
            ident_menu.rsize_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let f32_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.f32_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let f64_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.f64_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let lifetime_ty_path = TypePath::new(
            db,
            core_basic,
            ident_menu.lifetime_ty_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let module_ty_path = TypePath::new(
            db,
            core_basic,
            ident_menu.module_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let trai_ty_path = TypePath::new(
            db,
            core_basic,
            ident_menu.trai_ty_ident(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let ref_ty_path = TypePath::new(
            db,
            core_mem,
            db.it_ident_borrowed("Ref").unwrap(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let ref_mut_ty_path = TypePath::new(
            db,
            core_mem,
            db.it_ident_borrowed("RefMut").unwrap(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let leash_ty_path = TypePath::new(
            db,
            core_mem,
            db.it_ident_borrowed("Leash").unwrap(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let list_ty_path = TypePath::new(
            db,
            core_list,
            db.it_ident_borrowed("List").unwrap(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let slice_ty_path = TypePath::new(
            db,
            core_slice,
            db.it_ident_borrowed("Slice").unwrap(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let string_literal_ty_path = TypePath::new(
            db,
            core_str,
            db.it_ident_borrowed("StringLiteral").unwrap(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let str_ty_path = TypePath::new(
            db,
            core_str,
            db.it_ident_borrowed("str").unwrap(),
            ModuleItemConnection::Connected,
            TypeKind::Extern,
        );
        let core_ops_add = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("Add").unwrap(),
            ModuleItemConnection::Connected,
        );
        let core_ops_add_assign = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("AddAssign").unwrap(),
            ModuleItemConnection::Connected,
        );
        let core_ops_bit_and = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitAnd").unwrap(),
            ModuleItemConnection::Connected,
        );
        let core_ops_bit_and_assign = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitAndAssign").unwrap(),
            ModuleItemConnection::Connected,
        );
        let core_ops_bit_or = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitOr").unwrap(),
            ModuleItemConnection::Connected,
        );
        let core_ops_bit_or_assign = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitOrAssign").unwrap(),
            ModuleItemConnection::Connected,
        );
        let core_ops_bit_xor = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitXor").unwrap(),
            ModuleItemConnection::Connected,
        );
        let core_ops_bit_xor_assign = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitXorAssign").unwrap(),
            ModuleItemConnection::Connected,
        );
        let core_ops_div = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("Div").unwrap(),
            ModuleItemConnection::Connected,
        );
        let core_ops_div_assign = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("DivAssign").unwrap(),
            ModuleItemConnection::Connected,
        );
        let core_ops_mul = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("Mul").unwrap(),
            ModuleItemConnection::Connected,
        );
        let core_ops_mul_assign = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("MulAssign").unwrap(),
            ModuleItemConnection::Connected,
        );
        let core_ops_neg = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("Neg").unwrap(),
            ModuleItemConnection::Connected,
        );
        let core_ops_not = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("Not").unwrap(),
            ModuleItemConnection::Connected,
        );
        let default_trai_path = TraitPath::new(
            db,
            core_default,
            db.it_ident_borrowed("Default").unwrap(),
            ModuleItemConnection::Connected,
        );
        Self {
            unit_ty_path,
            never_ty_path,
            bool_ty_path,
            i8_ty_path,
            i16_ty_path,
            i32_ty_path,
            i64_ty_path,
            i128_ty_path,
            isize_ty_path,
            r8_ty_path,
            r16_ty_path,
            r32_ty_path,
            r64_ty_path,
            r128_ty_path,
            rsize_ty_path,
            u8_ty_path,
            u16_ty_path,
            u32_ty_path,
            u64_ty_path,
            u128_ty_path,
            usize_ty_path,
            f32_ty_path,
            f64_ty_path,
            trai_ty_path,
            module_ty_path,
            lifetime_ty_path,
            option_ty_path,
            slice_ty_path,
            string_literal_ty_path,
            str_ty_path,
            ref_ty_path,
            ref_mut_ty_path,
            list_ty_path,
            leash_ty_path,
            core_ops_add,
            core_ops_add_assign,
            core_ops_bit_and,
            core_ops_bit_and_assign,
            core_ops_bit_or,
            core_ops_bit_or_assign,
            core_ops_bit_xor,
            core_ops_bit_xor_assign,
            core_ops_div,
            core_ops_div_assign,
            core_ops_mul,
            core_ops_mul_assign,
            core_ops_neg,
            core_ops_not,
            default_trai_path,
        }
    }

    pub fn unit_ty_path(&self) -> TypePath {
        self.unit_ty_path
    }

    pub fn never_ty_path(&self) -> TypePath {
        self.never_ty_path
    }

    pub fn bool_ty_path(&self) -> TypePath {
        self.bool_ty_path
    }

    pub fn i8_ty_path(&self) -> TypePath {
        self.i8_ty_path
    }

    pub fn i16_ty_path(&self) -> TypePath {
        self.i16_ty_path
    }

    pub fn i32_ty_path(&self) -> TypePath {
        self.i32_ty_path
    }

    pub fn i64_ty_path(&self) -> TypePath {
        self.i64_ty_path
    }

    pub fn i128_ty_path(&self) -> TypePath {
        self.i128_ty_path
    }

    pub fn isize_ty_path(&self) -> TypePath {
        self.isize_ty_path
    }

    pub fn u8_ty_path(&self) -> TypePath {
        self.u8_ty_path
    }

    pub fn u16_ty_path(&self) -> TypePath {
        self.u16_ty_path
    }

    pub fn u32_ty_path(&self) -> TypePath {
        self.u32_ty_path
    }

    pub fn u64_ty_path(&self) -> TypePath {
        self.u64_ty_path
    }

    pub fn u128_ty_path(&self) -> TypePath {
        self.u128_ty_path
    }

    pub fn usize_ty_path(&self) -> TypePath {
        self.usize_ty_path
    }

    pub fn r8_ty_path(&self) -> TypePath {
        self.r8_ty_path
    }

    pub fn r16_ty_path(&self) -> TypePath {
        self.r16_ty_path
    }

    pub fn r32_ty_path(&self) -> TypePath {
        self.r32_ty_path
    }

    pub fn r64_ty_path(&self) -> TypePath {
        self.r64_ty_path
    }

    pub fn r128_ty_path(&self) -> TypePath {
        self.r128_ty_path
    }

    pub fn rsize_ty_path(&self) -> TypePath {
        self.rsize_ty_path
    }

    pub fn f32_ty_path(&self) -> TypePath {
        self.f32_ty_path
    }

    pub fn f64_ty_path(&self) -> TypePath {
        self.f64_ty_path
    }

    pub fn trai_ty_path(&self) -> TypePath {
        self.trai_ty_path
    }

    pub fn module_ty_path(&self) -> TypePath {
        self.module_ty_path
    }

    pub fn lifetime_ty_path(&self) -> TypePath {
        self.lifetime_ty_path
    }

    pub fn option_ty_path(&self) -> TypePath {
        self.option_ty_path
    }

    pub fn slice_ty_path(&self) -> TypePath {
        self.slice_ty_path
    }

    pub fn string_literal_ty_path(&self) -> TypePath {
        self.string_literal_ty_path
    }

    pub fn str_ty_path(&self) -> TypePath {
        self.str_ty_path
    }

    pub fn ref_ty_path(&self) -> TypePath {
        self.ref_ty_path
    }

    pub fn ref_mut_ty_path(&self) -> TypePath {
        self.ref_mut_ty_path
    }

    pub fn list_ty_path(&self) -> TypePath {
        self.list_ty_path
    }

    pub fn leash_ty_path(&self) -> TypePath {
        self.leash_ty_path
    }

    pub fn core_ops_add(&self) -> TraitPath {
        self.core_ops_add
    }

    pub fn core_ops_add_assign(&self) -> TraitPath {
        self.core_ops_add_assign
    }

    pub fn core_ops_bit_and(&self) -> TraitPath {
        self.core_ops_bit_and
    }

    pub fn core_ops_bit_and_assign(&self) -> TraitPath {
        self.core_ops_bit_and_assign
    }

    pub fn core_ops_bit_or(&self) -> TraitPath {
        self.core_ops_bit_or
    }

    pub fn core_ops_bit_or_assign(&self) -> TraitPath {
        self.core_ops_bit_or_assign
    }

    pub fn core_ops_bit_xor(&self) -> TraitPath {
        self.core_ops_bit_xor
    }

    pub fn core_ops_bit_xor_assign(&self) -> TraitPath {
        self.core_ops_bit_xor_assign
    }

    pub fn core_ops_div(&self) -> TraitPath {
        self.core_ops_div
    }

    pub fn core_ops_div_assign(&self) -> TraitPath {
        self.core_ops_div_assign
    }

    pub fn core_ops_mul(&self) -> TraitPath {
        self.core_ops_mul
    }

    pub fn core_ops_mul_assign(&self) -> TraitPath {
        self.core_ops_mul_assign
    }

    pub fn core_ops_neg(&self) -> TraitPath {
        self.core_ops_neg
    }

    pub fn core_ops_not(&self) -> TraitPath {
        self.core_ops_not
    }

    pub fn default_trai_path(&self) -> TraitPath {
        self.default_trai_path
    }
}

#[test]
fn menu_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain);
    assert_eq!(entity_path_menu.core_ops_add().show(&db), "core::ops::Add");
    assert_eq!(
        entity_path_menu.core_ops_add_assign().show(&db),
        "core::ops::AddAssign"
    );
    assert_eq!(
        entity_path_menu.core_ops_bit_and().show(&db),
        "core::ops::BitAnd"
    );
    assert_eq!(
        entity_path_menu.core_ops_bit_and_assign().show(&db),
        "core::ops::BitAndAssign"
    );
    assert_eq!(
        entity_path_menu.core_ops_bit_or().show(&db),
        "core::ops::BitOr"
    );
    assert_eq!(
        entity_path_menu.core_ops_bit_or_assign().show(&db),
        "core::ops::BitOrAssign"
    );
    assert_eq!(
        entity_path_menu.core_ops_bit_xor().show(&db),
        "core::ops::BitXor"
    );
    assert_eq!(
        entity_path_menu.core_ops_bit_xor_assign().show(&db),
        "core::ops::BitXorAssign"
    );
    assert_eq!(entity_path_menu.core_ops_div().show(&db), "core::ops::Div");
    assert_eq!(
        entity_path_menu.core_ops_div_assign().show(&db),
        "core::ops::DivAssign"
    );
    assert_eq!(entity_path_menu.core_ops_mul().show(&db), "core::ops::Mul");
    assert_eq!(
        entity_path_menu.core_ops_mul_assign().show(&db),
        "core::ops::MulAssign"
    );
    assert_eq!(entity_path_menu.core_ops_neg().show(&db), "core::ops::Neg");
    assert_eq!(entity_path_menu.core_ops_not().show(&db), "core::ops::Not");
}
