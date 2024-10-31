use crate::{
    annotation::{
        space::{LxApplyAnnotation, VdSpaceAnnotation},
        token::{LxVariableAnnotation, VdTokenAnnotation},
    },
    annotations::VdAnnotations,
};

#[derive(Debug, Clone)]
pub struct VdAnnotationsExample {
    pub input: String,
    pub root_mode: LxMode,
    pub annotations: VdAnnotations,
}

impl VdAnnotationsExample {
    fn collect_from_sparse(
        examples: &[(
            LxMode,
            &str,
            &[((&str, &str), VdTokenAnnotation)],
            &[((&str, &str), VdSpaceAnnotation)],
        )],
    ) -> Vec<Self> {
        examples
            .iter()
            .map(
                |&(root_mode, input, token_annotations, space_annotations)| Self {
                    root_mode,
                    input: input.to_string(),
                    annotations: VdAnnotations::from_sparse(
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
    pub static ref LX_ANNOTATIONS_EXAMPLES: Vec<VdAnnotationsExample> =
        VdAnnotationsExample::collect_from_sparse(&[
            (LxMode::Math, "", &[], &[]),
            (
                LxMode::Math,
                "xy",
                &[
                    (
                        ("", "x"),
                        VdTokenAnnotation::Variable(LxVariableAnnotation::Usage)
                    ),
                    (
                        ("x", "y"),
                        VdTokenAnnotation::Variable(LxVariableAnnotation::Usage)
                    ),
                ],
                &[(
                    ("x", "y"),
                    VdSpaceAnnotation::Apply(LxApplyAnnotation::ScalarMul)
                ),]
            ),
            (
                LxMode::Math,
                "dx",
                &[
                    (("", "d"), VdTokenAnnotation::Differential),
                    (
                        ("d", "x"),
                        VdTokenAnnotation::Variable(
                            LxVariableAnnotation::SingleVariableIntegralVariableDecl
                        )
                    ),
                ],
                &[]
            ),
        ]);
}

#[test]
fn latex_annotations_examples_works() {
    let examples = &*LX_ANNOTATIONS_EXAMPLES; // Dereference the lazy_static
    expect_test::expect_file!["../../expect-files/annotations/examples.txt"]
        .assert_eq(&format!("{:#?}", examples));
}
