mod trai;
mod ty;
mod ty_as_trai;

pub use self::trai::*;
pub use self::ty::*;
pub use self::ty_as_trai::*;

use crate::*;
use husky_decl::{TypeItemDecl, TypeMethodDecl};
use husky_entity_tree::{AssociatedItemId, EntityTreeBundleResult};
use husky_raw_ty::ty_path_ty_method_raw_ty;
use husky_signature::{SignatureResult, TypeMethodSignature};
use husky_word::IdentPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum MethodCard {
    Type(TypeMethodCard),
    TypeAsTrait(TypeAsTraitMethodCard),
}

#[salsa::tracked(db = TermDb, jar = TermJar, constructor = new_inner)]
pub struct TypeAsTraitMethodCard {
    #[id]
    id: AssociatedItemId,
    signature: SignatureResult<TypeMethodSignature>,
    method_ty: TermResult<Term>,
}
