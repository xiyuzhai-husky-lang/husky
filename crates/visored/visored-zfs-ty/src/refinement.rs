use crate::{term::ZfsTermId, ty::ZfsTypeData};

/// It's a term of type A -> B, where $A$ is the base type.
pub struct ZfsTypeRefinement {}

impl ZfsTypeRefinement {
    pub fn new(term: ZfsTermId, data: &ZfsTypeData, prev_refinements: &[Self]) -> Self {
        todo!()
    }
}
