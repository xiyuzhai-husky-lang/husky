mod alien_ty;
mod enum_ty;
mod inductive_ty;
mod props_struct_ty;
mod record_ty;
mod structure_ty;
mod tuple_struct_ty;
mod union_ty;
mod unit_struct_ty;

pub use alien_ty::*;
pub use enum_ty::*;
pub use inductive_ty::*;
pub use props_struct_ty::*;
pub use record_ty::*;
pub use structure_ty::*;
pub use tuple_struct_ty::*;
pub use union_ty::*;
pub use unit_struct_ty::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DefnDb)]
pub enum TypeDefn {
    Enum(EnumTypeDefn),
    Inductive(InductiveTypeDefn),
    Record(RecordTypeDefn),
    RegularStruct(RegularStructTypeDefn),
    TupleStruct(TupleStructTypeDefn),
    UnitStruct(UnitStructTypeDefn),
    Structure(StructureTypeDefn),
    Alien(AlienTypeDefn),
    Union(UnionTypeDefn),
}

impl From<AlienTypeDefn> for TypeDefn {
    fn from(v: AlienTypeDefn) -> Self {
        Self::Alien(v)
    }
}

impl From<UnionTypeDefn> for TypeDefn {
    fn from(v: UnionTypeDefn) -> Self {
        Self::Union(v)
    }
}

impl From<UnitStructTypeDefn> for TypeDefn {
    fn from(v: UnitStructTypeDefn) -> Self {
        Self::UnitStruct(v)
    }
}

impl From<TupleStructTypeDefn> for TypeDefn {
    fn from(v: TupleStructTypeDefn) -> Self {
        Self::TupleStruct(v)
    }
}

impl From<RegularStructTypeDefn> for TypeDefn {
    fn from(v: RegularStructTypeDefn) -> Self {
        Self::RegularStruct(v)
    }
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
            TypeDefn::Alien(defn) => defn.decl(db).into(),
            TypeDefn::Union(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn DefnDb) -> TypePath {
        match self {
            TypeDefn::Enum(defn) => defn.path(db),
            TypeDefn::Inductive(defn) => defn.path(db),
            TypeDefn::Record(defn) => defn.path(db),
            TypeDefn::UnitStruct(defn) => defn.path(db),
            TypeDefn::TupleStruct(defn) => defn.path(db),
            TypeDefn::RegularStruct(defn) => defn.path(db),
            TypeDefn::Structure(defn) => defn.path(db),
            TypeDefn::Alien(defn) => defn.path(db),
            TypeDefn::Union(defn) => defn.path(db),
        }
    }
}

impl From<EnumTypeDefn> for TypeDefn {
    fn from(v: EnumTypeDefn) -> Self {
        Self::Enum(v)
    }
}

impl From<InductiveTypeDefn> for TypeDefn {
    fn from(v: InductiveTypeDefn) -> Self {
        Self::Inductive(v)
    }
}

impl From<RecordTypeDefn> for TypeDefn {
    fn from(v: RecordTypeDefn) -> Self {
        Self::Record(v)
    }
}

impl From<StructureTypeDefn> for TypeDefn {
    fn from(v: StructureTypeDefn) -> Self {
        Self::Structure(v)
    }
}
