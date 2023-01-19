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

pub(crate) fn ty_signature(db: &dyn SignatureDb, decl: TypeDecl) -> TypeSignature {
    match decl {
        TypeDecl::Enum(decl) => enum_ty_signature(db, decl).into(),
        TypeDecl::RegularStruct(decl) => regular_struct_ty_signature(db, decl).into(),
        TypeDecl::UnitStruct(decl) => unit_struct_ty_signature(db, decl).into(),
        TypeDecl::TupleStruct(decl) => tuple_struct_ty_signature(db, decl).into(),
        TypeDecl::Record(decl) => record_ty_signature(db, decl).into(),
        TypeDecl::Inductive(decl) => inductive_ty_signature(db, decl).into(),
        TypeDecl::Structure(decl) => structure_ty_signature(db, decl).into(),
        TypeDecl::Foreign(decl) => alien_ty_signature(db, decl).into(),
        TypeDecl::Union(decl) => union_ty_signature(db, decl).into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeSignature {
    Enum(EnumTypeSignature),
    RegularStruct(RegularStructTypeSignature),
    UnitStruct(UnitStructTypeSignature),
    TupleStruct(TupleStructTypeSignature),
    Record(RecordTypeSignature),
    Inductive(InductiveTypeSignature),
    Structure(StructureTypeSignature),
    Foreign(AlienTypeSignature),
    Union(UnionTypeSignature),
}

impl TypeSignature {
    pub fn term_sheet<'a>(self, db: &'a dyn SignatureDb) -> &'a SignatureTermSheet {
        match self {
            TypeSignature::Enum(signature) => signature.term_sheet(db),
            TypeSignature::RegularStruct(signature) => signature.term_sheet(db),
            TypeSignature::UnitStruct(signature) => signature.term_sheet(db),
            TypeSignature::TupleStruct(signature) => signature.term_sheet(db),
            TypeSignature::Record(signature) => signature.term_sheet(db),
            TypeSignature::Inductive(signature) => signature.term_sheet(db),
            TypeSignature::Structure(signature) => signature.term_sheet(db),
            TypeSignature::Foreign(signature) => signature.term_sheet(db),
            TypeSignature::Union(signature) => signature.term_sheet(db),
        }
    }

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

impl From<UnionTypeSignature> for TypeSignature {
    fn from(v: UnionTypeSignature) -> Self {
        Self::Union(v)
    }
}

impl From<EnumTypeSignature> for TypeSignature {
    fn from(v: EnumTypeSignature) -> Self {
        Self::Enum(v)
    }
}

impl From<TupleStructTypeSignature> for TypeSignature {
    fn from(v: TupleStructTypeSignature) -> Self {
        Self::TupleStruct(v)
    }
}

impl From<UnitStructTypeSignature> for TypeSignature {
    fn from(v: UnitStructTypeSignature) -> Self {
        Self::UnitStruct(v)
    }
}

impl From<RegularStructTypeSignature> for TypeSignature {
    fn from(v: RegularStructTypeSignature) -> Self {
        Self::RegularStruct(v)
    }
}

impl From<RecordTypeSignature> for TypeSignature {
    fn from(v: RecordTypeSignature) -> Self {
        Self::Record(v)
    }
}

impl From<InductiveTypeSignature> for TypeSignature {
    fn from(v: InductiveTypeSignature) -> Self {
        Self::Inductive(v)
    }
}

impl From<StructureTypeSignature> for TypeSignature {
    fn from(v: StructureTypeSignature) -> Self {
        Self::Structure(v)
    }
}

impl From<AlienTypeSignature> for TypeSignature {
    fn from(v: AlienTypeSignature) -> Self {
        Self::Foreign(v)
    }
}

impl<Db: SignatureDb + ?Sized> salsa::DebugWithDb<Db> for TypeSignature {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<SignatureJar>>::as_jar_db(db);
        match self {
            TypeSignature::Enum(decl) => f
                .debug_tuple("Enum")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeSignature::Inductive(decl) => f
                .debug_tuple("Inductive")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeSignature::Record(decl) => f
                .debug_tuple("Record")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeSignature::RegularStruct(decl) => f
                .debug_tuple("RegularStruct")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeSignature::TupleStruct(decl) => f
                .debug_tuple("TupleStruct")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeSignature::UnitStruct(decl) => f
                .debug_tuple("UnitStruct")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeSignature::Structure(decl) => f
                .debug_tuple("Structure")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeSignature::Foreign(decl) => f
                .debug_tuple("Foreign")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeSignature::Union(_) => todo!(),
        }
    }
}
