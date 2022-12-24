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
}

impl TypeDefn {
    pub fn module_item_path(self, db: &dyn DefnDb) -> ModuleItemPath {
        match self {
            TypeDefn::Enum(defn) => defn.module_item_path(db),
            TypeDefn::Inductive(defn) => defn.module_item_path(db),
            TypeDefn::Record(defn) => defn.module_item_path(db),
            TypeDefn::Struct(defn) => defn.module_item_path(db),
            TypeDefn::Structure(defn) => defn.module_item_path(db),
        }
    }
}
