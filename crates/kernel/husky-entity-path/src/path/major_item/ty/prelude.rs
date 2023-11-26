use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
#[salsa::debug_with_db(db = EntityPathDb, jar = EntityPathJar)]
pub enum PreludeTypePath {
    Basic(PreludeBasicTypePath),
    // stack representable numbers
    Num(PreludeNumTypePath),
    Indirection(PreludeIndirectionTypePath),
    Arr(PreludeArrTypePath),
    // RawBits
    Nat,
    Lifetime,
    Module,
    Trait,
    List,
    StringLiteral,
    Str,
    Option,
    Result,
}

impl PreludeTypePath {
    pub const UNIT: Self = PreludeTypePath::Basic(PreludeBasicTypePath::Unit);
    pub const NEVER: Self = PreludeTypePath::Basic(PreludeBasicTypePath::Never);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreludeBasicTypePath {
    Unit,
    Never,
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreludeIndirectionTypePath {
    Ref,
    RefMut,
    Leash,
    At,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntityPathDb, jar = EntityPathJar)]
pub enum PreludeArrTypePath {
    Vec,
    Array,
    Array2d,
    Array3d,
    Array4d,
    Array5d,
    Slice,
    CyclicSlice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum PreludeNumTypePath {
    Int(PreludeIntTypePath),
    Float(PreludeFloatTypePath),
}

impl From<PreludeIntTypePath> for PreludeTypePath {
    fn from(path: PreludeIntTypePath) -> Self {
        PreludeTypePath::Num(path.into())
    }
}

impl From<PreludeFloatTypePath> for PreludeTypePath {
    fn from(path: PreludeFloatTypePath) -> Self {
        PreludeTypePath::Num(path.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreludeIntTypePath {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    // todo: move R* out of here
    R8,
    R16,
    R32,
    R64,
    R128,
    RSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreludeFloatTypePath {
    F32,
    F64,
}

#[test]
fn prelude_ty_path_size_works() {
    assert!(std::mem::size_of::<PreludeTypePath>() < std::mem::size_of::<i32>())
}

impl TypePath {
    pub fn prelude_ty_path(self, db: &::salsa::Db) -> Option<PreludeTypePath> {
        prelude_ty_path(db, self)
    }
}

#[salsa::tracked(jar = EntityPathJar)]
pub(crate) fn prelude_ty_path(db: &::salsa::Db, path: TypePath) -> Option<PreludeTypePath> {
    let menu: &ItemPathMenu = item_path_menu(db, path.toolchain(db));
    let vfs_path_menu: &VfsPathMenu = db.vfs_path_menu(path.toolchain(db));
    if path.crate_path(db) != vfs_path_menu.core_library() {
        return None;
    }
    Some(match path {
        path if path == menu.unit_ty_path() => PreludeBasicTypePath::Unit.into(),
        path if path == menu.never_ty_path() => PreludeBasicTypePath::Never.into(),
        path if path == menu.bool_ty_path() => PreludeBasicTypePath::Bool.into(),
        path if path == menu.i8_ty_path() => PreludeIntTypePath::I8.into(),
        path if path == menu.i16_ty_path() => PreludeIntTypePath::I16.into(),
        path if path == menu.i32_ty_path() => PreludeIntTypePath::I32.into(),
        path if path == menu.i64_ty_path() => PreludeIntTypePath::I64.into(),
        path if path == menu.i128_ty_path() => PreludeIntTypePath::I128.into(),
        path if path == menu.isize_ty_path() => PreludeIntTypePath::ISize.into(),
        path if path == menu.u8_ty_path() => PreludeIntTypePath::U8.into(),
        path if path == menu.u16_ty_path() => PreludeIntTypePath::U16.into(),
        path if path == menu.u32_ty_path() => PreludeIntTypePath::U32.into(),
        path if path == menu.u64_ty_path() => PreludeIntTypePath::U64.into(),
        path if path == menu.u128_ty_path() => PreludeIntTypePath::U128.into(),
        path if path == menu.usize_ty_path() => PreludeIntTypePath::USize.into(),
        path if path == menu.r8_ty_path() => PreludeIntTypePath::R8.into(),
        path if path == menu.r16_ty_path() => PreludeIntTypePath::R16.into(),
        path if path == menu.r32_ty_path() => PreludeIntTypePath::R32.into(),
        path if path == menu.r64_ty_path() => PreludeIntTypePath::R64.into(),
        path if path == menu.r128_ty_path() => PreludeIntTypePath::R128.into(),
        path if path == menu.rsize_ty_path() => PreludeIntTypePath::RSize.into(),
        path if path == menu.f32_ty_path() => PreludeFloatTypePath::F32.into(),
        path if path == menu.f64_ty_path() => PreludeFloatTypePath::F64.into(),
        path if path == menu.lifetime_ty_path() => PreludeTypePath::Lifetime.into(),
        path if path == menu.module_ty_path() => PreludeTypePath::Module.into(),
        path if path == menu.trai_ty_path() => PreludeTypePath::Trait.into(),
        path if path == menu.ref_ty_path() => PreludeIndirectionTypePath::Ref.into(),
        path if path == menu.ref_mut_ty_path() => PreludeIndirectionTypePath::RefMut.into(),
        path if path == menu.leash_ty_path() => PreludeIndirectionTypePath::Leash.into(),
        path if path == menu.at_ty_path() => PreludeIndirectionTypePath::At.into(),
        path if path == menu.vec_ty_path() => PreludeArrTypePath::Vec.into(),
        path if path == menu.slice_ty_path() => PreludeArrTypePath::Slice.into(),
        path if path == menu.cyclic_slice_leashed_ty_path() => {
            PreludeArrTypePath::CyclicSlice.into()
        }
        path if path == menu.string_literal_ty_path() => PreludeTypePath::StringLiteral.into(),
        path if path == menu.str_ty_path() => PreludeTypePath::Str.into(),
        path if path == menu.option_ty_path() => PreludeTypePath::Option.into(),
        path if path == menu.result_ty_path() => PreludeTypePath::Result.into(),
        _ => return None,
    })
}
