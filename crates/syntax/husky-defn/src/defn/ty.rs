mod enum_ty;
mod inductive_ty;
mod record_ty;
mod struct_ty;
mod structure_ty;

pub use enum_ty::*;
pub use inductive_ty::*;
pub use record_ty::*;
pub use struct_ty::*;
pub use structure_ty::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeDefn {
    Enum(EnumTypeDefn),
    Inductive(InductiveTypeDefn),
    Record(RecordTypeDefn),
    Struct(StructTypeDefn),
    Structure(StructureTypeDefn),
    Alias(AliasTypeDefn),
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

impl From<AliasTypeDefn> for TypeDefn {
    fn from(v: AliasTypeDefn) -> Self {
        Self::Alias(v)
    }
}

impl TypeDefn {
    pub fn module_item_path(self, db: &dyn DefnDb) -> ModuleItemPath {
        match self {
            TypeDefn::Enum(defn) => defn.module_item_path(db),
            TypeDefn::Inductive(defn) => defn.module_item_path(db),
            TypeDefn::Record(defn) => defn.module_item_path(db),
            TypeDefn::Struct(defn) => defn.module_item_path(db),
            TypeDefn::Structure(defn) => defn.module_item_path(db),
            TypeDefn::Alias(defn) => defn.module_item_path(db),
        }
    }
}
