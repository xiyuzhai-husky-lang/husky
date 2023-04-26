/// a card contains checked information from signature
mod error;
mod field;
mod method;
mod pattern_matcher;

pub use self::error::*;
pub use self::field::*;
pub use self::method::*;

use self::pattern_matcher::*;
use crate::*;
use husky_ethereal_signature::*;
use husky_scope::*;

pub(crate) trait AsFluffySignatureTemplate: Copy {
    type FluffySignature;

    fn fluffy_signature(self, engine: &mut impl FluffyTermEngine) -> Self::FluffySignature;
}
