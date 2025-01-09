pub mod num_relationship;

use super::*;
use crate::term::num_relationship::*;

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqPropTerm<'sess> {
    NumRelationship(VdBsqNumRelationshipPropTerm<'sess>),
    Trivial(bool),
}

#[floated]
pub struct VdBsqPropTermFld<'sess> {
    #[return_ref]
    pub data: VdBsqPropTermData<'sess>,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqPropTermData<'sess> {
    NumRelationship(VdBsqNumRelationshipPropTermData<'sess>),
}

impl<'sess> std::fmt::Debug for VdBsqPropTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PropTerm(`")?;
        self.show_fmt(VdPrecedenceRange::Any, f)?;
        f.write_str("`)")
    }
}

impl<'sess> VdBsqPropTerm<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqPropTerm::NumRelationship(term) => term.show_fmt(precedence_range, f),
            VdBsqPropTerm::Trivial(b) => write!(f, "{}", b),
        }
    }
}
