use super::*;
use std::fmt::Debug;

#[salsa::interned(jar = EntityPathJar, override_debug)]
pub struct TypePath {
    pub module_path: ModulePath,
    pub ident: Identifier,
    pub connection: ModuleItemConnection,
    pub ty_kind: TypeKind,
}

impl TypePath {
    pub fn eqs_lifetime_ty_path(self, db: &dyn EntityPathDb) -> EntityPathResult<bool> {
        Ok(self.prelude_ty_path(db)? == Some(PreludeTypePath::Lifetime))
    }

    pub fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        self.module_path(db).show_aux(f, db)?;
        f.write_str(show_connection(self.connection(db)))?;
        f.write_str(self.ident(db).data(db))
    }

    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        self.module_path(db).crate_path(db)
    }

    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.crate_path(db).toolchain(db)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityPathDb, jar = EntityPathJar)]
pub enum PreludeTypePath {
    Unit,
    Never,
    Bool,
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
    R8,
    R16,
    R32,
    R64,
    R128,
    RSize,
    F32,
    F64,
    Nat,
    Lifetime,
    Module,
    Trait,
    Ref,
    RefMut,
    List,
    Array,
    Array2d,
    Array3d,
    Array4d,
    Array5d,
    Slice,
    StringLiteral,
    Str,
}

impl TypePath {
    pub fn prelude_ty_path(
        self,
        db: &dyn EntityPathDb,
    ) -> EntityPathResult<Option<PreludeTypePath>> {
        prelude_ty_path(db, self)
    }
}

#[salsa::tracked(jar = EntityPathJar)]
pub(crate) fn prelude_ty_path(
    db: &dyn EntityPathDb,
    path: TypePath,
) -> EntityPathResult<Option<PreludeTypePath>> {
    let menu: &EntityPathMenu = db.entity_path_menu(path.toolchain(db))?;
    let vfs_path_menu: &VfsPathMenu = db.vfs_path_menu(path.toolchain(db))?;
    if path.crate_path(db) != vfs_path_menu.core_library() {
        return Ok(None);
    }
    Ok(Some(match path {
        path if path == menu.unit_ty_path() => PreludeTypePath::Unit,
        path if path == menu.never_ty_path() => PreludeTypePath::Never,
        path if path == menu.bool_ty_path() => PreludeTypePath::Bool,
        path if path == menu.i8_ty_path() => PreludeTypePath::I8,
        path if path == menu.i16_ty_path() => PreludeTypePath::I16,
        path if path == menu.i32_ty_path() => PreludeTypePath::I32,
        path if path == menu.i64_ty_path() => PreludeTypePath::I64,
        path if path == menu.i128_ty_path() => PreludeTypePath::I128,
        path if path == menu.isize_ty_path() => PreludeTypePath::ISize,
        path if path == menu.u8_ty_path() => PreludeTypePath::U8,
        path if path == menu.u16_ty_path() => PreludeTypePath::U16,
        path if path == menu.u32_ty_path() => PreludeTypePath::U32,
        path if path == menu.u64_ty_path() => PreludeTypePath::U64,
        path if path == menu.u128_ty_path() => PreludeTypePath::U128,
        path if path == menu.usize_ty_path() => PreludeTypePath::USize,
        path if path == menu.r8_ty_path() => PreludeTypePath::R8,
        path if path == menu.r16_ty_path() => PreludeTypePath::R16,
        path if path == menu.r32_ty_path() => PreludeTypePath::R32,
        path if path == menu.r64_ty_path() => PreludeTypePath::R64,
        path if path == menu.r128_ty_path() => PreludeTypePath::R128,
        path if path == menu.rsize_ty_path() => PreludeTypePath::RSize,
        path if path == menu.f32_ty_path() => PreludeTypePath::F32,
        path if path == menu.f64_ty_path() => PreludeTypePath::F64,
        path if path == menu.lifetime_ty_path() => PreludeTypePath::Lifetime,
        path if path == menu.module_ty_path() => PreludeTypePath::Module,
        path if path == menu.trai_ty_path() => PreludeTypePath::Trait,
        path if path == menu.ref_ty_path() => PreludeTypePath::Ref,
        path if path == menu.ref_mut_ty_path() => PreludeTypePath::RefMut,
        path if path == menu.list_ty_path() => PreludeTypePath::List,
        path if path == menu.slice_ty_path() => PreludeTypePath::Slice,
        path if path == menu.string_literal_ty_path() => PreludeTypePath::StringLiteral,
        path if path == menu.str_ty_path() => PreludeTypePath::Str,
        _ => return Ok(None),
    }))
}

impl<Db: EntityPathDb + ?Sized> salsa::DebugWithDb<Db> for TypePath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<EntityPathJar>>::as_jar_db(db);
        f.write_str("TypePath(`")?;
        self.show_aux(f, db)?;
        f.write_str("`, `")?;
        self.ty_kind(db).fmt(f)?;
        f.write_str("`)")
    }
}

impl<Db> salsa::DisplayWithDb<Db> for TypePath
where
    Db: EntityPathDb + ?Sized,
{
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityPathJar>>::as_jar_db(db);
        self.show_aux(f, db)
    }
}
