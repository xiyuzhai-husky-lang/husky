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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeDecl {
    Enum(EnumTypeDecl),
    Struct(StructTypeDecl),
    Record(RecordTypeDecl),
    Inductive(InductiveTypeDecl),
    Structure(StructureTypeDecl),
    Alien(AlienTypeDecl),
    Union(UnionTypeDecl),
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

impl From<AlienTypeDecl> for TypeDecl {
    fn from(v: AlienTypeDecl) -> Self {
        Self::Alien(v)
    }
}

impl TypeDecl {
    fn path(self, db: &dyn DeclDb) -> TypePath {
        match self {
            TypeDecl::Enum(decl) => decl.path(db),
            TypeDecl::Inductive(decl) => decl.path(db),
            TypeDecl::Record(decl) => decl.path(db),
            TypeDecl::Struct(decl) => decl.path(db),
            TypeDecl::Structure(decl) => decl.path(db),
            TypeDecl::Alien(decl) => decl.path(db),
            TypeDecl::Union(decl) => decl.path(db),
        }
    }

    pub fn entity_path(self, db: &dyn DeclDb) -> EntityPath {
        self.path(db).into()
    }
}
