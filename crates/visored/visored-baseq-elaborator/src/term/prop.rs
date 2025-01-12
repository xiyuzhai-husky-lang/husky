pub mod num_chain;
pub mod num_relation;

use self::{num_chain::*, num_relation::*};
use super::*;

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqProp<'sess> {
    NumRelation(VdBsqNumRelation<'sess>),
    NumChain(VdBsqNumChain<'sess>),
    Trivial(bool),
}

impl<'sess> std::fmt::Debug for VdBsqProp<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PropTerm(`")?;
        self.show_fmt(VdPrecedenceRange::Any, f)?;
        f.write_str("`)")
    }
}

impl<'sess> VdBsqProp<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqProp::NumRelation(term) => term.show_fmt(precedence_range, f),
            VdBsqProp::NumChain(_) => todo!(),
            VdBsqProp::Trivial(b) => write!(f, "{}", b),
        }
    }
}
