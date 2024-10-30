mod builder;
pub mod walker;

use crate::annotation::{space::LxSpaceAnnotation, token::LxTokenAnnotation};
use builder::sparce::collect_from_sparse_annotations;
use walker::LxAnnotationsWalker;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LxAnnotationEntry<A> {
    pub start: usize,
    pub end: usize,
    pub annotation: A,
}

pub type LxTokenAnnotationEntry = LxAnnotationEntry<LxTokenAnnotation>;
pub type LxSpaceAnnotationEntry = LxAnnotationEntry<LxSpaceAnnotation>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LxAnnotations {
    token_annotations: Vec<LxTokenAnnotationEntry>,
    space_annotations: Vec<LxSpaceAnnotationEntry>,
}

impl LxAnnotations {
    pub fn new(
        token_annotations: Vec<LxTokenAnnotationEntry>,
        space_annotations: Vec<LxSpaceAnnotationEntry>,
    ) -> Self {
        Self {
            token_annotations,
            space_annotations,
        }
    }

    pub fn from_sparse<'a>(
        input: &'a str,
        token_annotations: impl IntoIterator<Item = ((&'a str, &'a str), LxTokenAnnotation)>,
        space_annotations: impl IntoIterator<Item = ((&'a str, &'a str), LxSpaceAnnotation)>,
    ) -> Self {
        collect_from_sparse_annotations(
            input,
            token_annotations.into_iter(),
            space_annotations.into_iter(),
        )
    }

    pub fn token_annotations(&self) -> &[LxTokenAnnotationEntry] {
        &self.token_annotations
    }

    pub fn space_annotations(&self) -> &[LxSpaceAnnotationEntry] {
        &self.space_annotations
    }

    pub fn walker(&self) -> LxAnnotationsWalker {
        LxAnnotationsWalker::new(&self.token_annotations, &self.space_annotations)
    }
}
