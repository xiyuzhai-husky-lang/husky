mod alien_ty;
mod enum_ty;
mod inductive_ty;
mod record_ty;
mod regular_struct_ty;
mod structure_ty;
mod tuple_struct_ty;
mod union_ty;
mod unit_struct_ty;

pub use alien_ty::*;
pub use enum_ty::*;
pub use inductive_ty::*;
pub use record_ty::*;
pub use regular_struct_ty::*;
pub use structure_ty::*;
pub use tuple_struct_ty::*;
pub use union_ty::*;
pub use unit_struct_ty::*;

use super::*;
use salsa::DbWithJar;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
pub enum TypeDecl {
    Enum(EnumTypeDecl),
    RegularStruct(RegularStructTypeDecl),
    UnitStruct(UnitStructTypeDecl),
    TupleStruct(TupleStructTypeDecl),
    Record(RecordTypeDecl),
    Inductive(InductiveTypeDecl),
    Structure(StructureTypeDecl),
    Alien(AlienTypeDecl),
    Union(UnionTypeDecl),
}

impl From<UnionTypeDecl> for TypeDecl {
    fn from(v: UnionTypeDecl) -> Self {
        Self::Union(v)
    }
}

impl TypeDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TypeDecl::Enum(decl) => decl.ast_idx(db),
            TypeDecl::UnitStruct(decl) => decl.ast_idx(db),
            TypeDecl::TupleStruct(decl) => decl.ast_idx(db),
            TypeDecl::RegularStruct(decl) => decl.ast_idx(db),
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
            TypeDecl::RegularStruct(decl) => decl.implicit_parameters(db),
            TypeDecl::Record(decl) => decl.implicit_parameters(db),
            TypeDecl::Inductive(decl) => decl.implicit_parameters(db),
            TypeDecl::Structure(decl) => decl.implicit_parameters(db),
            TypeDecl::Alien(decl) => decl.implicit_parameters(db),
            TypeDecl::Union(decl) => decl.implicit_parameters(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            TypeDecl::Enum(decl) => decl.expr_region(db),
            TypeDecl::UnitStruct(decl) => decl.expr_region(db),
            TypeDecl::TupleStruct(decl) => decl.expr_region(db),
            TypeDecl::RegularStruct(decl) => decl.expr_region(db),
            TypeDecl::Record(decl) => decl.expr_region(db),
            TypeDecl::Inductive(decl) => decl.expr_region(db),
            TypeDecl::Structure(decl) => decl.expr_region(db),
            TypeDecl::Alien(decl) => decl.expr_region(db),
            TypeDecl::Union(decl) => decl.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> TypePath {
        match self {
            TypeDecl::Enum(decl) => decl.path(db),
            TypeDecl::Inductive(decl) => decl.path(db),
            TypeDecl::Record(decl) => decl.path(db),
            TypeDecl::UnitStruct(decl) => decl.path(db),
            TypeDecl::RegularStruct(decl) => decl.path(db),
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

impl From<RegularStructTypeDecl> for TypeDecl {
    fn from(v: RegularStructTypeDecl) -> Self {
        Self::RegularStruct(v)
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
