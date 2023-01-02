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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeDecl {
    Enum(EnumTypeDecl),
    PropsStruct(PropsStructTypeDecl),
    UnitStruct(UnitStructTypeDecl),
    TupleStruct(TupleStructTypeDecl),
    Record(RecordTypeDecl),
    Inductive(InductiveTypeDecl),
    Structure(StructureTypeDecl),
    Alien(AlienTypeDecl),
    Union(UnionTypeDecl),
}

impl TypeDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TypeDecl::Enum(decl) => decl.ast_idx(db),
            TypeDecl::UnitStruct(decl) => decl.ast_idx(db),
            TypeDecl::TupleStruct(decl) => decl.ast_idx(db),
            TypeDecl::PropsStruct(decl) => decl.ast_idx(db),
            TypeDecl::Record(decl) => decl.ast_idx(db),
            TypeDecl::Inductive(decl) => decl.ast_idx(db),
            TypeDecl::Structure(decl) => decl.ast_idx(db),
            TypeDecl::Alien(decl) => decl.ast_idx(db),
            TypeDecl::Union(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        match self {
            TypeDecl::Enum(decl) => decl.implicit_parameters(db),
            TypeDecl::UnitStruct(decl) => decl.implicit_parameters(db),
            TypeDecl::TupleStruct(decl) => decl.implicit_parameters(db),
            TypeDecl::PropsStruct(decl) => decl.implicit_parameters(db),
            TypeDecl::Record(decl) => decl.implicit_parameters(db),
            TypeDecl::Inductive(decl) => decl.implicit_parameters(db),
            TypeDecl::Structure(decl) => decl.implicit_parameters(db),
            TypeDecl::Alien(decl) => decl.implicit_parameters(db),
            TypeDecl::Union(decl) => decl.implicit_parameters(db),
        }
    }
}

impl From<EnumTypeDecl> for TypeDecl {
    fn from(v: EnumTypeDecl) -> Self {
        Self::Enum(v)
    }
}

impl From<TupleStructTypeDecl> for TypeDecl {
    fn from(v: TupleStructTypeDecl) -> Self {
        Self::TupleStruct(v)
    }
}

impl From<UnitStructTypeDecl> for TypeDecl {
    fn from(v: UnitStructTypeDecl) -> Self {
        Self::UnitStruct(v)
    }
}

impl From<PropsStructTypeDecl> for TypeDecl {
    fn from(v: PropsStructTypeDecl) -> Self {
        Self::PropsStruct(v)
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
            TypeDecl::UnitStruct(decl) => decl.path(db),
            TypeDecl::PropsStruct(decl) => decl.path(db),
            TypeDecl::TupleStruct(decl) => decl.path(db),
            TypeDecl::Structure(decl) => decl.path(db),
            TypeDecl::Alien(decl) => decl.path(db),
            TypeDecl::Union(decl) => decl.path(db),
        }
    }

    pub fn entity_path(self, db: &dyn DeclDb) -> EntityPath {
        self.path(db).into()
    }
}
