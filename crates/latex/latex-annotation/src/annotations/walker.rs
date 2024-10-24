use crate::annotation::{space::LxSpaceAnnotation, token::LxTokenAnnotation};

pub struct LxAnnotationsWalker<'a> {
    token_annotations: &'a [(usize, LxTokenAnnotation)],
    space_annotations: &'a [(usize, LxSpaceAnnotation)],
    next_token_annotation_index: usize,
    next_space_annotation_index: usize,
}

impl<'a> LxAnnotationsWalker<'a> {
    pub fn new(
        token_annotations: &'a [(usize, LxTokenAnnotation)],
        space_annotations: &'a [(usize, LxSpaceAnnotation)],
    ) -> Self {
        Self {
            token_annotations,
            space_annotations,
            next_token_annotation_index: 0,
            next_space_annotation_index: 0,
        }
    }
}
