pub mod num_relationship;

use super::*;
use crate::term::num_relationship::*;

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqPropTerm<'sess> {
    NumRelationship(VdBsqNumRelationshipPropTerm<'sess>),
}

#[floated]
pub struct VdBsqPropTermFld<'sess> {
    pub data: VdBsqPropTermData<'sess>,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqPropTermData<'sess> {
    NumRelationship(VdBsqNumRelationshipPropTermData<'sess>),
}
