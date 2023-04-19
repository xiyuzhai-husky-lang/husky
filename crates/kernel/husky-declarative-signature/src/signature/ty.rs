mod enum_ty;
mod extern_ty;
mod inductive_ty;
mod record_ty;
mod regular_struct_ty;
mod structure_ty;
mod tuple_struct_ty;
mod union_ty;
mod unit_struct_ty;

pub use self::enum_ty::*;
pub use self::extern_ty::*;
pub use self::inductive_ty::*;
pub use self::record_ty::*;
pub use self::regular_struct_ty::*;
pub use self::structure_ty::*;
pub use self::tuple_struct_ty::*;
pub use self::union_ty::*;
pub use self::unit_struct_ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub enum TypeDeclarativeSignature {
    Enum(EnumTypeDeclarativeSignature),
    RegularStruct(RegularStructTypeDeclarativeSignature),
    UnitStruct(UnitStructTypeDeclarativeSignature),
    TupleStruct(TupleStructTypeDeclarativeSignature),
    Record(RecordTypeDeclarativeSignature),
    Inductive(InductiveTypeDeclarativeSignature),
    Structure(StructureTypeDeclarativeSignature),
    Extern(ExternTypeDeclarativeSignature),
    Union(UnionTypeDeclarativeSignature),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TypeDeclarativeSignatureTemplate {
    Enum(EnumTypeDeclarativeSignatureTemplate),
    RegularStruct(RegularStructTypeDeclarativeSignatureTemplate),
    UnitStruct(UnitStructTypeDeclarativeSignatureTemplate),
    TupleStruct(TupleStructTypeDeclarativeSignatureTemplate),
    Record(RecordTypeDeclarativeSignatureTemplate),
    Inductive(InductiveTypeDeclarativeSignatureTemplate),
    Structure(StructureTypeDeclarativeSignatureTemplate),
    Extern(ExternTypeDeclarativeSignatureTemplate),
    Union(UnionTypeDeclarativeSignatureTemplate),
}

impl TypeDeclarativeSignatureTemplate {
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        match self {
            TypeDeclarativeSignatureTemplate::Enum(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::UnitStruct(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::TupleStruct(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::RegularStruct(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::Record(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::Inductive(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::Structure(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::Extern(decl) => decl.implicit_parameters(db),
            TypeDeclarativeSignatureTemplate::Union(decl) => decl.implicit_parameters(db),
        }
    }
}

impl HasDeclarativeSignatureTemplate for TypePath {
    type DeclarativeSignatureTemplate = TypeDeclarativeSignatureTemplate;

    #[inline(always)]
    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        ty_declarative_signature_template_from_decl(db, self.decl(db)?)
    }
}

pub(crate) fn ty_declarative_signature_template_from_decl(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeDecl,
) -> DeclarativeSignatureResult<TypeDeclarativeSignatureTemplate> {
    match decl {
        TypeDecl::Enum(decl) => enum_ty_declarative_signature_template(db, decl).map(Into::into),
        TypeDecl::RegularStruct(decl) => {
            regular_struct_ty_declarative_signature_template(db, decl).map(Into::into)
        }
        TypeDecl::UnitStruct(decl) => {
            unit_struct_ty_declarative_signature_template(db, decl).map(Into::into)
        }
        TypeDecl::TupleStruct(decl) => {
            tuple_struct_ty_declarative_signature_template(db, decl).map(Into::into)
        }
        TypeDecl::Record(decl) => {
            record_ty_declarative_signature_template(db, decl).map(Into::into)
        }
        TypeDecl::Inductive(decl) => {
            inductive_ty_declarative_signature_template(db, decl).map(Into::into)
        }
        TypeDecl::Structure(decl) => {
            structure_ty_declarative_signature_template(db, decl).map(Into::into)
        }
        TypeDecl::Extern(decl) => alien_ty_declarative_signature_template(db, decl).map(Into::into),
        TypeDecl::Union(decl) => union_ty_declarative_signature_template(db, decl).map(Into::into),
    }
}
