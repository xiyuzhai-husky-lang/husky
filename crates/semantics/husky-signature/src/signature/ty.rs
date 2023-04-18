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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TypeDeclarativeSignature {
    Enum(EnumTypeDeclarativeSignature),
    RegularStruct(RegularStructTypeDeclarativeSignature),
    UnitStruct(UnitStructTypeDeclarativeSignature),
    TupleStruct(TupleStructTypeDeclarativeSignature),
    Record(RecordTypeDeclarativeSignature),
    Inductive(InductiveTypeDeclarativeSignature),
    Structure(StructureTypeDeclarativeSignature),
    Foreign(ExternTypeDeclarativeSignature),
    Union(UnionTypeDeclarativeSignature),
}

impl TypeDeclarativeSignature {
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        match self {
            TypeDeclarativeSignature::Enum(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignature::UnitStruct(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignature::TupleStruct(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignature::RegularStruct(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignature::Record(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignature::Inductive(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignature::Structure(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignature::Foreign(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignature::Union(decl) => decl.implicit_parameters(db),
        }
    }
}

impl HasDeclarativeSignature for TypePath {
    type DeclarativeSignature = TypeDeclarativeSignature;

    #[inline(always)]
    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignature> {
        ty_declarative_signature_from_decl(db, self.decl(db)?)
    }
}

pub(crate) fn ty_declarative_signature_from_decl(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeDecl,
) -> DeclarativeSignatureResult<TypeDeclarativeSignature> {
    match decl {
        TypeDecl::Enum(decl) => enum_ty_declarative_signature(db, decl).map(Into::into),
        TypeDecl::RegularStruct(decl) => {
            regular_struct_ty_declarative_signature(db, decl).map(Into::into)
        }
        TypeDecl::UnitStruct(decl) => {
            unit_struct_ty_declarative_signature(db, decl).map(Into::into)
        }
        TypeDecl::TupleStruct(decl) => {
            tuple_struct_ty_declarative_signature(db, decl).map(Into::into)
        }
        TypeDecl::Record(decl) => record_ty_declarative_signature(db, decl).map(Into::into),
        TypeDecl::Inductive(decl) => inductive_ty_declarative_signature(db, decl).map(Into::into),
        TypeDecl::Structure(decl) => structure_ty_declarative_signature(db, decl).map(Into::into),
        TypeDecl::Extern(decl) => alien_ty_declarative_signature(db, decl).map(Into::into),
        TypeDecl::Union(decl) => union_ty_declarative_signature(db, decl).map(Into::into),
    }
}
