use crate::{
    annotation::{space::LxSpaceAnnotation, token::LxTokenAnnotation},
    annotations::LxAnnotations,
};

pub(crate) fn collect_from_sparse_annotations<'a>(
    raw_text: &'a str,
    token_annotation_iter: impl Iterator<Item = (&'a str, LxTokenAnnotation)>,
    space_annotation_iter: impl Iterator<Item = (&'a str, LxSpaceAnnotation)>,
) -> LxAnnotations {
    let token_annotations = collect_from_sparse_token_annotations(raw_text, token_annotation_iter);
    let space_annotations = collect_from_sparse_space_annotations(raw_text, space_annotation_iter);

    LxAnnotations::new(token_annotations, space_annotations)
}

fn collect_from_sparse_token_annotations<'a>(
    raw_text: &'a str,
    token_annotation_iter: impl Iterator<Item = (&'a str, LxTokenAnnotation)>,
) -> Vec<(usize, LxTokenAnnotation)> {
    collect_from_sparse_annotations_aux(raw_text, token_annotation_iter)
}

fn collect_from_sparse_space_annotations<'a>(
    raw_text: &'a str,
    space_annotation_iter: impl Iterator<Item = (&'a str, LxSpaceAnnotation)>,
) -> Vec<(usize, LxSpaceAnnotation)> {
    collect_from_sparse_annotations_aux(raw_text, space_annotation_iter)
}

fn collect_from_sparse_annotations_aux<'a, A>(
    raw_text: &'a str,
    annotation_iter: impl Iterator<Item = (&'a str, A)>,
) -> Vec<(usize, A)> {
    let mut annotations = Vec::new();
    let mut offset = 0;
    for (s, annotation) in annotation_iter {
        assert_eq!(&raw_text[offset..offset + s.len()], s);
        offset += s.len();
        annotations.push((offset, annotation));
    }
    annotations
}

#[cfg(test)]
mod tests {
    use crate::annotation::{
        space::LxApplyAnnotation,
        token::{LxIntegralAnnotation, LxVariableAnnotation},
    };

    use super::*;

    #[test]
    fn test_collect_from_sparse_annotations_integral() {
        let input = "\\int xdx";

        let token_annotations = vec![
            (
                "\\int",
                LxTokenAnnotation::Integral(
                    LxIntegralAnnotation::SingleVariableIndefiniteIntegralOverReal,
                ),
            ),
            (
                " x",
                LxTokenAnnotation::Variable(LxVariableAnnotation::Usage),
            ),
            ("d", LxTokenAnnotation::Differential),
            (
                "x",
                LxTokenAnnotation::Variable(
                    LxVariableAnnotation::SingleVariableIntegralVariableDecl,
                ),
            ),
        ];

        let space_annotations = vec![
            (
                "\\int",
                LxSpaceAnnotation::Apply(LxApplyAnnotation::Integration),
            ),
            (
                " x",
                LxSpaceAnnotation::Apply(LxApplyAnnotation::ScalarDifferentialFormMul),
            ),
            (
                "d",
                LxSpaceAnnotation::Apply(LxApplyAnnotation::Differentiation),
            ),
        ];

        let result = collect_from_sparse_annotations(
            input,
            token_annotations.iter().copied(),
            space_annotations.iter().copied(),
        );

        assert_eq!(result.token_annotations().len(), 4);
        assert_eq!(result.space_annotations().len(), 3);

        // Check token annotations
        assert_eq!(
            result.token_annotations()[0],
            (
                4,
                LxTokenAnnotation::Integral(
                    LxIntegralAnnotation::SingleVariableIndefiniteIntegralOverReal
                )
            )
        );
        assert_eq!(
            result.token_annotations()[1],
            (6, LxTokenAnnotation::Variable(LxVariableAnnotation::Usage))
        );
        assert_eq!(
            result.token_annotations()[2],
            (7, LxTokenAnnotation::Differential)
        );
        assert_eq!(
            result.token_annotations()[3],
            (
                8,
                LxTokenAnnotation::Variable(
                    LxVariableAnnotation::SingleVariableIntegralVariableDecl
                )
            )
        );

        // Check space annotations
        assert_eq!(
            result.space_annotations()[0],
            (4, LxSpaceAnnotation::Apply(LxApplyAnnotation::Integration))
        );
        assert_eq!(
            result.space_annotations()[1],
            (
                6,
                LxSpaceAnnotation::Apply(LxApplyAnnotation::ScalarDifferentialFormMul)
            )
        );
        assert_eq!(
            result.space_annotations()[2],
            (
                7,
                LxSpaceAnnotation::Apply(LxApplyAnnotation::Differentiation)
            )
        );

        // Verify that the strings sum up to the original text
        let reconstructed_text: String = token_annotations.iter().map(|&(s, _)| s).collect();
        assert_eq!(reconstructed_text, input);

        let space_reconstructed_text: String = space_annotations.iter().map(|&(s, _)| s).collect();
        assert!(input.starts_with(&space_reconstructed_text));

        // Verify the order of annotations
        let all_token_annotations: Vec<&str> = token_annotations.iter().map(|&(s, _)| s).collect();
        assert_eq!(all_token_annotations, vec!["\\int", " x", "d", "x"]);

        let all_space_annotations: Vec<&str> = space_annotations.iter().map(|&(s, _)| s).collect();
        assert_eq!(all_space_annotations, vec!["\\int", " x", "d"]);
    }
}
