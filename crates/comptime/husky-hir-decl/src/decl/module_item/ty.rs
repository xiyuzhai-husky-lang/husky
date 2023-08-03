mod r#enum;
mod r#extern;
mod props_struct;
mod record;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::record::*;
pub use self::tuple_struct::*;
pub use self::union::*;
pub use self::unit_struct::*;

use super::*;
use husky_entity_path::TypePath;
use husky_print_utils::p;
use salsa::DebugWithDb;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum TypeHirDecl {
    Enum(EnumTypeHirDecl),
    PropsStruct(PropsStructTypeHirDecl),
    UnitStruct(UnitStructTypeHirDecl),
    TupleStruct(TupleStructTypeHirDecl),
    Record(RecordTypeHirDecl),
    Extern(ExternTypeHirDecl),
    Union(UnionTypeHirDecl),
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
            TypeHirDecl::Enum(decl) => decl.hir_expr_region(db).into(),
            TypeHirDecl::UnitStruct(decl) => decl.hir_expr_region(db).into(),
            TypeHirDecl::TupleStruct(decl) => decl.hir_expr_region(db).into(),
            TypeHirDecl::PropsStruct(decl) => decl.hir_expr_region(db).into(),
            TypeHirDecl::Record(decl) => decl.hir_expr_region(db).into(),
            TypeHirDecl::Extern(decl) => decl.hir_expr_region(db).into(),
            TypeHirDecl::Union(decl) => decl.hir_expr_region(db).into(),
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
    p!(
        path.ethereal_signature_template(db).debug(db),
        path.debug(db)
    );
    match path
        .ethereal_signature_template(db)
        .expect("no errors for hir stage")
    {
        TypeEtherealSignatureTemplate::Enum(ethereal_signature_template) => {
            Some(EnumTypeHirDecl::from_ethereal(path, ethereal_signature_template, db).into())
        }
        TypeEtherealSignatureTemplate::PropsStruct(ethereal_signature_template) => Some(
            PropsStructTypeHirDecl::from_ethereal(path, ethereal_signature_template, db).into(),
        ),
        TypeEtherealSignatureTemplate::UnitStruct(ethereal_signature_template) => {
            Some(UnitStructTypeHirDecl::from_ethereal(path, ethereal_signature_template, db).into())
        }
        TypeEtherealSignatureTemplate::TupleStruct(ethereal_signature_template) => Some(
            TupleStructTypeHirDecl::from_ethereal(path, ethereal_signature_template, db).into(),
        ),
        TypeEtherealSignatureTemplate::Record(_) => todo!(),
        TypeEtherealSignatureTemplate::Inductive(_)
        | TypeEtherealSignatureTemplate::Structure(_) => None,
        TypeEtherealSignatureTemplate::Extern(ethereal_signature_template) => {
            Some(ExternTypeHirDecl::new(path, ethereal_signature_template, db).into())
        }
        TypeEtherealSignatureTemplate::Union(_) => todo!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = HirDeclDb)]
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
