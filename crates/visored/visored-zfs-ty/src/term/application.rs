use super::{VisoredZfsTerm, VisoredZfsTermId, VisoredZfsTerms};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApplicationVisoredZfsTerm(VisoredZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApplicationVisoredZfsTermData {
    pub function: VisoredZfsTerm,
    pub arguments: VisoredZfsTerms,
}
