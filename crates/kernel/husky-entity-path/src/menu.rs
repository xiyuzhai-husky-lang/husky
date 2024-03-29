use husky_coword::coword_menu;

use crate::*;

#[salsa::tracked(jar = EntityPathJar, return_ref)]
pub fn item_path_menu(db: &::salsa::Db, toolchain: Toolchain) -> ItemPathMenu {
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
    bit_or_assign_trai_path: TraitPath,
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
    debug_trai_path: TraitPath,
    clone_trai_path: TraitPath,
    copy_trai_path: TraitPath,
    pub partial_eq_trai_path: TraitPath,
    pub eq_trai_path: TraitPath,
    pub partial_ord_trai_path: TraitPath,
    pub ord_trai_path: TraitPath,
    // hash_trai_path: TraitPath,
    default_trai_path: TraitPath,
    visualize_trai_path: TraitPath,
    option_ty_path: TypePath,
    result_ty_path: TypePath,
    slice_ty_path: TypePath,
    cyclic_slice_leashed_ty_path: TypePath,
    string_literal_ty_path: TypePath,
    str_ty_path: TypePath,
    ref_ty_path: TypePath,
    ref_mut_ty_path: TypePath,
    at_ty_path: TypePath,
    universe_ty_path: TypePath,
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
    visual_ty_path: TypePath,
    trai_ty_path: TypePath,
    lifetime_ty_path: TypePath,
    place_ty_path: TypePath,
    module_ty_path: TypePath,
}

