mod engine;
mod error;
mod region;

pub use error::*;
pub use region::*;

pub(crate) use engine::*;

use crate::*;

#[enum_class::from_variants]
pub enum RawSignatureRawTerm {
    EntityPath(RawSignatureRawTermEntityPath),
    Application(RawSignatureRawTermApplication),
    Curry(RawSignatureRawTermCurry),
}

pub enum RawSignatureRawTermEntityPath {
    Form(FormPath),
    Trai(TraitPath),
    Type(TypePath),
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct RawSignatureRawTermApplication {}

/// representing term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct RawSignatureRawTermCurry {
    pub curry_kind: RawTermCurryKind,
    pub variance: Variance,
    /// a
    pub input_symbol: Option<RawTermSymbol>,
    /// X
    pub input_ty: RawTerm,
    /// Y
    pub return_ty: RawTerm,
}
