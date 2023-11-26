mod r#enum;
mod r#extern;
mod props_struct;
mod record;
mod tuple_struct;
mod r#union;
mod unit_struct;

pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::r#union::*;
pub use self::record::*;
pub use self::tuple_struct::*;
pub use self::unit_struct::*;

use super::*;
use husky_entity_path::TypePath;

use husky_syn_decl::TypeSynDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum TypeHirDecl {
    Enum(EnumTypeHirDecl),
    PropsStruct(PropsStructTypeHirDecl),
    UnitStruct(UnitStructHirDecl),
    TupleStruct(TupleStructTypeHirDecl),
    Record(RecordTypeHirDecl),
    Extern(ExternTypeHirDecl),
    Union(UnionHirDecl),
}

impl TypeHirDecl {
    pub fn path(self, db: &dyn HirDeclDb) -> TypePath {
        match self {
            TypeHirDecl::Enum(decl) => decl.path(db),
            TypeHirDecl::Record(decl) => decl.path(db),
            TypeHirDecl::UnitStruct(decl) => decl.path(db),
            TypeHirDecl::PropsStruct(decl) => decl.path(db),
            TypeHirDecl::TupleStruct(decl) => decl.path(db),
            TypeHirDecl::Extern(decl) => decl.path(db),
            TypeHirDecl::Union(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a dyn HirDeclDb) -> &'a [HirTemplateParameter] {
        match self {
            TypeHirDecl::Enum(decl) => decl.template_parameters(db),
            TypeHirDecl::UnitStruct(decl) => decl.template_parameters(db),
            TypeHirDecl::TupleStruct(decl) => decl.template_parameters(db),
            TypeHirDecl::PropsStruct(decl) => decl.template_parameters(db),
            TypeHirDecl::Record(decl) => decl.template_parameters(db),
            TypeHirDecl::Extern(decl) => decl.template_parameters(db),
            TypeHirDecl::Union(decl) => decl.template_parameters(db),
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> HirExprRegion {
        match self {
            TypeHirDecl::Enum(decl) => decl.hir_eager_expr_region(db).into(),
            TypeHirDecl::UnitStruct(decl) => decl.hir_eager_expr_region(db).into(),
            TypeHirDecl::TupleStruct(decl) => decl.hir_eager_expr_region(db).into(),
            TypeHirDecl::PropsStruct(decl) => decl.hir_eager_expr_region(db).into(),
            TypeHirDecl::Record(decl) => decl.hir_eager_expr_region(db).into(),
            TypeHirDecl::Extern(decl) => decl.hir_eager_expr_region(db).into(),
            TypeHirDecl::Union(decl) => decl.hir_eager_expr_region(db).into(),
        }
    }
}

impl HasHirDecl for TypePath {
    type HirDecl = TypeHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        ty_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn ty_hir_decl(db: &dyn HirDeclDb, path: TypePath) -> Option<TypeHirDecl> {
    match path.syn_decl(db).expect("no errors for hir stage") {
        TypeSynDecl::Enum(syn_decl) => Some(EnumTypeHirDecl::from_syn(path, syn_decl, db).into()),
        TypeSynDecl::PropsStruct(syn_decl) => {
            Some(PropsStructTypeHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeSynDecl::UnitStruct(syn_decl) => {
            Some(UnitStructHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeSynDecl::TupleStruct(syn_decl) => {
            Some(TupleStructTypeHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeSynDecl::Record(syn_decl) => {
            Some(RecordTypeHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeSynDecl::Inductive(_syn_decl) => None,
        TypeSynDecl::Structure(_syn_decl) => None,
        TypeSynDecl::Extern(syn_decl) => {
            Some(ExternTypeHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeSynDecl::Union(syn_decl) => Some(UnionHirDecl::from_syn(path, syn_decl, db).into()),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = HirDeclDb, jar = HirDeclJar)]
#[enum_class::from_variants]
pub enum PropsFieldEtherealSignature {
    PropsStruct(PropsStructFieldEtherealSignature),
}

impl PropsFieldEtherealSignature {
    pub fn ty(self) -> EtherealTerm {
        match self {
            PropsFieldEtherealSignature::PropsStruct(signature) => signature.ty(),
        }
    }
}
