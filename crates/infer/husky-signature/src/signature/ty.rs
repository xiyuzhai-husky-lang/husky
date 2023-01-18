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
use salsa::DbWithJar;
pub use structure_ty::*;
pub use tuple_struct_ty::*;
pub use union_ty::*;
pub use unit_struct_ty::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeSignature {
    Enum(EnumTypeSignature),
    PropsStruct(PropsStructTypeSignature),
    UnitStruct(UnitStructTypeSignature),
    TupleStruct(TupleStructTypeSignature),
    Record(RecordTypeSignature),
    Inductive(InductiveTypeSignature),
    Structure(StructureTypeSignature),
    Foreign(AlienTypeSignature),
    Union(UnionTypeSignature),
}

impl TypeSignature {
    pub fn ast_idx(self, db: &dyn SignatureDb) -> AstIdx {
        match self {
            TypeSignature::Enum(decl) => decl.ast_idx(db),
            TypeSignature::UnitStruct(decl) => decl.ast_idx(db),
            TypeSignature::TupleStruct(decl) => decl.ast_idx(db),
            TypeSignature::PropsStruct(decl) => decl.ast_idx(db),
            TypeSignature::Record(decl) => decl.ast_idx(db),
            TypeSignature::Inductive(decl) => decl.ast_idx(db),
            TypeSignature::Structure(decl) => decl.ast_idx(db),
            TypeSignature::Foreign(decl) => decl.ast_idx(db),
            TypeSignature::Union(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        match self {
            TypeSignature::Enum(decl) => decl.implicit_parameters(db),
            TypeSignature::UnitStruct(decl) => decl.implicit_parameters(db),
            TypeSignature::TupleStruct(decl) => decl.implicit_parameters(db),
            TypeSignature::PropsStruct(decl) => decl.implicit_parameters(db),
            TypeSignature::Record(decl) => decl.implicit_parameters(db),
            TypeSignature::Inductive(decl) => decl.implicit_parameters(db),
            TypeSignature::Structure(decl) => decl.implicit_parameters(db),
            TypeSignature::Foreign(decl) => decl.implicit_parameters(db),
            TypeSignature::Union(decl) => decl.implicit_parameters(db),
        }
    }

    pub fn expr_page(self, db: &dyn SignatureDb) -> ExprPage {
        match self {
            TypeSignature::Enum(decl) => decl.expr_page(db),
            TypeSignature::UnitStruct(decl) => decl.expr_page(db),
            TypeSignature::TupleStruct(decl) => decl.expr_page(db),
            TypeSignature::PropsStruct(decl) => decl.expr_page(db),
            TypeSignature::Record(decl) => decl.expr_page(db),
            TypeSignature::Inductive(decl) => decl.expr_page(db),
            TypeSignature::Structure(decl) => decl.expr_page(db),
            TypeSignature::Foreign(decl) => decl.expr_page(db),
            TypeSignature::Union(decl) => decl.expr_page(db),
        }
    }

    pub fn path(self, db: &dyn SignatureDb) -> TypePath {
        match self {
            TypeSignature::Enum(decl) => decl.path(db),
            TypeSignature::Inductive(decl) => decl.path(db),
            TypeSignature::Record(decl) => decl.path(db),
            TypeSignature::UnitStruct(decl) => decl.path(db),
            TypeSignature::PropsStruct(decl) => decl.path(db),
            TypeSignature::TupleStruct(decl) => decl.path(db),
            TypeSignature::Structure(decl) => decl.path(db),
            TypeSignature::Foreign(decl) => decl.path(db),
            TypeSignature::Union(decl) => decl.path(db),
        }
    }

    pub fn entity_path(self, db: &dyn SignatureDb) -> EntityPath {
        self.path(db).into()
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

impl From<PropsStructTypeSignature> for TypeSignature {
    fn from(v: PropsStructTypeSignature) -> Self {
        Self::PropsStruct(v)
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
            TypeSignature::PropsStruct(decl) => f
                .debug_tuple("PropsStruct")
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
