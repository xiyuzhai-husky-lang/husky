mod alien_ty;
mod enum_ty;
mod inductive_ty;
mod record_ty;
mod struct_ty;
mod structure_ty;
mod union_ty;

pub use alien_ty::*;
pub use enum_ty::*;
pub use inductive_ty::*;
pub use record_ty::*;
pub use struct_ty::*;
pub use structure_ty::*;
pub use union_ty::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeDefn {
    Enum(EnumTypeDefn),
    Inductive(InductiveTypeDefn),
    Record(RecordTypeDefn),
    Struct(StructTypeDefn),
    Structure(StructureTypeDefn),
    Foreign(AlienTypeDefn),
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

impl From<StructTypeDefn> for TypeDefn {
    fn from(v: StructTypeDefn) -> Self {
        Self::Struct(v)
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

impl TypeDefn {
    pub fn path(self, db: &dyn DefnDb) -> TypePath {
        match self {
            TypeDefn::Enum(defn) => defn.path(db),
            TypeDefn::Inductive(defn) => defn.path(db),
            TypeDefn::Record(defn) => defn.path(db),
            TypeDefn::Struct(defn) => defn.path(db),
            TypeDefn::Structure(defn) => defn.path(db),
            TypeDefn::Foreign(defn) => defn.path(db),
        }
    }
}
