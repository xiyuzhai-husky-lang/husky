use super::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id(jar = EntityPathJar)]
#[salsa::deref_id]
pub struct TypePath(ItemPathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TypePathData {
    module_path: ModulePath,
    ident: Ident,
    connection: MajorItemConnection,
    ty_kind: TypeKind,
}

impl TypePath {
    pub fn new(
        module_path: ModulePath,
        ident: Ident,
        connection: MajorItemConnection,
        ty_kind: TypeKind,
        db: &::salsa::Db,
    ) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::MajorItem(MajorItemPathData::Type(TypePathData {
                module_path,
                ident,
                connection,
                ty_kind,
            })),
        ))
    }

    pub fn eqs_lifetime_ty_path(self, db: &::salsa::Db) -> bool {
        self.prelude_ty_path(db) == Some(PreludeTypePath::LIFETIME)
    }

    pub fn eqs_place_ty_path(self, db: &::salsa::Db) -> bool {
        self.prelude_ty_path(db) == Some(PreludeTypePath::PLACE)
    }

    pub fn crate_path(self, db: &::salsa::Db) -> CratePath {
        self.module_path(db).crate_path(db)
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }

    pub fn data(self, db: &::salsa::Db) -> TypePathData {
        match self.0.data(db) {
            ItemPathData::MajorItem(MajorItemPathData::Type(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db).ident
    }

    pub fn ty_kind(self, db: &::salsa::Db) -> TypeKind {
        self.data(db).ty_kind
    }

    pub fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        self.data(db).show_aux(f, db)
    }
}

impl TypePathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> TypePath {
        TypePath(id)
    }

    pub(super) fn item_kind(self) -> EntityKind {
        EntityKind::MajorItem {
            module_item_kind: MajorItemKind::Type(self.ty_kind),
            connection: self.connection.kind(),
        }
    }

    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn connection(&self) -> MajorItemConnection {
        self.connection
    }

    pub fn ty_kind(&self) -> TypeKind {
        self.ty_kind
    }

    #[inline(never)]
    pub fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        self.module_path.show_aux(f, db)?;
        f.write_str(show_connection(self.connection))?;
        f.write_str(self.ident.data(db))
    }
}

impl salsa::DebugWithDb for TypePath {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        let data = self.data(db);
        f.write_str("TypePath(`")?;
        data.show_aux(f, db)?;
        f.write_str("`, `")?;
        data.ty_kind.fmt(f)?;
        f.write_str("`)")
    }
}

impl salsa::DisplayWithDb for TypePath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_aux(f, db)
    }
}

#[enum_class::from_variants]
#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreludeTypePath {
    Basic(PreludeBasicTypePath),
    // stack representable numbers
    Num(PreludeNumTypePath),
    Indirection(PreludeIndirectionTypePath),
    Container(PreludeContainerTypePath),
    // RawBits
    Nat,
    Lifetime,
    Place,
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
    pub const BOOL: Self = PreludeTypePath::Basic(PreludeBasicTypePath::Bool);
    pub const REF: Self = PreludeTypePath::Indirection(PreludeIndirectionTypePath::Ref);
    pub const REF_MUT: Self = PreludeTypePath::Indirection(PreludeIndirectionTypePath::RefMut);
    pub const NEVER: Self = PreludeTypePath::Basic(PreludeBasicTypePath::Never);
    pub const ARRAY: Self = PreludeTypePath::Container(PreludeContainerTypePath::Array);
    pub const VEC: Self = PreludeTypePath::Container(PreludeContainerTypePath::Vec);
    pub const SLICE: Self = PreludeTypePath::Container(PreludeContainerTypePath::Slice);
    pub const CYCLIC_SLICE: Self =
        PreludeTypePath::Container(PreludeContainerTypePath::CyclicSlice);
    pub const LIFETIME: Self = PreludeTypePath::Lifetime;
    pub const PLACE: Self = PreludeTypePath::Place;
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
#[salsa::debug_with_db]
pub enum PreludeContainerTypePath {
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
fn prelude_ty_path(db: &::salsa::Db, path: TypePath) -> Option<PreludeTypePath> {
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
        path if path == menu.lifetime_ty_path() => PreludeTypePath::LIFETIME,
        path if path == menu.place_ty_path() => PreludeTypePath::PLACE,
        path if path == menu.module_ty_path() => PreludeTypePath::Module.into(),
        path if path == menu.trai_ty_path() => PreludeTypePath::Trait.into(),
        path if path == menu.ref_ty_path() => PreludeIndirectionTypePath::Ref.into(),
        path if path == menu.ref_mut_ty_path() => PreludeIndirectionTypePath::RefMut.into(),
        path if path == menu.leash_ty_path() => PreludeIndirectionTypePath::Leash.into(),
        path if path == menu.at_ty_path() => PreludeIndirectionTypePath::At.into(),
        path if path == menu.vec_ty_path() => PreludeContainerTypePath::Vec.into(),
        path if path == menu.array_ty_path() => PreludeContainerTypePath::Array.into(),
        path if path == menu.slice_ty_path() => PreludeContainerTypePath::Slice.into(),
        path if path == menu.cyclic_slice_leashed_ty_path() => {
            PreludeContainerTypePath::CyclicSlice.into()
        }
        path if path == menu.string_literal_ty_path() => PreludeTypePath::StringLiteral.into(),
        path if path == menu.str_ty_path() => PreludeTypePath::Str.into(),
        path if path == menu.option_ty_path() => PreludeTypePath::Option.into(),
        path if path == menu.result_ty_path() => PreludeTypePath::Result.into(),
        _ => return None,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db]
pub struct CustomTypePath(TypePath);

impl From<CustomTypePath> for TypePath {
    fn from(path: CustomTypePath) -> Self {
        path.0
    }
}

impl TypePath {
    pub fn refine(self, db: &::salsa::Db) -> Either<PreludeTypePath, CustomTypePath> {
        match self.prelude_ty_path(db) {
            Some(path) => Left(path),
            None => Right(CustomTypePath(self)),
        }
    }
}
