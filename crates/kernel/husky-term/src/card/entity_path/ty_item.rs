mod method;

pub use self::method::*;

use super::*;
use husky_decl::{TypeAssociatedFnDecl, TypeItemDecl};
use husky_entity_tree::AssociatedItemId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
#[enum_class::from_variants]
pub enum TypeItemCard {
    MethodFn(TypeMethodFnCard),
    AssociatedFn(TypeAssociatedFnCard),
}

#[salsa::tracked(db = TermDb, jar = TermJar)]
pub struct TypeAssociatedFnCard {
    #[id]
    pub id: AssociatedItemId,
}

pub(crate) fn ty_item_path_ty(db: &dyn TermDb, path: TypeItemPath) -> TermResult<Term> {
    ty_item_path_ty_unchecked(db, path)?.checked(db)
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn ty_item_path_ty_unchecked(db: &dyn TermDb, path: TypeItemPath) -> TermResult<Term> {
    let decl = db
        .ty_item_decl(path)
        .ok_or(TermError::NoDeclForEntityPath {
            entity_path: path.into(),
        })?;
    Ok(match decl {
        TypeItemDecl::AssociatedFn(decl) => ty_associated_fn_ty_unchecked(db, decl)?.into(),
        TypeItemDecl::MethodFn(_) => todo!(),
        TypeItemDecl::AssociatedType(_) => todo!(),
        TypeItemDecl::AssociatedValue(_) => todo!(),
        TypeItemDecl::Memo(_) => todo!(),
    })
}

fn ty_associated_fn_ty_unchecked(db: &dyn TermDb, decl: TypeAssociatedFnDecl) -> TermResult<Term> {
    Ok(todo!())
}
