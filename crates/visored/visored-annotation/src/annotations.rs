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
    token_annotation_entries: Vec<VdTokenAnnotationEntry>,
    space_annotation_entries: Vec<VdSpaceAnnotationEntry>,
    token_annotations: Vec<VdTokenAnnotation>,
}

impl VdAnnotations {
    pub fn new(
        token_annotation_entries: Vec<VdTokenAnnotationEntry>,
        space_annotation_entries: Vec<VdSpaceAnnotationEntry>,
        token_storage: &LxTokenStorage,
    ) -> Self {
        let token_annotations = collect_token_annotations(&token_annotation_entries, token_storage);
        Self {
            token_annotation_entries,
            space_annotation_entries,
            token_annotations,
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

    pub fn token_annotation_entries(&self) -> &[VdTokenAnnotationEntry] {
        &self.token_annotation_entries
    }

    pub fn space_annotation_entries(&self) -> &[VdSpaceAnnotationEntry] {
        &self.space_annotation_entries
    }

    pub fn walker(&self) -> VdAnnotationsWalker {
        VdAnnotationsWalker::new(
            &self.token_annotation_entries,
            &self.space_annotation_entries,
        )
    }
}

fn collect_token_annotations(
    token_annotation_entries: &[VdTokenAnnotationEntry],
    token_storage: &LxTokenStorage,
) -> Vec<VdTokenAnnotation> {
    todo!()
}
