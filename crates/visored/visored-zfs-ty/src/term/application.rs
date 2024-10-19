use super::{VisoredZfsTerm, VisoredZfsTermId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApplicationVisoredZfsTerm(VisoredZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApplicationVisoredZfsTermData {
    pub function: VisoredZfsTerm,
    pub argument: VisoredZfsTerm,
}
