mod associated_item;
mod derive_decr;
mod fugitive;
mod impl_block;
mod method;
mod trai;
mod ty;
mod variant;

pub use self::associated_item::*;
pub use self::derive_decr::*;
pub use self::fugitive::*;
pub use self::impl_block::*;
pub use self::method::*;
pub use self::trai::*;
pub use self::ty::*;
pub use self::variant::*;

use crate::*;
use husky_declarative_signature::*;

pub trait HasEtherealSignatureTemplate {
    type EtherealSignatureTemplate;

    fn ethereal_signature_template<'a>(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate>;
}
