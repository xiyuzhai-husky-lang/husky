mod builder;
mod walker;

use crate::annotation::{space::LxSpaceAnnotation, token::LxTokenAnnotation};
use builder::sparce::collect_from_sparse_annotations;
use husky_text::Text;
use latex_token::{idx::LxTokenIdx, storage::LxTokenStorage};
use walker::LxAnnotationsWalker;

#[derive(Debug, PartialEq, Eq)]
pub struct LxAnnotations {
    token_annotations: Vec<(usize, LxTokenAnnotation)>,
    space_annotations: Vec<(usize, LxSpaceAnnotation)>,
}

impl LxAnnotations {
    pub fn new(
        token_annotations: Vec<(usize, LxTokenAnnotation)>,
        space_annotations: Vec<(usize, LxSpaceAnnotation)>,
    ) -> Self {
        Self {
            token_annotations,
            space_annotations,
        }
    }

    pub fn from_sparse<'a>(
        input: &'a str,
        token_annotation_iter: impl Iterator<Item = (&'a str, LxTokenAnnotation)>,
        space_annotation_iter: impl Iterator<Item = (&'a str, LxSpaceAnnotation)>,
    ) -> Self {
        collect_from_sparse_annotations(input, token_annotation_iter, space_annotation_iter)
    }
}

impl LxAnnotations {
    pub fn token_annotations(&self) -> &[(usize, LxTokenAnnotation)] {
        &self.token_annotations
    }

    pub fn space_annotations(&self) -> &[(usize, LxSpaceAnnotation)] {
        &self.space_annotations
    }

    pub fn walker(&self) -> LxAnnotationsWalker {
        LxAnnotationsWalker::new(&self.token_annotations, &self.space_annotations)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationIdx {
    Token(LxTokenIdx),
    Space(LxTokenIdx),
}
