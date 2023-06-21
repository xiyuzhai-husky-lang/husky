mod r#enum;
mod r#extern;
mod inductive;
mod record;
mod regular_struct;
mod structure;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::inductive::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::record::*;
pub use self::regular_struct::*;
pub use self::structure::*;
pub use self::tuple_struct::*;
pub use self::union::*;
pub use self::unit_struct::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub enum TypeDeclarativeSignature {
    Enum(EnumTypeDeclarativeSignature),
    RegularStruct(RegularStructDeclarativeSignature),
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
    Enum(EnumDeclarativeSignatureTemplate),
    RegularStruct(RegularStructDeclarativeSignatureTemplate),
    UnitStruct(UnitStructDeclarativeSignatureTemplate),
    TupleStruct(TupleStructDeclarativeSignatureTemplate),
    Record(RecordDeclarativeSignatureTemplate),
    Inductive(InductiveDeclarativeSignatureTemplate),
    Structure(StructureDeclarativeSignatureTemplate),
    Extern(ExternDeclarativeSignatureTemplate),
    Union(UnionDeclarativeSignatureTemplate),
}

impl TypeDeclarativeSignatureTemplate {
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterDeclarativeSignature] {
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
        ty_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    path: TypePath,
) -> DeclarativeSignatureResult<TypeDeclarativeSignatureTemplate> {
    let decl = path.decl(db)?;
    match decl {
        TypeDecl::Enum(decl) => enum_declarative_signature_template(db, decl).map(Into::into),
        TypeDecl::RegularStruct(decl) => {
            regular_struct_declarative_signature_template(db, decl).map(Into::into)
        }
        TypeDecl::UnitStruct(decl) => {
            unit_struct_declarative_signature_template(db, decl).map(Into::into)
        }
        TypeDecl::TupleStruct(decl) => {
            tuple_struct_declarative_signature_template(db, decl).map(Into::into)
        }
        TypeDecl::Record(decl) => record_declarative_signature_template(db, decl).map(Into::into),
        TypeDecl::Inductive(decl) => {
            inductive_declarative_signature_template(db, decl).map(Into::into)
        }
        TypeDecl::Structure(decl) => {
            structure_declarative_signature_template(db, decl).map(Into::into)
        }
        TypeDecl::Extern(decl) => extern_declarative_signature_template(db, decl).map(Into::into),
        TypeDecl::Union(decl) => union_declarative_signature_template(db, decl).map(Into::into),
    }
}
