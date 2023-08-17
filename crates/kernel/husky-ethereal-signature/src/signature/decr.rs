mod derive;

pub use self::derive::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb)]
#[enum_class::from_variants]
pub enum DecrEtherealSignatureTemplate {
    Derive(DeriveDecrEtherealSignatureTemplate),
}
