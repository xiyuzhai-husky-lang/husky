use super::*;
use crate::annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation};

pub struct VdAnnotationsWalker<'a> {
    token_annotations: &'a [VdTokenAnnotationEntry],
    space_annotations: &'a [VdSpaceAnnotationEntry],
    next_token_annotation_index: usize,
    next_space_annotation_index: usize,
}

impl<'a> VdAnnotationsWalker<'a> {
    pub fn new(
        token_annotations: &'a [VdTokenAnnotationEntry],
        space_annotations: &'a [VdSpaceAnnotationEntry],
    ) -> Self {
        Self {
            token_annotations,
            space_annotations,
            next_token_annotation_index: 0,
            next_space_annotation_index: 0,
        }
    }
}

impl<'a> VdAnnotationsWalker<'a> {
    pub fn next(&mut self, start: usize, end: usize) -> (VdTokenAnnotation, VdSpaceAnnotation) {
        (
            self.next_token_annotation(start, end),
            self.next_space_annotation(start, end),
        )
    }

    fn next_token_annotation(&mut self, start: usize, end: usize) -> VdTokenAnnotation {
        next_annotation_aux(
            start,
            end,
            self.token_annotations,
            &mut self.next_token_annotation_index,
        )
    }

    fn next_space_annotation(&mut self, start: usize, end: usize) -> VdSpaceAnnotation {
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
    token_annotations: &[VdAnnotationEntry<A>],
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
    use latex_ast::ast::{parse_latex_input_into_asts, LxAstArena};
    use latex_prelude::mode::LxMode;

    use super::*;
    use crate::annotation::token::{LxIntegralAnnotation, LxVariableAnnotation, VdTokenAnnotation};
    use crate::{
        annotation::space::{LxApplyAnnotation, VdSpaceAnnotation},
        annotations::VdAnnotations,
    };

    fn setup_test_data() -> VdAnnotations {
        let db = DB::default();

        let input = "\\int xdx".to_string();

        let token_annotations = vec![
            (
                ("", "\\int"),
                VdTokenAnnotation::Integral(
                    LxIntegralAnnotation::SingleVariableIndefiniteIntegralOverReal,
                ),
            ),
            (
                ("\\int ", "x"),
                VdTokenAnnotation::Variable(LxVariableAnnotation::Usage),
            ),
            (("\\int x", "d"), VdTokenAnnotation::Differential),
            (
                ("\\int xd", "x"),
                VdTokenAnnotation::Variable(
                    LxVariableAnnotation::SingleVariableIntegralVariableDecl,
                ),
            ),
        ];

        let space_annotations = vec![(
            ("\\int x", "d"),
            VdSpaceAnnotation::Apply(LxApplyAnnotation::ScalarDifferentialFormMul),
        )];

        let mut token_storage = LxTokenStorage::default();
        let mut ast_arena = LxAstArena::default();
        let asts = parse_latex_input_into_asts(
            &db,
            &input,
            LxMode::Math,
            &mut token_storage,
            &mut ast_arena,
        );
        VdAnnotations::from_sparse(
            &input,
            token_annotations.into_iter(),
            space_annotations.into_iter(),
            &token_storage,
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
                VdTokenAnnotation::Integral(
                    LxIntegralAnnotation::SingleVariableIndefiniteIntegralOverReal
                ),
                VdSpaceAnnotation::default()
            )
        );

        // Test at offset 5 ("x" after "\\int ")
        assert_eq!(
            walker.next(5, 6),
            (
                VdTokenAnnotation::Variable(LxVariableAnnotation::Usage),
                VdSpaceAnnotation::Void
            )
        );

        // Test at offset 6 ("d" after "x")
        assert_eq!(
            walker.next(6, 7),
            (
                VdTokenAnnotation::Differential,
                VdSpaceAnnotation::Apply(LxApplyAnnotation::ScalarDifferentialFormMul)
            )
        );
    }

    // Additional tests for next_token_annotation and next_space_annotation can be added here
}