impl ItemPathMenu {
    pub(crate) fn new(db: &::salsa::Db, toolchain: Toolchain) -> Self {
        let coword_menu = coword_menu(db);
        let path_menu = db.vfs_path_menu(toolchain);
        let core_ops = path_menu.core_ops().inner();
        let core_option = path_menu.core_option().inner();
        let core_slice = path_menu.core_slice().inner();
        let core_str = path_menu.core_str().inner();
        let core_basic = path_menu.core_basic().inner();
        let core_clone = path_menu.core_clone().inner();
        let core_cmp = path_menu.core_cmp().inner();
        let core_fmt = path_menu.core_fmt().inner();
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
            core_option,
            Ident::from_ref(db, "Option").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Enum,
            db,
        );
        let result_ty_path = TypePath::new(
            core_result,
            Ident::from_ref(db, "Result").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Enum,
            db,
        );
        let unit_ty_path = TypePath::new(
            core_basic,
            coword_menu.unit_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let never_ty_path = TypePath::new(
            core_basic,
            coword_menu.never_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let bool_ty_path = TypePath::new(
            core_basic,
            coword_menu.bool_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let i8_ty_path = TypePath::new(
            core_num,
            coword_menu.i8_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let i16_ty_path = TypePath::new(
            core_num,
            coword_menu.i16_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let i32_ty_path = TypePath::new(
            core_num,
            coword_menu.i32_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let i64_ty_path = TypePath::new(
            core_num,
            coword_menu.i64_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let i128_ty_path = TypePath::new(
            core_num,
            coword_menu.i128_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let isize_ty_path = TypePath::new(
            core_num,
            coword_menu.isize_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let u8_ty_path = TypePath::new(
            core_num,
            coword_menu.u8_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let u16_ty_path = TypePath::new(
            core_num,
            coword_menu.u16_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let u32_ty_path = TypePath::new(
            core_num,
            coword_menu.u32_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let u64_ty_path = TypePath::new(
            core_num,
            coword_menu.u64_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let u128_ty_path = TypePath::new(
            core_num,
            coword_menu.u128_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let usize_ty_path = TypePath::new(
            core_num,
            coword_menu.usize_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let r8_ty_path = TypePath::new(
            core_raw_bits,
            coword_menu.r8_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let r16_ty_path = TypePath::new(
            core_raw_bits,
            coword_menu.r16_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let r32_ty_path = TypePath::new(
            core_raw_bits,
            coword_menu.r32_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let r64_ty_path = TypePath::new(
            core_raw_bits,
            coword_menu.r64_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let r128_ty_path = TypePath::new(
            core_raw_bits,
            coword_menu.r128_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let rsize_ty_path = TypePath::new(
            core_raw_bits,
            coword_menu.rsize_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let f32_ty_path = TypePath::new(
            core_num,
            coword_menu.f32_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let f64_ty_path = TypePath::new(
            core_num,
            coword_menu.f64_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let lifetime_ty_path = TypePath::new(
            core_basic,
            coword_menu.lifetime_ty_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let place_ty_path = TypePath::new(
            core_basic,
            coword_menu.place_ty_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let module_ty_path = TypePath::new(
            core_basic,
            coword_menu.module_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let visual_ty_path = TypePath::new(
            core_visual,
            Ident::from_ref(db, "Visual").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let trai_ty_path = TypePath::new(
            core_basic,
            coword_menu.trai_ty_ident(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let ref_ty_path = TypePath::new(
            core_mem,
            Ident::from_ref(db, "Ref").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let ref_mut_ty_path = TypePath::new(
            core_mem,
            Ident::from_ref(db, "RefMut").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let leash_ty_path = TypePath::new(
            core_mem,
            Ident::from_ref(db, "Leash").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let at_ty_path = TypePath::new(
            core_mem,
            Ident::from_ref(db, "At").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let universe_ty_path = TypePath::new(
            core_basic,
            Ident::from_ref(db, "Universe").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let vec_ty_path = TypePath::new(
            core_vec,
            Ident::from_ref(db, "Vec").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let array_ty_path = TypePath::new(
            core_array,
            Ident::from_ref(db, "Array").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let slice_ty_path = TypePath::new(
            core_slice,
            Ident::from_ref(db, "Slice").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let cyclic_slice_leashed_ty_path = TypePath::new(
            core_slice,
            Ident::from_ref(db, "CyclicSlice").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let string_literal_ty_path = TypePath::new(
            core_str,
            Ident::from_ref(db, "StringLiteral").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let str_ty_path = TypePath::new(
            core_str,
            Ident::from_ref(db, "str").unwrap(),
            MajorItemConnection::Connected,
            TypeKind::Extern,
            db,
        );
        let core_ops_add_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "Add").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let add_assign_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "AddAssign").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let bit_and_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "BitAnd").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let bit_and_assign_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "BitAndAssign").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let bit_or_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "BitOr").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let bit_or_assign_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "BitOrAssign").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let bit_xor_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "BitXor").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let bit_xor_assign_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "BitXorAssign").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let div_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "Div").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let div_assign_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "DivAssign").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let int_index_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "IntIndex").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let mul_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "Mul").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let mul_assign_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "MulAssign").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let neg_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "Neg").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let not_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "Not").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let unveil_trai_path = TraitPath::new(
            core_ops,
            Ident::from_ref(db, "Unveil").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let debug_trai_path = TraitPath::new(
            core_fmt,
            Ident::from_ref(db, "Debug").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let clone_trai_path = TraitPath::new(
            core_clone,
            Ident::from_ref(db, "Clone").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let copy_trai_path = TraitPath::new(
            core_marker,
            Ident::from_ref(db, "Copy").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let partial_eq_trai_path = TraitPath::new(
            core_cmp,
            Ident::from_ref(db, "PartialEq").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let eq_trai_path = TraitPath::new(
            core_cmp,
            Ident::from_ref(db, "Eq").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let partial_ord_trai_path = TraitPath::new(
            core_cmp,
            Ident::from_ref(db, "PartialOrd").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let ord_trai_path = TraitPath::new(
            core_cmp,
            Ident::from_ref(db, "Ord").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let default_trai_path = TraitPath::new(
            core_default,
            Ident::from_ref(db, "Default").unwrap(),
            MajorItemConnection::Connected,
            db,
        );
        let visualize_trai_path = TraitPath::new(
            core_visual,
            Ident::from_ref(db, "Visualize").unwrap(),
            MajorItemConnection::Connected,
            db,
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
            universe_ty_path,
            visual_ty_path,
            vec_ty_path,
            array_ty_path,
            core_ops_add_trai_path,
            add_assign_trai_path,
            bit_and_trai_path,
            bit_and_assign_trai_path,
            bit_or_trai_path,
            bit_or_assign_trai_path,
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
            debug_trai_path,
            clone_trai_path,
            copy_trai_path,
            partial_eq_trai_path,
            eq_trai_path,
            partial_ord_trai_path,
            ord_trai_path,
            default_trai_path,
            visualize_trai_path,
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

    pub fn universe_ty_path(&self) -> TypePath {
        self.universe_ty_path
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

    pub fn bit_or_assign_trai_path(&self) -> TraitPath {
        self.bit_or_assign_trai_path
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

    pub fn debug_trai_path(&self) -> TraitPath {
        self.debug_trai_path
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

    pub fn visualize_trai_path(&self) -> TraitPath {
        self.visualize_trai_path
    }

    pub fn visual_ty_path(&self) -> TypePath {
        self.visual_ty_path
    }
}

#[test]
fn menu_works() {
    use husky_vfs::test_utils::db::VfsTestUtilsDb;

    let db = DB::default();
    let db = &*db;
    let toolchain = db.dev_toolchain().unwrap();
    let item_path_menu = item_path_menu(db, toolchain);
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
        item_path_menu.bit_or_assign_trai_path().show(db),
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
