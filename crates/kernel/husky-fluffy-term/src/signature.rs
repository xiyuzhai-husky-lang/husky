/// a card contains checked information from signature
mod error;
mod field;
mod index;
mod method;

pub use self::error::*;
pub use self::field::*;
pub use self::index::*;
pub use self::method::*;

use crate::*;
use husky_ethereal_signature::*;
use husky_scope::*;

pub(crate) trait AsFluffySignatureTemplate: Copy {
    type FluffySignature;

    fn fluffy_signature(self, engine: &mut impl FluffyTermEngine) -> Self::FluffySignature;
}
