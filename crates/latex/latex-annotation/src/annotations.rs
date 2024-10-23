mod builder;

use crate::annotation::{space::LxSpaceAnnotation, token::LxTokenAnnotation};
use builder::sparce::LxAnnotationSparseBuilder;
use husky_text::Text;
use latex_token::{idx::LxTokenIdx, storage::LxTokenStorage};

#[derive(Debug, PartialEq, Eq)]
pub struct Annotations {
    token_annotations: Vec<LxTokenAnnotation>,
    space_annotations: Vec<LxSpaceAnnotation>,
}

impl Annotations {
    pub fn new(
        token_annotations: Vec<LxTokenAnnotation>,
        space_annotations: Vec<LxSpaceAnnotation>,
    ) -> Self {
        Self {
            token_annotations,
            space_annotations,
        }
    }

    pub fn from_sparse<'a>(
        token_annotation_iter: impl Iterator<Item = (&'a str, LxTokenAnnotation)>,
        space_annotation_iter: impl Iterator<Item = ((&'a str, &'a str), LxSpaceAnnotation)>,
        token_storage: &'a LxTokenStorage,
        text: Text<'a>,
    ) -> Self {
        let mut builder = LxAnnotationSparseBuilder::new(
            token_storage,
            text,
            token_annotation_iter,
            space_annotation_iter,
        );
        builder.collect_all_annotations();
        builder.finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationIdx {
    Token(LxTokenIdx),
    Space(LxTokenIdx),
}
