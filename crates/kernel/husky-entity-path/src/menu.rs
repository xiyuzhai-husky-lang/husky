use crate::*;

#[salsa::tracked(jar = EntityPathJar, return_ref)]
pub(crate) fn item_path_menu(db: &dyn EntityPathDb, toolchain: Toolchain) -> ItemPathMenu {
    ItemPathMenu::new(db, toolchain)
}

#[derive(Debug, PartialEq, Eq)]
pub struct ItemPathMenu {
    // core::ops::Add	The addition operator +.
    core_ops_add_trai_path: TraitPath,
    // core::ops::AddAssign	The addition assignment operator +=.
    add_assign_trai_path: TraitPath,
    // core::ops::BitAnd	The bitwise AND operator &.
    bit_and_trai_path: TraitPath,
    // core::ops::BitAndAssign	The bitwise AND assignment operator &=.
    bit_and_assign_trai_path: TraitPath,
    // core::ops::BitOr	The bitwise OR operator |.
    bit_or_trai_path: TraitPath,
    // core::ops::BitOrAssign	The bitwise OR assignment operator |=.
    core_ops_bit_or_assign_trai_path: TraitPath,
    // core::ops::BitXor	The bitwise XOR operator ^.
    bit_xor_trai_path: TraitPath,
    // core::ops::BitXorAssign	The bitwise XOR assignment operator ^=.
    bit_xor_assign_trai_path: TraitPath,
    // core::ops::Div	The division operator /.
    div_trai_path: TraitPath,
    // core::ops::DivAssign	The division assignment operator /=.
    div_assign_trai_path: TraitPath,
    // core::ops::IntIndex
    int_index_trai_path: TraitPath,
    // core::ops::Mul	The multiplication operator *.
    mul_trai_path: TraitPath,
    // core::ops::MulAssign	The multiplication assignment operator *=.
    mul_assign_trai_path: TraitPath,
    // core::ops::Neg	The unary negation operator -.
    neg_trai_path: TraitPath,
    // Not	The unary logical negation operator !.
    not_trai_path: TraitPath,
    unveil_trai_path: TraitPath,
    clone_trai_path: TraitPath,
    copy_trai_path: TraitPath,
    default_trai_path: TraitPath,
    option_ty_path: TypePath,
    result_ty_path: TypePath,
    slice_ty_path: TypePath,
    cyclic_slice_leashed_ty_path: TypePath,
    string_literal_ty_path: TypePath,
    str_ty_path: TypePath,
    ref_ty_path: TypePath,
    ref_mut_ty_path: TypePath,
    at_ty_path: TypePath,
    vec_ty_path: TypePath,
    array_ty_path: TypePath,
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
    html_ty_path: TypePath,
    trai_ty_path: TypePath,
    lifetime_ty_path: TypePath,
    place_ty_path: TypePath,
    module_ty_path: TypePath,
}

