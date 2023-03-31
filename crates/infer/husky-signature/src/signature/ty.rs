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
#[salsa::derive_debug_with_db(db = SignatureDb)]
#[enum_class::from_variants]
pub enum TypeSignature {
    Enum(EnumTypeSignature),
    RegularStruct(RegularStructTypeSignature),
    UnitStruct(UnitStructTypeSignature),
    TupleStruct(TupleStructTypeSignature),
    Record(RecordTypeSignature),
    Inductive(InductiveTypeSignature),
    Structure(StructureTypeSignature),
    Foreign(ExternTypeSignature),
    Union(UnionTypeSignature),
}

impl TypeSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TypeSignature::Enum(decl) => decl.implicit_parameters(db),
            TypeSignature::UnitStruct(decl) => decl.implicit_parameters(db),
            TypeSignature::TupleStruct(decl) => decl.implicit_parameters(db),
            TypeSignature::RegularStruct(decl) => decl.implicit_parameters(db),
            TypeSignature::Record(decl) => decl.implicit_parameters(db),
            TypeSignature::Inductive(decl) => decl.implicit_parameters(db),
            TypeSignature::Structure(decl) => decl.implicit_parameters(db),
            TypeSignature::Foreign(decl) => decl.implicit_parameters(db),
            TypeSignature::Union(decl) => decl.implicit_parameters(db),
        }
    }
}

impl HasSignature for TypePath {
    type Signature = TypeSignature;

    #[inline(always)]
    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature> {
        ty_signature_from_decl(db, self.decl(db)?)
    }
}

pub(crate) fn ty_signature_from_decl(
    db: &dyn SignatureDb,
    decl: TypeDecl,
) -> SignatureResult<TypeSignature> {
    match decl {
        TypeDecl::Enum(decl) => enum_ty_signature(db, decl).map(Into::into),
        TypeDecl::RegularStruct(decl) => regular_struct_ty_signature(db, decl).map(Into::into),
        TypeDecl::UnitStruct(decl) => unit_struct_ty_signature(db, decl).map(Into::into),
        TypeDecl::TupleStruct(decl) => tuple_struct_ty_signature(db, decl).map(Into::into),
        TypeDecl::Record(decl) => record_ty_signature(db, decl).map(Into::into),
        TypeDecl::Inductive(decl) => inductive_ty_signature(db, decl).map(Into::into),
        TypeDecl::Structure(decl) => structure_ty_signature(db, decl).map(Into::into),
        TypeDecl::Extern(decl) => alien_ty_signature(db, decl).map(Into::into),
        TypeDecl::Union(decl) => union_ty_signature(db, decl).map(Into::into),
    }
}
