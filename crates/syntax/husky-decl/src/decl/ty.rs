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
pub enum TypeDecl {
    Enum(EnumTypeDecl),
    PropsStruct(PropsStructTypeDecl),
    UnitStruct(UnitStructTypeDecl),
    TupleStruct(TupleStructTypeDecl),
    Record(RecordTypeDecl),
    Inductive(InductiveTypeDecl),
    Structure(StructureTypeDecl),
    Foreign(AlienTypeDecl),
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
            TypeDecl::Foreign(decl) => decl.ast_idx(db),
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
            TypeDecl::Foreign(decl) => decl.implicit_parameters(db),
            TypeDecl::Union(decl) => decl.implicit_parameters(db),
        }
    }

    pub fn expr_sheet(self, db: &dyn DeclDb) -> ExprSheet {
        match self {
            TypeDecl::Enum(decl) => decl.expr_sheet(db),
            TypeDecl::UnitStruct(decl) => decl.expr_sheet(db),
            TypeDecl::TupleStruct(decl) => decl.expr_sheet(db),
            TypeDecl::PropsStruct(decl) => decl.expr_sheet(db),
            TypeDecl::Record(decl) => decl.expr_sheet(db),
            TypeDecl::Inductive(decl) => decl.expr_sheet(db),
            TypeDecl::Structure(decl) => decl.expr_sheet(db),
            TypeDecl::Foreign(decl) => decl.expr_sheet(db),
            TypeDecl::Union(decl) => decl.expr_sheet(db),
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
        Self::Foreign(v)
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
            TypeDecl::Foreign(decl) => decl.path(db),
            TypeDecl::Union(decl) => decl.path(db),
        }
    }

    pub fn entity_path(self, db: &dyn DeclDb) -> EntityPath {
        self.path(db).into()
    }
}

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for TypeDecl {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<DeclJar>>::as_jar_db(db);
        match self {
            TypeDecl::Enum(decl) => f
                .debug_tuple("Enum")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDecl::Inductive(decl) => f
                .debug_tuple("Inductive")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDecl::Record(decl) => f
                .debug_tuple("Record")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDecl::PropsStruct(decl) => f
                .debug_tuple("PropsStruct")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDecl::TupleStruct(decl) => f
                .debug_tuple("TupleStruct")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDecl::UnitStruct(decl) => f
                .debug_tuple("UnitStruct")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDecl::Structure(decl) => f
                .debug_tuple("Structure")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDecl::Foreign(decl) => f
                .debug_tuple("Foreign")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeDecl::Union(_) => todo!(),
        }
    }
}
