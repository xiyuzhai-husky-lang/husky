mod engine;
mod error;
mod region;

pub use error::*;
pub use region::*;

pub(crate) use engine::*;

use crate::*;

#[enum_class::from_variants]
pub enum RawSignatureTerm {
    EntityPath(RawSignatureTermEntityPath),
    Application(RawSignatureTermApplication),
    Curry(RawSignatureTermCurry),
}

pub enum RawSignatureTermEntityPath {
    Form(FormPath),
    Trai(TraitPath),
    Type(TypePath),
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct RawSignatureTermApplication {}

/// representing term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct RawSignatureTermCurry {
    pub curry_kind: TermCurryKind,
    pub variance: Variance,
    /// a
    pub input_symbol: Option<TermSymbol>,
    /// X
    pub input_ty: Term,
    /// Y
    pub return_ty: Term,
}
