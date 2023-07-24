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
use husky_item_path::TypePath;

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

    pub fn generic_parameters<'a>(self, db: &'a dyn HirDeclDb) -> &'a [EtherealGenericParameter] {
        match self {
            TypeHirDecl::Enum(decl) => decl.generic_parameters(db),
            TypeHirDecl::UnitStruct(decl) => decl.generic_parameters(db),
            TypeHirDecl::TupleStruct(decl) => decl.generic_parameters(db),
            TypeHirDecl::PropsStruct(decl) => decl.generic_parameters(db),
            TypeHirDecl::Record(decl) => decl.generic_parameters(db),
            TypeHirDecl::Extern(decl) => decl.generic_parameters(db),
            TypeHirDecl::Union(decl) => decl.generic_parameters(db),
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> HirExprRegion {
        match self {
            TypeHirDecl::Enum(decl) => decl.hir_expr_region(db),
            TypeHirDecl::UnitStruct(decl) => decl.hir_expr_region(db),
            TypeHirDecl::TupleStruct(decl) => decl.hir_expr_region(db),
            TypeHirDecl::PropsStruct(decl) => decl.hir_expr_region(db),
            TypeHirDecl::Record(decl) => decl.hir_expr_region(db),
            TypeHirDecl::Extern(decl) => decl.hir_expr_region(db),
            TypeHirDecl::Union(decl) => decl.hir_expr_region(db),
        }
    }
}

impl HasHirDecl for TypePath {
    type HirDecl = TypeHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Self::HirDecl {
        ty_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn ty_hir_decl(db: &dyn HirDeclDb, path: TypePath) -> TypeHirDecl {
    todo!()
    // Ok(match path.declarative_signature_template(db)? {
    //     TypeDeclarativeSignatureTemplate::Enum(declarative_signature_template) => {
    //         EnumHirDecl::from_declarative(db, path, declarative_signature_template)?.into()
    //     }
    //     TypeDeclarativeSignatureTemplate::PropsStruct(declarative_signature_template) => {
    //         PropsStructHirDecl::from_declarative(db, declarative_signature_template)?.into()
    //     }
    //     TypeDeclarativeSignatureTemplate::UnitStruct(_) => todo!(),
    //     TypeDeclarativeSignatureTemplate::TupleStruct(_) => todo!(),
    //     TypeDeclarativeSignatureTemplate::Record(_) => todo!(),
    //     TypeDeclarativeSignatureTemplate::Inductive(_) => todo!(),
    //     TypeDeclarativeSignatureTemplate::Structure(_) => todo!(),
    //     TypeDeclarativeSignatureTemplate::Extern(_) => todo!(),
    //     TypeDeclarativeSignatureTemplate::Union(_) => todo!(),
    // })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = HirDeclDb)]
#[enum_class::from_variants]
pub enum RegularFieldEtherealSignature {
    PropsStruct(PropsStructFieldEtherealSignature),
}

impl RegularFieldEtherealSignature {
    pub fn ty(self) -> EtherealTerm {
        match self {
            RegularFieldEtherealSignature::PropsStruct(signature) => signature.ty(),
        }
    }
}
