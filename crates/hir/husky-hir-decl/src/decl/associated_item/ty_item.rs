mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;

use super::*;

use husky_syn_decl::TypeItemSynDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum TypeItemHirDecl {
    AssociatedFn(TypeAssociatedFnHirDecl),
    AssociatedType(TypeAssociatedTypeHirDecl),
    AssociatedVal(TypeAssociatedValHirDecl),
    MethodFn(TypeMethodFnHirDecl),
    MemoizedField(TypeMemoizedFieldHirDecl),
}

impl TypeItemHirDecl {
    pub fn path(self, db: &::salsa::Db) -> TypeItemPath {
        match self {
            TypeItemHirDecl::AssociatedFn(decl) => decl.path(db),
            TypeItemHirDecl::MethodFn(decl) => decl.path(db),
            TypeItemHirDecl::AssociatedType(_) => todo!(),
            TypeItemHirDecl::AssociatedVal(_) => todo!(),
            TypeItemHirDecl::MemoizedField(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [HirTemplateParameter] {
        match self {
            TypeItemHirDecl::AssociatedFn(slf) => slf.template_parameters(db),
            TypeItemHirDecl::MethodFn(slf) => slf.template_parameters(db),
            TypeItemHirDecl::AssociatedType(slf) => &[],
            TypeItemHirDecl::AssociatedVal(slf) => &[],
            TypeItemHirDecl::MemoizedField(slf) => &[],
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> HirExprRegion {
        match self {
            TypeItemHirDecl::AssociatedFn(decl) => decl.hir_eager_expr_region(db).into(),
            TypeItemHirDecl::MethodFn(decl) => decl.hir_eager_expr_region(db).into(),
            TypeItemHirDecl::AssociatedType(decl) => decl.hir_eager_expr_region(db).into(),
            TypeItemHirDecl::AssociatedVal(decl) => decl.hir_expr_region(db).into(),
            TypeItemHirDecl::MemoizedField(decl) => decl.hir_eager_expr_region(db).into(),
        }
    }
}

impl HasHirDecl for TypeItemPath {
    type HirDecl = TypeItemHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        ty_item_hir_decl(db, self)
    }
}

// #[salsa::tracked(jar = HirDeclJar)]
pub(crate) fn ty_item_hir_decl(db: &::salsa::Db, path: TypeItemPath) -> Option<TypeItemHirDecl> {
    match path.syn_decl(db).expect("ok") {
        TypeItemSynDecl::AssociatedFn(syn_decl) => {
            Some(TypeAssociatedFnHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeItemSynDecl::MethodFn(syn_decl) => {
            Some(TypeMethodFnHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeItemSynDecl::AssociatedType(syn_decl) => {
            Some(TypeAssociatedTypeHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeItemSynDecl::AssociatedVal(syn_decl) => {
            Some(TypeAssociatedValHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeItemSynDecl::MemoizedField(syn_decl) => {
            Some(TypeMemoizedFieldHirDecl::from_syn(path, syn_decl, db).into())
        }
    }
}
// TypeItemEtherealSignatureTemplate::AssociatedFn(ethereal_signature_template) => {
//     Some(TypeAssociatedFnHirDecl::from_syn(path, ethereal_signature_template, db).into())
// }
// TypeItemEtherealSignatureTemplate::MethodFn(syn_decl) => {
//     Some(TypeMethodFnHirDecl::from_syn(path, syn_decl, db).into())
// }
// TypeItemEtherealSignatureTemplate::MethodFunction(_) => None,
// TypeItemEtherealSignatureTemplate::MemoizedField(ethereal_signature_template) => {
//     Some(TypeMemoizedFieldHirDecl::from_syn(path, ethereal_signature_template, db).into())
// }

// TypeItemDeclarativeSignatureTemplate::AssociatedFn(template) => {
//     TypeAssociatedFnHirDecl::from_declarative(db, path, template)?.into()
// }
// TypeItemDeclarativeSignatureTemplate::MethodFn(template) => {
//     TypeMethodFnHirDecl::from_declarative(db, template)?.into()
// }
// TypeItemDeclarativeSignatureTemplate::AssociatedType(_) => todo!(),
// TypeItemDeclarativeSignatureTemplate::AssociatedVal(_) => todo!(),
// TypeItemDeclarativeSignatureTemplate::MemoizedField(template) => {
//     TypeMemoizedFieldHirDecl::from_declarative(db, template)?.into()
// }
