use super::*;
use crate::builder::VdLeanTranspilationBuilder;
use either::*;
use latex_prelude::mode::LxMode;
use visored_annotation::annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation};
use visored_hir_expr::test_helpers::example::VdHirExprExample;

pub struct VdLeanTranspilationExample {}

impl VdLeanTranspilationExample {
    pub fn new(
        input: &str,
        root_mode: LxMode,
        token_annotations: &[((&str, &str), VdTokenAnnotation)],
        space_annotations: &[((&str, &str), VdSpaceAnnotation)],
        db: &::salsa::Db,
    ) -> Self {
        let VdHirExprExample {
            expr_arena: vd_hir_expr_arena,
            stmt_arena: vd_hir_stmt_arena,
            result,
        } = VdHirExprExample::new(input, LxMode::Math, &[], &[], db);
        let mut builder = VdLeanTranspilationBuilder::new(
            db,
            vd_hir_expr_arena.as_arena_ref(),
            vd_hir_stmt_arena.as_arena_ref(),
        );
        let result = match result {
            Left(expr) => expr.to_lean(&mut builder),
            Right(stmts) => todo!(),
        };
        let _ = builder.finish();
        Self {}
    }
}
