use husky_entity_taxonomy::ModuleItemConnection;

use crate::*;

#[salsa::tracked(jar = EntityPathJar, return_ref)]
pub(crate) fn entity_path_menu(
    db: &dyn EntityPathDb,
    toolchain: Toolchain,
) -> EntityPathResult<EntityPathMenu> {
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
    core_slice_slice_type: TypePath,
    // prelude
    unit: TypePath,
    bool: TypePath,
    i32: TypePath,
    i64: TypePath,
    f32: TypePath,
    f64: TypePath,
    r32: TypePath,
    b64: TypePath,
    u32: TypePath,
    u64: TypePath,
    trai: TypePath,
    module: TypePath,
}

impl EntityPathMenu {
    pub(crate) fn new(db: &dyn EntityPathDb, toolchain: Toolchain) -> EntityPathResult<Self> {
        let word_menu = db.word_menu();
        let path_menu = db.vfs_path_menu(toolchain)?;
        let core_ops = path_menu.core_ops();
        let core_slice = path_menu.core_slice();
        let core_basic = path_menu.core_basic();
        let core_num = path_menu.core_num();
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
        let core_slice_slice_type = TypePath::new(
            db,
            core_slice,
            db.it_ident_borrowed("Slice").unwrap(),
            ModuleItemConnection::Connected,
            TypeKind::Alien,
        );
        let i32 = TypePath::new(
            db,
            core_num,
            word_menu.i32(),
            ModuleItemConnection::Connected,
            TypeKind::Alien,
        );
        let i64 = TypePath::new(
            db,
            core_num,
            word_menu.i64(),
            ModuleItemConnection::Connected,
            TypeKind::Alien,
        );
        let unit = TypePath::new(
            db,
            core_basic,
            word_menu.unit(),
            ModuleItemConnection::Connected,
            TypeKind::Alien,
        );
        let bool = TypePath::new(
            db,
            core_basic,
            word_menu.bool(),
            ModuleItemConnection::Connected,
            TypeKind::Alien,
        );
        let f32 = TypePath::new(
            db,
            core_num,
            word_menu.f32(),
            ModuleItemConnection::Connected,
            TypeKind::Alien,
        );
        let f64 = TypePath::new(
            db,
            core_num,
            word_menu.f64(),
            ModuleItemConnection::Connected,
            TypeKind::Alien,
        );
        let r32 = TypePath::new(
            db,
            core_num,
            word_menu.r32(),
            ModuleItemConnection::Connected,
            TypeKind::Alien,
        );
        let b64 = TypePath::new(
            db,
            core_num,
            word_menu.b64(),
            ModuleItemConnection::Connected,
            TypeKind::Alien,
        );
        let u32 = TypePath::new(
            db,
            core_num,
            word_menu.u32(),
            ModuleItemConnection::Connected,
            TypeKind::Alien,
        );
        let u64 = TypePath::new(
            db,
            core_num,
            word_menu.u64(),
            ModuleItemConnection::Connected,
            TypeKind::Alien,
        );
        let trai = TypePath::new(
            db,
            core_num,
            word_menu.trai(),
            ModuleItemConnection::Connected,
            TypeKind::Alien,
        );
        let module = TypePath::new(
            db,
            core_num,
            word_menu.module(),
            ModuleItemConnection::Connected,
            TypeKind::Alien,
        );
        Ok(Self {
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
            core_slice_slice_type,
            unit,
            bool,
            i32,
            i64,
            f32,
            f64,
            r32,
            b64,
            u32,
            u64,
            trai,
            module,
        })
    }

    pub fn i32(&self) -> TypePath {
        self.i32
    }

    pub fn i64(&self) -> TypePath {
        self.i64
    }

    pub fn f32(&self) -> TypePath {
        self.f32
    }

    pub fn f64(&self) -> TypePath {
        self.f64
    }

    pub fn r32(&self) -> TypePath {
        self.r32
    }

    pub fn b64(&self) -> TypePath {
        self.b64
    }

    pub fn u32(&self) -> TypePath {
        self.u32
    }

    pub fn u64(&self) -> TypePath {
        self.u64
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

    pub fn core_slice_slice_type(&self) -> TypePath {
        self.core_slice_slice_type
    }

    pub fn trai(&self) -> TypePath {
        self.trai
    }

    pub fn module(&self) -> TypePath {
        self.module
    }

    pub fn unit(&self) -> TypePath {
        self.unit
    }

    pub fn bool(&self) -> TypePath {
        self.bool
    }
}
