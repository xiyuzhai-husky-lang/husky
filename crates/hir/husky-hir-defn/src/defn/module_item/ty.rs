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
    Enum(EnumTypeHirDefn),
    PropsStruct(PropsStructTypeHirDefn),
    TupleStruct(TupleStructTypeHirDefn),
    UnitStruct(UnitStructTypeHirDefn),
    Extern(ExternTypeHirDefn),
    Union(UnionTypeHirDefn),
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
        TypeHirDecl::Enum(hir_decl) => EnumTypeHirDefn::new(db, path, hir_decl).into(),
        TypeHirDecl::PropsStruct(hir_decl) => {
            PropsStructTypeHirDefn::new(db, path, hir_decl).into()
        }
        TypeHirDecl::TupleStruct(hir_decl) => {
            TupleStructTypeHirDefn::new(db, path, hir_decl).into()
        }
        TypeHirDecl::UnitStruct(hir_decl) => UnitStructTypeHirDefn::new(db, path, hir_decl).into(),
        TypeHirDecl::Record(hir_decl) => todo!(),
        // RecordTypeHirDefn::new(db, path, hir_decl).into(),
        TypeHirDecl::Extern(hir_decl) => ExternTypeHirDefn::new(db, path, hir_decl).into(),
        TypeHirDecl::Union(hir_decl) => UnionTypeHirDefn::new(db, path, hir_decl).into(),
    })
}
