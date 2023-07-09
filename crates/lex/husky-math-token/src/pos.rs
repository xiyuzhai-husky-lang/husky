mod determiner;

pub use self::determiner::*;

/// part of speech tokens
pub enum PosToken {
    Noun,
    Adjective,
    TransitiveVerb(TransitiveVerbToken),
    Determiner(DeterminerToken),
    UnidentifiedWord(UnidentifiedWordToken),
}

pub struct TransitiveVerbToken {}

pub struct UnidentifiedWordToken {}
