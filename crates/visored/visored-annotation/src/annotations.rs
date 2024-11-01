mod builder;
pub mod walker;

use crate::{
    annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation},
    *,
};
use builder::sparce::collect_from_sparse_annotations;
use latex_token::storage::LxTokenStorage;
use walker::VdAnnotationsWalker;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VdAnnotationEntry<A> {
    pub start: usize,
    pub end: usize,
    pub annotation: A,
}

pub type VdTokenAnnotationEntry = VdAnnotationEntry<VdTokenAnnotation>;
pub type VdSpaceAnnotationEntry = VdAnnotationEntry<VdSpaceAnnotation>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VdAnnotations {
    token_annotations: Vec<VdTokenAnnotationEntry>,
    space_annotations: Vec<VdSpaceAnnotationEntry>,
}

impl VdAnnotations {
    pub fn new(
        token_annotations: Vec<VdTokenAnnotationEntry>,
        space_annotations: Vec<VdSpaceAnnotationEntry>,
        token_storage: &LxTokenStorage,
    ) -> Self {
        Self {
            token_annotations,
            space_annotations,
        }
    }

    pub fn from_sparse<'a>(
        input: &'a str,
        token_annotations: impl IntoIterator<Item = ((&'a str, &'a str), VdTokenAnnotation)>,
        space_annotations: impl IntoIterator<Item = ((&'a str, &'a str), VdSpaceAnnotation)>,
        token_storage: &LxTokenStorage,
    ) -> Self {
        collect_from_sparse_annotations(
            input,
            token_annotations.into_iter(),
            space_annotations.into_iter(),
            token_storage,
        )
    }

    pub fn token_annotations(&self) -> &[VdTokenAnnotationEntry] {
        &self.token_annotations
    }

    pub fn space_annotations(&self) -> &[VdSpaceAnnotationEntry] {
        &self.space_annotations
    }

    pub fn walker(&self) -> VdAnnotationsWalker {
        VdAnnotationsWalker::new(&self.token_annotations, &self.space_annotations)
    }
}
