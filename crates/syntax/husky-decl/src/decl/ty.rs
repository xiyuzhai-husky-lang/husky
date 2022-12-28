mod alias_ty;
mod enum_ty;
mod inductive_ty;
mod record_ty;
mod struct_ty;
mod structure_ty;

pub use alias_ty::*;
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
    Alias(AliasTypeDecl),
}

impl From<EnumTypeDecl> for TypeDecl {
    fn from(v: EnumTypeDecl) -> Self {
        Self::Enum(v)
    }
}

impl From<StructTypeDecl> for TypeDecl {
    fn from(v: StructTypeDecl) -> Self {
        Self::Struct(v)
    }
}

impl From<RecordTypeDecl> for TypeDecl {
    fn from(v: RecordTypeDecl) -> Self {
        Self::Record(v)
    }
}

impl From<InductiveTypeDecl> for TypeDecl {
    fn from(v: InductiveTypeDecl) -> Self {
        Self::Inductive(v)
    }
}

impl From<StructureTypeDecl> for TypeDecl {
    fn from(v: StructureTypeDecl) -> Self {
        Self::Structure(v)
    }
}

impl From<AliasTypeDecl> for TypeDecl {
    fn from(v: AliasTypeDecl) -> Self {
        Self::Alias(v)
    }
}

impl TypeDecl {
    fn module_item_path(self, db: &dyn DeclDb) -> ModuleItemPath {
        match self {
            TypeDecl::Enum(decl) => decl.module_item_path(db),
            TypeDecl::Inductive(decl) => decl.module_item_path(db),
            TypeDecl::Record(decl) => decl.module_item_path(db),
            TypeDecl::Struct(decl) => decl.module_item_path(db),
            TypeDecl::Structure(decl) => decl.module_item_path(db),
            TypeDecl::Alias(decl) => decl.module_item_path(db),
        }
    }

    pub fn entity_path(self, db: &dyn DeclDb) -> EntityPath {
        self.module_item_path(db).into()
    }
}
