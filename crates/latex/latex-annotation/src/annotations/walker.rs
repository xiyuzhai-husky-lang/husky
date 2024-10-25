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

impl<'a> LxAnnotationsWalker<'a> {
    #[track_caller]
    pub fn next(&mut self, start: usize, end: usize) -> (LxTokenAnnotation, LxSpaceAnnotation) {
        (
            self.next_token_annotation(end),
            self.next_space_annotation(start),
        )
    }

    #[track_caller]
    fn next_token_annotation(&mut self, offset: usize) -> LxTokenAnnotation {
        next_annotation_aux(
            offset,
            self.token_annotations,
            &mut self.next_token_annotation_index,
        )
    }

    #[track_caller]
    fn next_space_annotation(&mut self, offset: usize) -> LxSpaceAnnotation {
        next_annotation_aux(
            offset,
            self.space_annotations,
            &mut self.next_space_annotation_index,
        )
    }
}

#[track_caller]
fn next_annotation_aux<A: Copy + Default>(
    offset: usize,
    token_annotations: &[(usize, A)],
    next_token_annotation_index: &mut usize,
) -> A {
    if *next_token_annotation_index >= token_annotations.len() {
        return A::default();
    }
    let offset1 = token_annotations[*next_token_annotation_index].0;
    if offset1 > offset {
        return A::default();
    } else if offset1 == offset {
        let result = token_annotations[*next_token_annotation_index].1;
        *next_token_annotation_index += 1;
        result
    } else {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::annotation::token::{LxIntegralAnnotation, LxTokenAnnotation, LxVariableAnnotation};
    use crate::{
        annotation::space::{LxApplyAnnotation, LxSpaceAnnotation},
        annotations::LxAnnotations,
    };

    fn setup_test_data() -> LxAnnotations {
        let input = "\\int xdx".to_string();

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
                "\\int ",
                LxSpaceAnnotation::Apply(LxApplyAnnotation::Integration),
            ),
            (
                "x",
                LxSpaceAnnotation::Apply(LxApplyAnnotation::ScalarDifferentialFormMul),
            ),
            (
                "d",
                LxSpaceAnnotation::Apply(LxApplyAnnotation::Differentiation),
            ),
        ];

        LxAnnotations::from_sparse(
            &input,
            token_annotations.into_iter(),
            space_annotations.into_iter(),
        )
    }

    #[test]
    fn test_walker_next_token_and_space_annotations() {
        let annotations = setup_test_data();
        let mut walker = annotations.walker();

        // Test at offset 0 ("\\int")
        assert_eq!(
            walker.next(0, 4),
            (
                LxTokenAnnotation::Integral(
                    LxIntegralAnnotation::SingleVariableIndefiniteIntegralOverReal
                ),
                LxSpaceAnnotation::default()
            )
        );

        // Test at offset 5 ("x" after space)
        assert_eq!(
            walker.next(5, 6),
            (
                LxTokenAnnotation::Variable(LxVariableAnnotation::Usage),
                LxSpaceAnnotation::Apply(LxApplyAnnotation::Integration)
            )
        );

        // Test at offset 6 ("d" after "x")
        assert_eq!(
            walker.next(6, 7),
            (
                LxTokenAnnotation::Differential,
                LxSpaceAnnotation::Apply(LxApplyAnnotation::ScalarDifferentialFormMul)
            )
        );

        // Test at offset 7 ("x" after "d")
        assert_eq!(
            walker.next(7, 8),
            (
                LxTokenAnnotation::Variable(
                    LxVariableAnnotation::SingleVariableIntegralVariableDecl
                ),
                LxSpaceAnnotation::Apply(LxApplyAnnotation::Differentiation)
            )
        );

        // Test at offset 8 (end of input)
        assert_eq!(
            walker.next(8, 9),
            (LxTokenAnnotation::default(), LxSpaceAnnotation::default())
        );
    }

    // Additional tests for next_token_annotation and next_space_annotation can be added here
}
