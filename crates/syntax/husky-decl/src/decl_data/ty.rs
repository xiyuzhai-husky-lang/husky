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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeDecl {
    Enum(EnumTypeDecl),
    Struct(StructTypeDecl),
    Record(RecordTypeDecl),
    Inductive(InductiveTypeDecl),
    Structure(StructureTypeDecl),
}

impl TypeDecl {
    fn module_item_path(self, db: &dyn DeclDb) -> ModuleItemPath {
        match self {
            TypeDecl::Enum(decl) => decl.module_item_path(db),
            TypeDecl::Inductive(decl) => decl.module_item_path(db),
            TypeDecl::Record(decl) => decl.module_item_path(db),
            TypeDecl::Struct(decl) => decl.module_item_path(db),
            TypeDecl::Structure(decl) => decl.module_item_path(db),
        }
    }

    pub fn entity_path(self, db: &dyn DeclDb) -> EntityPath {
        self.module_item_path(db).into()
    }
}
