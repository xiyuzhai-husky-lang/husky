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
    pub fn path(self, db: &dyn HirDeclDb) -> TypeItemPath {
        match self {
            TypeItemHirDecl::AssociatedFn(decl) => decl.path(db),
            TypeItemHirDecl::MethodFn(decl) => decl.path(db),
            TypeItemHirDecl::AssociatedType(_) => todo!(),
            TypeItemHirDecl::AssociatedVal(_) => todo!(),
            TypeItemHirDecl::MemoizedField(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, _db: &'a dyn HirDeclDb) -> &'a [HirTemplateParameter] {
        match self {
            TypeItemHirDecl::AssociatedFn(_) => todo!(),
            TypeItemHirDecl::MethodFn(_) => todo!(),
            TypeItemHirDecl::AssociatedType(_) => todo!(),
            TypeItemHirDecl::AssociatedVal(_) => todo!(),
            TypeItemHirDecl::MemoizedField(_) => todo!(),
        }
    }

    // pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> HirExprRegion {
    //     match self {
    //         TypeItemHirDecl::AssociatedFn(decl) => decl.hir_expr_region(db).into(),
    //         TypeItemHirDecl::MethodFn(decl) => decl.hir_expr_region(db).into(),
    //         TypeItemHirDecl::AssociatedType(decl) => decl.hir_expr_region(db).into(),
    //         TypeItemHirDecl::AssociatedVal(decl) => decl.hir_expr_region(db).into(),
    //         TypeItemHirDecl::MemoizedField(decl) => decl.hir_expr_region(db).into(),
    //     }
    // }
}

impl HasHirDecl for TypeItemPath {
    type HirDecl = TypeItemHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        ty_item_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
pub(crate) fn ty_item_hir_decl(db: &dyn HirDeclDb, path: TypeItemPath) -> Option<TypeItemHirDecl> {
    match path.ethereal_signature_template(db).expect("ok") {
        TypeItemEtherealSignatureTemplate::AssociatedFn(_) => todo!(),
        TypeItemEtherealSignatureTemplate::MethodFn(template) => {
            Some(TypeMethodFnHirDecl::from_ethereal(path, template, db).into())
        }
        TypeItemEtherealSignatureTemplate::MethodFunction(_) => None,
        TypeItemEtherealSignatureTemplate::MemoizedField(_) => todo!(),
    }
}

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
