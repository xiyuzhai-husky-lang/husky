mod engine;
mod error;
mod region;

pub use error::*;
pub use region::*;

pub(crate) use engine::*;

use crate::*;

#[enum_class::from_variants]
pub enum SignatureRawTerm {
    EntityPath(SignatureRawTermEntityPath),
    Application(SignatureRawTermApplication),
    Curry(SignatureRawTermCurry),
}

pub enum SignatureRawTermEntityPath {
    Form(FormPath),
    Trai(TraitPath),
    Type(TypePath),
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct SignatureRawTermApplication {}

/// representing term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct SignatureRawTermCurry {
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub input_symbol: Option<RawTermSymbol>,
    /// X
    pub input_ty: RawTerm,
    /// Y
    pub return_ty: RawTerm,
}
