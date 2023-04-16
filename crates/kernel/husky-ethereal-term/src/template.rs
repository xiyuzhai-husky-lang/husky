mod parameter;
mod term;
mod trai_for_ty_impl;

pub(crate) use self::parameter::*;
pub(crate) use self::trai_for_ty_impl::*;

use self::term::*;
use crate::*;
use husky_decr::Decr;
use husky_entity_tree::TraitForTypeImplBlock;
use husky_signature::{HasSignature, ImplicitParameterSignature, TypeSignature};

pub(crate) trait HasTemplate: Copy {
    type Template;

    fn template(self, db: &dyn EtherealTermDb) -> TermResult<Self::Template>;
}