impl ItemPathMenu {
    pub(crate) fn new(db: &dyn EntityPathDb, toolchain: Toolchain) -> Self {
        let ident_menu = db.coword_menu();
        let path_menu = db.vfs_path_menu(toolchain);
        let core_ops = path_menu.core_ops().inner();
        let core_option = path_menu.core_option().inner();
        let core_slice = path_menu.core_slice().inner();
        let core_str = path_menu.core_str().inner();
        let core_basic = path_menu.core_basic().inner();
        let core_clone = path_menu.core_clone().inner();
        let core_result = path_menu.core_result().inner();
        let core_default = path_menu.core_default().inner();
        let core_marker = path_menu.core_marker().inner();
        let core_num = path_menu.core_num().inner();
        let core_raw_bits = path_menu.core_raw_bits().inner();
        let core_mem = path_menu.core_mem().inner();
        let core_vec = path_menu.core_vec().inner();
        let core_array = path_menu.core_array().inner();
        let core_visual = path_menu.core_visual().inner();
        let option_ty_path = TypePath::new(
            db,
            core_option,
            db.it_ident_borrowed("Option").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Enum,
        );
        let result_ty_path = TypePath::new(
            db,
            core_result,
            db.it_ident_borrowed("Result").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Enum,
        );
        let unit_ty_path = TypePath::new(
            db,
            core_basic,
            ident_menu.unit_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let never_ty_path = TypePath::new(
            db,
            core_basic,
            ident_menu.never_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let bool_ty_path = TypePath::new(
            db,
            core_basic,
            ident_menu.bool_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let i8_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.i8_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let i16_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.i16_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let i32_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.i32_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let i64_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.i64_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let i128_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.i128_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let isize_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.isize_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let u8_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.u8_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let u16_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.u16_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let u32_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.u32_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let u64_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.u64_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let u128_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.u128_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let usize_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.usize_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let r8_ty_path = TypePath::new(
            db,
            core_raw_bits,
            ident_menu.r8_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let r16_ty_path = TypePath::new(
            db,
            core_raw_bits,
            ident_menu.r16_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let r32_ty_path = TypePath::new(
            db,
            core_raw_bits,
            ident_menu.r32_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let r64_ty_path = TypePath::new(
            db,
            core_raw_bits,
            ident_menu.r64_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let r128_ty_path = TypePath::new(
            db,
            core_raw_bits,
            ident_menu.r128_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let rsize_ty_path = TypePath::new(
            db,
            core_raw_bits,
            ident_menu.rsize_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let f32_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.f32_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let f64_ty_path = TypePath::new(
            db,
            core_num,
            ident_menu.f64_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let lifetime_ty_path = TypePath::new(
            db,
            core_basic,
            ident_menu.lifetime_ty_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let place_ty_path = TypePath::new(
            db,
            core_basic,
            ident_menu.place_ty_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let module_ty_path = TypePath::new(
            db,
            core_basic,
            ident_menu.module_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let html_ty_path = TypePath::new(
            db,
            core_visual,
            db.it_ident_borrowed("Html").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let trai_ty_path = TypePath::new(
            db,
            core_basic,
            ident_menu.trai_ty_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let ref_ty_path = TypePath::new(
            db,
            core_mem,
            db.it_ident_borrowed("Ref").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let ref_mut_ty_path = TypePath::new(
            db,
            core_mem,
            db.it_ident_borrowed("RefMut").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let leash_ty_path = TypePath::new(
            db,
            core_mem,
            db.it_ident_borrowed("Leash").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let at_ty_path = TypePath::new(
            db,
            core_mem,
            db.it_ident_borrowed("At").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let vec_ty_path = TypePath::new(
            db,
            core_vec,
            db.it_ident_borrowed("Vec").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let array_ty_path = TypePath::new(
            db,
            core_array,
            db.it_ident_borrowed("Array").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let slice_ty_path = TypePath::new(
            db,
            core_slice,
            db.it_ident_borrowed("Slice").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let cyclic_slice_leashed_ty_path = TypePath::new(
            db,
            core_slice,
            db.it_ident_borrowed("CyclicSlice").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let string_literal_ty_path = TypePath::new(
            db,
            core_str,
            db.it_ident_borrowed("StringLiteral").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let str_ty_path = TypePath::new(
            db,
            core_str,
            db.it_ident_borrowed("str").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
        );
        let core_ops_add_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("Add").unwrap(),
            MajorItemConnection::Connected,
        );
        let add_assign_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("AddAssign").unwrap(),
            MajorItemConnection::Connected,
        );
        let bit_and_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitAnd").unwrap(),
            MajorItemConnection::Connected,
        );
        let bit_and_assign_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitAndAssign").unwrap(),
            MajorItemConnection::Connected,
        );
        let bit_or_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitOr").unwrap(),
            MajorItemConnection::Connected,
        );
        let core_ops_bit_or_assign_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitOrAssign").unwrap(),
            MajorItemConnection::Connected,
        );
        let bit_xor_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitXor").unwrap(),
            MajorItemConnection::Connected,
        );
        let bit_xor_assign_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("BitXorAssign").unwrap(),
            MajorItemConnection::Connected,
        );
        let div_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("Div").unwrap(),
            MajorItemConnection::Connected,
        );
        let div_assign_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("DivAssign").unwrap(),
            MajorItemConnection::Connected,
        );
        let int_index_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("IntIndex").unwrap(),
            MajorItemConnection::Connected,
        );
        let mul_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("Mul").unwrap(),
            MajorItemConnection::Connected,
        );
        let mul_assign_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("MulAssign").unwrap(),
            MajorItemConnection::Connected,
        );
        let neg_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("Neg").unwrap(),
            MajorItemConnection::Connected,
        );
        let not_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("Not").unwrap(),
            MajorItemConnection::Connected,
        );
        let unveil_trai_path = TraitPath::new(
            db,
            core_ops,
            db.it_ident_borrowed("Unveil").unwrap(),
            MajorItemConnection::Connected,
        );
        let clone_trai_path = TraitPath::new(
            db,
            core_clone,
            db.it_ident_borrowed("Clone").unwrap(),
            MajorItemConnection::Connected,
        );
        let copy_trai_path = TraitPath::new(
            db,
            core_marker,
            db.it_ident_borrowed("Copy").unwrap(),
            MajorItemConnection::Connected,
        );
        let default_trai_path = TraitPath::new(
            db,
            core_default,
            db.it_ident_borrowed("Default").unwrap(),
            MajorItemConnection::Connected,
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
            place_ty_path,
            option_ty_path,
            slice_ty_path,
            cyclic_slice_leashed_ty_path,
            string_literal_ty_path,
            str_ty_path,
            ref_ty_path,
            ref_mut_ty_path,
            leash_ty_path,
            at_ty_path,
            html_ty_path,
            vec_ty_path,
            array_ty_path,
            core_ops_add_trai_path,
            add_assign_trai_path,
            bit_and_trai_path,
            bit_and_assign_trai_path,
            bit_or_trai_path,
            core_ops_bit_or_assign_trai_path,
            bit_xor_trai_path,
            bit_xor_assign_trai_path,
            div_trai_path,
            div_assign_trai_path,
            int_index_trai_path,
            mul_trai_path,
            mul_assign_trai_path,
            neg_trai_path,
            not_trai_path,
            unveil_trai_path,
            clone_trai_path,
            copy_trai_path,
            default_trai_path,
            result_ty_path,
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

    pub fn place_ty_path(&self) -> TypePath {
        self.place_ty_path
    }

    pub fn option_ty_path(&self) -> TypePath {
        self.option_ty_path
    }

    pub fn slice_ty_path(&self) -> TypePath {
        self.slice_ty_path
    }

    pub fn cyclic_slice_leashed_ty_path(&self) -> TypePath {
        self.cyclic_slice_leashed_ty_path
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

    pub fn at_ty_path(&self) -> TypePath {
        self.at_ty_path
    }

    pub fn result_ty_path(&self) -> TypePath {
        self.result_ty_path
    }

    pub fn vec_ty_path(&self) -> TypePath {
        self.vec_ty_path
    }

    pub fn array_ty_path(&self) -> TypePath {
        self.array_ty_path
    }

    pub fn leash_ty_path(&self) -> TypePath {
        self.leash_ty_path
    }

    pub fn add_trai_path(&self) -> TraitPath {
        self.core_ops_add_trai_path
    }

    pub fn add_assign_trai_path(&self) -> TraitPath {
        self.add_assign_trai_path
    }

    pub fn bit_and_trai_path(&self) -> TraitPath {
        self.bit_and_trai_path
    }

    pub fn bit_and_assign_trai_path(&self) -> TraitPath {
        self.bit_and_assign_trai_path
    }

    pub fn bit_or_trai_path(&self) -> TraitPath {
        self.bit_or_trai_path
    }

    pub fn core_ops_bit_or_assign_trai_path(&self) -> TraitPath {
        self.core_ops_bit_or_assign_trai_path
    }

    pub fn bit_xor_trai_path(&self) -> TraitPath {
        self.bit_xor_trai_path
    }

    pub fn bit_xor_assign_trai_path(&self) -> TraitPath {
        self.bit_xor_assign_trai_path
    }

    pub fn div_trai_path(&self) -> TraitPath {
        self.div_trai_path
    }

    pub fn div_assign_trai_path(&self) -> TraitPath {
        self.div_assign_trai_path
    }

    pub fn int_index_trai_path(&self) -> TraitPath {
        self.int_index_trai_path
    }

    pub fn mul_trai_path(&self) -> TraitPath {
        self.mul_trai_path
    }

    pub fn mul_assign_trai_path(&self) -> TraitPath {
        self.mul_assign_trai_path
    }

    pub fn neg_trai_path(&self) -> TraitPath {
        self.neg_trai_path
    }

    pub fn not_trai_path(&self) -> TraitPath {
        self.not_trai_path
    }

    pub fn unveil_trai_path(&self) -> TraitPath {
        self.unveil_trai_path
    }

    pub fn clone_trai_path(&self) -> TraitPath {
        self.clone_trai_path
    }

    pub fn copy_trai_path(&self) -> TraitPath {
        self.copy_trai_path
    }

    pub fn default_trai_path(&self) -> TraitPath {
        self.default_trai_path
    }

    pub fn html_ty_path(&self) -> TypePath {
        self.html_ty_path
    }
}

#[test]
fn menu_works() {
    let db = DB::default();
    let db = &*db;
    let toolchain = db.dev_toolchain().unwrap();
    let item_path_menu = db.item_path_menu(toolchain);
    assert_eq!(item_path_menu.add_trai_path().show(db), "core::ops::Add");
    assert_eq!(
        item_path_menu.add_assign_trai_path().show(db),
        "core::ops::AddAssign"
    );
    assert_eq!(
        item_path_menu.bit_and_trai_path().show(db),
        "core::ops::BitAnd"
    );
    assert_eq!(
        item_path_menu.bit_and_assign_trai_path().show(db),
        "core::ops::BitAndAssign"
    );
    assert_eq!(
        item_path_menu.bit_or_trai_path().show(db),
        "core::ops::BitOr"
    );
    assert_eq!(
        item_path_menu.core_ops_bit_or_assign_trai_path().show(db),
        "core::ops::BitOrAssign"
    );
    assert_eq!(
        item_path_menu.bit_xor_trai_path().show(db),
        "core::ops::BitXor"
    );
    assert_eq!(
        item_path_menu.bit_xor_assign_trai_path().show(db),
        "core::ops::BitXorAssign"
    );
    assert_eq!(item_path_menu.div_trai_path().show(db), "core::ops::Div");
    assert_eq!(
        item_path_menu.div_assign_trai_path().show(db),
        "core::ops::DivAssign"
    );
    assert_eq!(item_path_menu.mul_trai_path().show(db), "core::ops::Mul");
    assert_eq!(
        item_path_menu.mul_assign_trai_path().show(db),
        "core::ops::MulAssign"
    );
    assert_eq!(item_path_menu.neg_trai_path().show(db), "core::ops::Neg");
    assert_eq!(item_path_menu.not_trai_path().show(db), "core::ops::Not");
}
