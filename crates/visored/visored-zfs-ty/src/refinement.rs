use crate::{term::VisoredZfsTermId, ty::VisoredZfsTypeData};

/// It's a term of type A -> B, where $A$ is the base type.
pub struct VisoredZfsTypeRefinement {}

impl VisoredZfsTypeRefinement {
    pub fn new(
        term: VisoredZfsTermId,
        data: &VisoredZfsTypeData,
        prev_refinements: &[Self],
    ) -> Self {
        todo!()
    }
}
