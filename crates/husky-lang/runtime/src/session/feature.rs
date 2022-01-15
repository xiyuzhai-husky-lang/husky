use common::*;
use syntax_types::Literal;

use super::*;

pub struct Feature {
    cached_values: HashMap<usize, SessionValue>,
    pub(super) kind: FeatureKind,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum FeatureKind {
    Literal(Literal),
    FunctionCall,
    Binary,
    MembAccess,
    MembCall,
}

impl Feature {
    pub(super) fn new(kind: FeatureKind) -> Self {
        Self {
            cached_values: HashMap::new(),
            kind,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct FeatureId(pub(super) usize);
