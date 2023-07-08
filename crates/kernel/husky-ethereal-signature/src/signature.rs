mod associated_item;
mod derive_decr;
mod impl_block;
mod module_item;
mod variant;

pub use self::associated_item::*;
pub use self::derive_decr::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::variant::*;

use crate::*;
use husky_declarative_signature::*;

pub trait HasEtherealSignatureTemplate {
    type EtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate>;
}
