mod method;

pub use self::method::*;

use super::*;
use husky_decl::{TypeAssociatedFnDecl, TypeItemDecl};
use husky_entity_tree::AssociatedItemId;
use husky_raw_ty::HasRawType;

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

pub(crate) fn ty_item_path_ty_unchecked(db: &dyn TermDb, path: TypeItemPath) -> TermResult<Term> {
    Term::ty_from_raw_unchecked(db, path.raw_ty(db)?)
}
