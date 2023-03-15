mod method;

pub use self::method::*;

use super::*;
use husky_decl::{TypeAssociatedFnDecl, TypeItemDecl};
use husky_entity_tree::AssociatedItemId;
use husky_signature::{SignatureResult, TypeAssociatedFnSignature, TypeItemSignature};

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

#[salsa::tracked(jar = TermJar)]
pub fn ty_item_card(db: &dyn TermDb, path: TypeItemPath) -> TermResult<TypeItemCard> {
    let decl = db
        .ty_item_decl(path)
        .ok_or(TermError::NoDeclForEntityPath {
            entity_path: path.into(),
        })?;
    Ok(match decl {
        TypeItemDecl::AssociatedFn(decl) => ty_associated_fn_card(db, decl)?.into(),
        TypeItemDecl::MethodFn(_) => todo!(),
        TypeItemDecl::AssociatedType(_) => todo!(),
        TypeItemDecl::AssociatedValue(_) => todo!(),
        TypeItemDecl::Memo(_) => todo!(),
    })
}

fn ty_associated_fn_card(
    db: &dyn TermDb,
    decl: TypeAssociatedFnDecl,
) -> TermResult<TypeAssociatedFnCard> {
    let signature = todo!();
    Ok(TypeAssociatedFnCard::new(db, decl.id(db)))
}
