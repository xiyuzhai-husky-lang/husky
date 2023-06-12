mod r#enum;
mod r#extern;
mod inductive;
mod props_struct;
mod record;
mod structure;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::inductive::*;
pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::record::*;
pub use self::structure::*;
pub use self::tuple_struct::*;
pub use self::union::*;
pub use self::unit_struct::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TypeDefn {
    Enum(EnumTypeDefn),
    Inductive(InductiveTypeDefn),
    Record(RecordTypeDefn),
    RegularStruct(RegularStructTypeDefn),
    TupleStruct(TupleStructTypeDefn),
    UnitStruct(UnitStructTypeDefn),
    Structure(StructureTypeDefn),
    Extern(ExternTypeDefn),
    Union(UnionTypeDefn),
}

impl TypeDefn {
    pub fn decl(self, db: &dyn DefnDb) -> TypeDecl {
        match self {
            TypeDefn::Enum(defn) => defn.decl(db).into(),
            TypeDefn::Inductive(defn) => defn.decl(db).into(),
            TypeDefn::Record(defn) => defn.decl(db).into(),
            TypeDefn::UnitStruct(defn) => defn.decl(db).into(),
            TypeDefn::TupleStruct(defn) => defn.decl(db).into(),
            TypeDefn::RegularStruct(defn) => defn.decl(db).into(),
            TypeDefn::Structure(defn) => defn.decl(db).into(),
            TypeDefn::Extern(defn) => defn.decl(db).into(),
            TypeDefn::Union(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn DefnDb) -> TypePath {
        todo!()
        // match self {
        //     TypeDefn::Enum(defn) => defn.path(db),
        //     TypeDefn::Inductive(defn) => defn.path(db),
        //     TypeDefn::Record(defn) => defn.path(db),
        //     TypeDefn::UnitStruct(defn) => defn.path(db),
        //     TypeDefn::TupleStruct(defn) => defn.path(db),
        //     TypeDefn::RegularStruct(defn) => defn.path(db),
        //     TypeDefn::Structure(defn) => defn.path(db),
        //     TypeDefn::Extern(defn) => defn.path(db),
        //     TypeDefn::Union(defn) => defn.path(db),
        // }
    }
}

impl HasDefn for TypeDecl {
    type Defn = TypeDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        match self {
            TypeDecl::Enum(decl) => enum_ty_defn(db, decl).into(),
            TypeDecl::RegularStruct(decl) => regular_struct_ty_defn(db, decl).into(),
            TypeDecl::TupleStruct(decl) => tuple_struct_ty_defn(db, decl).into(),
            TypeDecl::UnitStruct(decl) => unit_struct_ty_defn(db, decl).into(),
            TypeDecl::Record(decl) => record_ty_defn(db, decl).into(),
            TypeDecl::Inductive(decl) => inductive_ty_defn(db, decl).into(),
            TypeDecl::Structure(decl) => structure_ty_defn(db, decl).into(),
            TypeDecl::Extern(decl) => alien_ty_defn(db, decl).into(),
            TypeDecl::Union(decl) => union_ty_defn(db, decl).into(),
        }
    }
}
