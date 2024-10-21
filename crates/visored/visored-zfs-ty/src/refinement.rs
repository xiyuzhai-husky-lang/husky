use crate::{term::VdZfsTermId, ty::VdZfsTypeData};

/// It's a term of type A -> B, where $A$ is the base type.
pub struct ZfsTypeRefinement {}

impl ZfsTypeRefinement {
    pub fn new(term: VdZfsTermId, data: &VdZfsTypeData, prev_refinements: &[Self]) -> Self {
        todo!()
    }
}
