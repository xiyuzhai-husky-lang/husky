mod r#enum;
mod r#extern;
mod props_struct;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::tuple_struct::*;
pub use self::union::*;
pub use self::unit_struct::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum TypeHirDefn {
    Enum(EnumHirDefn),
    PropsStruct(PropsStructHirDefn),
    TupleStruct(TupleStructHirDefn),
    UnitStruct(UnitStructHirDefn),
    Extern(ExternHirDefn),
    Union(UnionHirDefn),
}

impl TypeHirDefn {
    pub fn path(self, db: &dyn HirDefnDb) -> TypePath {
        match self {
            TypeHirDefn::Enum(hir_defn) => hir_defn.path(db),
            TypeHirDefn::UnitStruct(hir_defn) => hir_defn.path(db),
            TypeHirDefn::TupleStruct(hir_defn) => hir_defn.path(db),
            TypeHirDefn::PropsStruct(hir_defn) => hir_defn.path(db),
            TypeHirDefn::Extern(hir_defn) => hir_defn.path(db),
            TypeHirDefn::Union(hir_defn) => hir_defn.path(db),
        }
    }

    pub fn hir_decl(self, db: &dyn HirDefnDb) -> TypeHirDecl {
        match self {
            TypeHirDefn::Enum(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeHirDefn::UnitStruct(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeHirDefn::TupleStruct(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeHirDefn::PropsStruct(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeHirDefn::Extern(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeHirDefn::Union(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        match self {
            TypeHirDefn::Enum(hir_defn) => hir_defn.dependencies(db),
            TypeHirDefn::PropsStruct(hir_defn) => hir_defn.dependencies(db),
            TypeHirDefn::TupleStruct(hir_defn) => hir_defn.dependencies(db),
            TypeHirDefn::UnitStruct(hir_defn) => hir_defn.dependencies(db),
            TypeHirDefn::Extern(hir_defn) => hir_defn.dependencies(db),
            TypeHirDefn::Union(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        match self {
            TypeHirDefn::Enum(hir_defn) => hir_defn.version_stamp(db),
            TypeHirDefn::PropsStruct(hir_defn) => hir_defn.version_stamp(db),
            TypeHirDefn::TupleStruct(hir_defn) => hir_defn.version_stamp(db),
            TypeHirDefn::UnitStruct(hir_defn) => hir_defn.version_stamp(db),
            TypeHirDefn::Extern(hir_defn) => hir_defn.version_stamp(db),
            TypeHirDefn::Union(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for TypePath {
    type HirDefn = TypeHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        ty_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn ty_hir_defn(db: &dyn HirDefnDb, path: TypePath) -> Option<TypeHirDefn> {
    Some(match path.hir_decl(db)? {
        TypeHirDecl::Enum(hir_decl) => EnumHirDefn::new(db, path, hir_decl).into(),
        TypeHirDecl::PropsStruct(hir_decl) => PropsStructHirDefn::new(db, path, hir_decl).into(),
        TypeHirDecl::TupleStruct(hir_decl) => TupleStructHirDefn::new(db, path, hir_decl).into(),
        TypeHirDecl::UnitStruct(hir_decl) => UnitStructHirDefn::new(db, path, hir_decl).into(),
        TypeHirDecl::Record(_hir_decl) => todo!(),
        // RecordTypeHirDefn::new(db, path, hir_decl).into(),
        TypeHirDecl::Extern(hir_decl) => ExternHirDefn::new(db, path, hir_decl).into(),
        TypeHirDecl::Union(hir_decl) => UnionHirDefn::new(db, path, hir_decl).into(),
    })
}
