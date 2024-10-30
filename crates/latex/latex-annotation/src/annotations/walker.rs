use super::{LxAnnotationEntry, LxSpaceAnnotationEntry, LxTokenAnnotationEntry};
use crate::annotation::{space::LxSpaceAnnotation, token::LxTokenAnnotation};

pub struct LxAnnotationsWalker<'a> {
    token_annotations: &'a [LxTokenAnnotationEntry],
    space_annotations: &'a [LxSpaceAnnotationEntry],
    next_token_annotation_index: usize,
    next_space_annotation_index: usize,
}

impl<'a> LxAnnotationsWalker<'a> {
    pub fn new(
        token_annotations: &'a [LxTokenAnnotationEntry],
        space_annotations: &'a [LxSpaceAnnotationEntry],
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
    pub fn next(&mut self, start: usize, end: usize) -> (LxTokenAnnotation, LxSpaceAnnotation) {
        (
            self.next_token_annotation(start, end),
            self.next_space_annotation(start, end),
        )
    }

    fn next_token_annotation(&mut self, start: usize, end: usize) -> LxTokenAnnotation {
        next_annotation_aux(
            start,
            end,
            self.token_annotations,
            &mut self.next_token_annotation_index,
        )
    }

    fn next_space_annotation(&mut self, start: usize, end: usize) -> LxSpaceAnnotation {
        next_annotation_aux(
            start,
            end,
            self.space_annotations,
            &mut self.next_space_annotation_index,
        )
    }
}

fn next_annotation_aux<A: Copy + Default + std::fmt::Debug>(
    start: usize,
    end: usize,
    token_annotations: &[LxAnnotationEntry<A>],
    next_token_annotation_index: &mut usize,
) -> A {
    if *next_token_annotation_index >= token_annotations.len() {
        return A::default();
    }
    let start1 = token_annotations[*next_token_annotation_index].start;
    if start1 > start {
        return A::default();
    } else if start1 == start {
        use husky_print_utils::p;
        p!(token_annotations[*next_token_annotation_index]);
        assert_eq!(end, token_annotations[*next_token_annotation_index].end);
        let result = token_annotations[*next_token_annotation_index].annotation;
        *next_token_annotation_index += 1;
        result
    } else {
        panic!("Invalid annotation sequence: annotation start ({start1}) is before token start ({start})");
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
                ("", "\\int"),
                LxTokenAnnotation::Integral(
                    LxIntegralAnnotation::SingleVariableIndefiniteIntegralOverReal,
                ),
            ),
            (
                ("\\int ", "x"),
                LxTokenAnnotation::Variable(LxVariableAnnotation::Usage),
            ),
            (("\\int x", "d"), LxTokenAnnotation::Differential),
            (
                ("\\int xd", "x"),
                LxTokenAnnotation::Variable(
                    LxVariableAnnotation::SingleVariableIntegralVariableDecl,
                ),
            ),
        ];

        let space_annotations = vec![(
            ("\\int x", "d"),
            LxSpaceAnnotation::Apply(LxApplyAnnotation::ScalarDifferentialFormMul),
        )];

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

        // Test at offset 5 ("x" after "\\int ")
        assert_eq!(
            walker.next(5, 6),
            (
                LxTokenAnnotation::Variable(LxVariableAnnotation::Usage),
                LxSpaceAnnotation::Void
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
    }

    // Additional tests for next_token_annotation and next_space_annotation can be added here
}
