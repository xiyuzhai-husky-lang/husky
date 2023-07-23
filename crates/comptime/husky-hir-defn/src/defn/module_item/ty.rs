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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = HirDefnDb)]
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
    pub fn decl(self, db: &dyn HirDefnDb) -> TypeHirDecl {
        match self {
            TypeHirDefn::Enum(defn) => defn.decl(db).into(),
            TypeHirDefn::UnitStruct(defn) => defn.decl(db).into(),
            TypeHirDefn::TupleStruct(defn) => defn.decl(db).into(),
            TypeHirDefn::PropsStruct(defn) => defn.decl(db).into(),
            TypeHirDefn::Extern(defn) => defn.decl(db).into(),
            TypeHirDefn::Union(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn HirDefnDb) -> TypePath {
        todo!()
        // match self {
        //     TypeHirDefn::Enum(defn) => defn.path(db),
        //     TypeHirDefn::Inductive(defn) => defn.path(db),
        //     TypeHirDefn::Record(defn) => defn.path(db),
        //     TypeHirDefn::UnitStruct(defn) => defn.path(db),
        //     TypeHirDefn::TupleStruct(defn) => defn.path(db),
        //     TypeHirDefn::PropsStruct(defn) => defn.path(db),
        //     TypeHirDefn::Structure(defn) => defn.path(db),
        //     TypeHirDefn::Extern(defn) => defn.path(db),
        //     TypeHirDefn::Union(defn) => defn.path(db),
        // }
    }
}

impl HasHirDefn for TypePath {
    type HirDefn = TypeHirDefn;

    fn syn_defn(self, db: &dyn HirDefnDb) -> HirDefnResult<Self::HirDefn> {
        ty_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn ty_defn(db: &dyn HirDefnDb, path: TypePath) -> HirDefnResult<TypeHirDefn> {
    Ok(match path.decl(db)? {
        TypeDecl::Enum(decl) => EnumTypeHirDefn::new(db, path, decl).into(),
        TypeDecl::PropsStruct(decl) => PropsStructTypeHirDefn::new(db, path, decl).into(),
        TypeDecl::TupleStruct(decl) => TupleStructTypeHirDefn::new(db, path, decl).into(),
        TypeDecl::UnitStruct(decl) => UnitStructTypeHirDefn::new(db, path, decl).into(),
        TypeDecl::Record(decl) => RecordTypeHirDefn::new(db, path, decl).into(),
        TypeDecl::Inductive(decl) => InductiveTypeHirDefn::new(db, path, decl).into(),
        TypeDecl::Structure(decl) => StructureTypeHirDefn::new(db, path, decl).into(),
        TypeDecl::Extern(decl) => ExternTypeHirDefn::new(db, path, decl).into(),
        TypeDecl::Union(decl) => UnionTypeHirDefn::new(db, path, decl).into(),
    })
}
