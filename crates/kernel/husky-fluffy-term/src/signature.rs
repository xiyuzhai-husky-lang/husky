/// a card contains checked information from signature
mod error;
mod field;
mod indirection;
mod method;

pub use self::error::*;
pub use self::field::*;
pub use self::indirection::*;
pub use self::method::*;

use crate::*;
use husky_ethereal_signature::*;
use husky_scope::*;

pub(crate) trait AsFluffySignatureTemplate: Copy {
    type FluffySignature;

    fn fluffy_signature(self, engine: &mut impl FluffyTermEngine) -> Self::FluffySignature;
}
