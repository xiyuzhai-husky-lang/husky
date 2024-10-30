use crate::{
    annotation::{space::LxSpaceAnnotation, token::LxTokenAnnotation},
    annotations::LxAnnotations,
};

pub struct LxAnnotationsExample {
    pub root_mode: LxMode,
    pub input: String,
    pub annotations: LxAnnotations,
}

pub type LxAnnotationsExamples = Vec<LxAnnotationsExample>;

impl LxAnnotationsExample {
    fn collect_from_sparse(
        examples: &[(
            LxMode,
            &str,
            &[((&str, &str), LxTokenAnnotation)],
            &[((&str, &str), LxSpaceAnnotation)],
        )],
    ) -> LxAnnotationsExamples {
        examples
            .iter()
            .map(
                |&(root_mode, input, token_annotations, space_annotations)| Self {
                    root_mode,
                    input: input.to_string(),
                    annotations: LxAnnotations::from_sparse(
                        input,
                        token_annotations.iter().copied(),
                        space_annotations.iter().copied(),
                    ),
                },
            )
            .collect()
    }
}

use latex_prelude::mode::LxMode;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref EXAMPLES: LxAnnotationsExamples =
        LxAnnotationsExample::collect_from_sparse(&[(LxMode::Math, "", &[], &[])]);
}
