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
pub enum TypeDefn {
    Enum(EnumTypeDefn),
    Inductive(InductiveTypeDefn),
    Record(RecordTypeDefn),
    PropsStruct(PropsStructTypeDefn),
    TupleStruct(TupleStructTypeDefn),
    UnitStruct(UnitStructTypeDefn),
    Structure(StructureTypeDefn),
    Foreign(AlienTypeDefn),
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

impl From<PropsStructTypeDefn> for TypeDefn {
    fn from(v: PropsStructTypeDefn) -> Self {
        Self::PropsStruct(v)
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
            TypeDefn::PropsStruct(defn) => defn.decl(db).into(),
            TypeDefn::Structure(defn) => defn.decl(db).into(),
            TypeDefn::Foreign(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn DefnDb) -> TypePath {
        match self {
            TypeDefn::Enum(defn) => defn.path(db),
            TypeDefn::Inductive(defn) => defn.path(db),
            TypeDefn::Record(defn) => defn.path(db),
            TypeDefn::UnitStruct(defn) => defn.path(db),
            TypeDefn::TupleStruct(defn) => defn.path(db),
            TypeDefn::PropsStruct(defn) => defn.path(db),
            TypeDefn::Structure(defn) => defn.path(db),
            TypeDefn::Foreign(defn) => defn.path(db),
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

impl From<AlienTypeDefn> for TypeDefn {
    fn from(v: AlienTypeDefn) -> Self {
        Self::Foreign(v)
    }
}

impl<Db: DefnDb + ?Sized> salsa::DebugWithDb<Db> for TypeDefn {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<DefnJar>>::as_jar_db(db);
        match self {
            TypeDefn::Enum(decl) => f
                .debug_tuple("Enum")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDefn::Inductive(decl) => f
                .debug_tuple("Inductive")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDefn::Record(decl) => f
                .debug_tuple("Record")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDefn::PropsStruct(decl) => f
                .debug_tuple("PropsStruct")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDefn::TupleStruct(decl) => f
                .debug_tuple("TupleStruct")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDefn::UnitStruct(decl) => f
                .debug_tuple("UnitStruct")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDefn::Structure(decl) => f
                .debug_tuple("Structure")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDefn::Foreign(decl) => f
                .debug_tuple("Foreign")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
