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

pub(crate) fn ty_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TypeDecl,
) -> RawSignatureResultRef<TypeRawSignature> {
    match decl {
        TypeDecl::Enum(decl) => enum_ty_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeDecl::RegularStruct(decl) => regular_struct_ty_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeDecl::UnitStruct(decl) => unit_struct_ty_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeDecl::TupleStruct(decl) => tuple_struct_ty_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeDecl::Record(decl) => record_ty_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeDecl::Inductive(decl) => inductive_ty_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeDecl::Structure(decl) => structure_ty_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeDecl::Extern(decl) => alien_ty_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        TypeDecl::Union(decl) => union_ty_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = RawSignatureDb)]
pub enum TypeRawSignature {
    Enum(EnumTypeRawSignature),
    RegularStruct(RegularStructTypeRawSignature),
    UnitStruct(UnitStructTypeRawSignature),
    TupleStruct(TupleStructTypeRawSignature),
    Record(RecordTypeRawSignature),
    Inductive(InductiveTypeRawSignature),
    Structure(StructureTypeRawSignature),
    Foreign(ExternTypeRawSignature),
    Union(UnionTypeRawSignature),
}

impl TypeRawSignature {
    pub fn implicit_parameters(self, db: &dyn RawSignatureDb) -> &[ImplicitParameterRawSignature] {
        match self {
            TypeRawSignature::Enum(decl) => decl.implicit_parameters(db),
            TypeRawSignature::UnitStruct(decl) => decl.implicit_parameters(db),
            TypeRawSignature::TupleStruct(decl) => decl.implicit_parameters(db),
            TypeRawSignature::RegularStruct(decl) => decl.implicit_parameters(db),
            TypeRawSignature::Record(decl) => decl.implicit_parameters(db),
            TypeRawSignature::Inductive(decl) => decl.implicit_parameters(db),
            TypeRawSignature::Structure(decl) => decl.implicit_parameters(db),
            TypeRawSignature::Foreign(decl) => decl.implicit_parameters(db),
            TypeRawSignature::Union(decl) => decl.implicit_parameters(db),
        }
    }
}

impl From<UnionTypeRawSignature> for TypeRawSignature {
    fn from(v: UnionTypeRawSignature) -> Self {
        Self::Union(v)
    }
}

impl From<EnumTypeRawSignature> for TypeRawSignature {
    fn from(v: EnumTypeRawSignature) -> Self {
        Self::Enum(v)
    }
}

impl From<TupleStructTypeRawSignature> for TypeRawSignature {
    fn from(v: TupleStructTypeRawSignature) -> Self {
        Self::TupleStruct(v)
    }
}

impl From<UnitStructTypeRawSignature> for TypeRawSignature {
    fn from(v: UnitStructTypeRawSignature) -> Self {
        Self::UnitStruct(v)
    }
}

impl From<RegularStructTypeRawSignature> for TypeRawSignature {
    fn from(v: RegularStructTypeRawSignature) -> Self {
        Self::RegularStruct(v)
    }
}

impl From<RecordTypeRawSignature> for TypeRawSignature {
    fn from(v: RecordTypeRawSignature) -> Self {
        Self::Record(v)
    }
}

impl From<InductiveTypeRawSignature> for TypeRawSignature {
    fn from(v: InductiveTypeRawSignature) -> Self {
        Self::Inductive(v)
    }
}

impl From<StructureTypeRawSignature> for TypeRawSignature {
    fn from(v: StructureTypeRawSignature) -> Self {
        Self::Structure(v)
    }
}

impl From<ExternTypeRawSignature> for TypeRawSignature {
    fn from(v: ExternTypeRawSignature) -> Self {
        Self::Foreign(v)
    }
}
