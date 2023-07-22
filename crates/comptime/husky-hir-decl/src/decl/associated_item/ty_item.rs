mod associated_fn;
mod associated_val;
mod memoized_field;
mod method_fn;
mod method_function;

pub use self::associated_fn::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;
pub use self::method_function::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum TypeItemHirDecl {
    AssociatedFn(TypeAssociatedFnHirDecl),
    MethodFn(TypeMethodFnHirDecl),
    MethodFunction(TypeMethodFunctionHirDecl),
    MemoizedField(TypeMemoizedFieldHirDecl),
}

impl HasHirDecl for TypeItemPath {
    type HirDecl = TypeItemHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Self::HirDecl {
        ty_item_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
pub(crate) fn ty_item_hir_decl(db: &dyn HirDeclDb, path: TypeItemPath) -> TypeItemHirDecl {
    todo!()
    // Ok(match path.declarative_signature_template(db)? {
    //     TypeItemDeclarativeSignatureTemplate::AssociatedFn(template) => {
    //         TypeAssociatedFnHirDecl::from_declarative(db, path, template)?.into()
    //     }
    //     TypeItemDeclarativeSignatureTemplate::MethodFn(template) => {
    //         TypeMethodFnHirDecl::from_declarative(db, template)?.into()
    //     }
    //     TypeItemDeclarativeSignatureTemplate::AssociatedType(_) => todo!(),
    //     TypeItemDeclarativeSignatureTemplate::AssociatedVal(_) => todo!(),
    //     TypeItemDeclarativeSignatureTemplate::MemoizedField(template) => {
    //         TypeMemoizedFieldHirDecl::from_declarative(db, template)?.into()
    //     }
    // })
}
