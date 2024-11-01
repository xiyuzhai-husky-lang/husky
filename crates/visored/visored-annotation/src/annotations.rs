mod builder;

use crate::{
    annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation},
    *,
};
use builder::sparce::collect_from_sparse_annotations;
use latex_token::storage::LxTokenStorage;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VdAnnotationRecord<A> {
    pub start: usize,
    pub end: usize,
    pub annotation: A,
}

pub type VdTokenAnnotationRecord = VdAnnotationRecord<VdTokenAnnotation>;
pub type VdSpaceAnnotationRecord = VdAnnotationRecord<VdSpaceAnnotation>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VdAnnotations {
    token_annotation_records: Vec<VdTokenAnnotationRecord>,
    space_annotation_records: Vec<VdSpaceAnnotationRecord>,
    /// the ith element is the annotation of the ith token
    token_annotations: Vec<Option<VdTokenAnnotation>>,
    /// the ith element is the annotation of the space before the ith token
    space_annotations: Vec<Option<VdSpaceAnnotation>>,
}

impl VdAnnotations {
    pub fn new(
        token_annotation_records: Vec<VdTokenAnnotationRecord>,
        space_annotation_records: Vec<VdSpaceAnnotationRecord>,
        token_storage: &LxTokenStorage,
    ) -> Self {
        let token_annotations = collect_token_annotations(&token_annotation_records, token_storage);
        let space_annotations = collect_space_annotations(&space_annotation_records, token_storage);
        Self {
            token_annotation_records,
            space_annotation_records,
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

    pub fn token_annotation_records(&self) -> &[VdTokenAnnotationRecord] {
        &self.token_annotation_records
    }

    pub fn space_annotation_records(&self) -> &[VdSpaceAnnotationRecord] {
        &self.space_annotation_records
    }
}

fn collect_token_annotations(
    token_annotation_entries: &[VdTokenAnnotationRecord],
    token_storage: &LxTokenStorage,
) -> Vec<Option<VdTokenAnnotation>> {
    let mut entry_idx = 0;
    token_storage
        .ranged_tokens()
        .iter()
        .map(|&((start, end), _, _)| {
            if entry_idx < token_annotation_entries.len()
                && token_annotation_entries[entry_idx].start == start
            {
                assert_eq!(token_annotation_entries[entry_idx].end, end);
                entry_idx += 1;
                Some(token_annotation_entries[entry_idx - 1].annotation)
            } else {
                None
            }
        })
        .collect()
}

fn collect_space_annotations(
    space_annotation_entries: &[VdSpaceAnnotationRecord],
    token_storage: &LxTokenStorage,
) -> Vec<Option<VdSpaceAnnotation>> {
    let mut entry_idx = 0;
    token_storage
        .ranged_tokens()
        .iter()
        .map(|&((start, end), _, _)| {
            let annotation = if entry_idx < space_annotation_entries.len()
                && space_annotation_entries[entry_idx].start == start
            {
                assert_eq!(space_annotation_entries[entry_idx].end, end);
                entry_idx += 1;
                Some(space_annotation_entries[entry_idx - 1].annotation)
            } else {
                None
            };
            annotation
        })
        .collect()
}
