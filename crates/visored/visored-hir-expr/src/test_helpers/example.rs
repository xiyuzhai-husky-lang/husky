use crate::{
    builder::VdHirExprBuilder,
    expr::{VdHirExprArena, VdHirExprIdx},
    stmt::{VdHirStmtArena, VdHirStmtIdxRange},
    *,
};
use either::*;
use expr::{application::VdHirApplicationFunction, VdHirExprData};
use husky_tree_utils::display::DisplayTree;
use latex_prelude::mode::LxMode;
use visored_annotation::annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation};
use visored_sem_expr::test_helpers::example::VdSemExprExample;

pub struct VdHirExprExample<'db> {
    db: &'db salsa::Db,
    expr_arena: VdHirExprArena,
    stmt_arena: VdHirStmtArena,
    result: Either<VdHirExprIdx, VdHirStmtIdxRange>,
}

impl<'db> VdHirExprExample<'db> {
    pub fn new(
        input: &str,
        root_mode: LxMode,
        token_annotations: &[((&str, &str), VdTokenAnnotation)],
        space_annotations: &[((&str, &str), VdSpaceAnnotation)],
        db: &'db salsa::Db,
    ) -> Self {
        let VdSemExprExample {
            input,
            root_mode,
            annotations,
            default_resolution_table,
            token_storage,
            ast_arena,
            asts,
            ast_token_idx_range_map,
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
            stmt_arena,
            division_arena,
            expr_range_map,
            phrase_range_map,
            clause_range_map,
            sentence_range_map,
            stmt_range_map,
            division_range_map,
            result,
        } = VdSemExprExample::new(input, root_mode, token_annotations, space_annotations, db);
        let mut builder = VdHirExprBuilder::new(
            db,
            expr_arena.as_arena_ref(),
            phrase_arena.as_arena_ref(),
            clause_arena.as_arena_ref(),
            sentence_arena.as_arena_ref(),
            stmt_arena.as_arena_ref(),
            division_arena.as_arena_ref(),
        );
        let result = result.to_vd_hir(&mut builder);
        let (expr_arena, stmt_arena) = builder.finish();
        Self {
            db,
            expr_arena,
            stmt_arena,
            result,
        }
    }

    pub(crate) fn show_display_tree(&self) -> String {
        self.display_tree().show(&Default::default())
    }

    fn display_tree(&self) -> DisplayTree {
        match self.result {
            Left(expr) => self.render_expr(expr),
            Right(_) => todo!(),
        }
    }

    fn render_expr(&self, expr: VdHirExprIdx) -> DisplayTree {
        let db = self.db;
        match self.expr_arena[expr] {
            VdHirExprData::Literal(literal) => {
                DisplayTree::new(literal.data(db).as_str().to_string(), vec![])
            }
            VdHirExprData::Variable(ref variable) => todo!(),
            VdHirExprData::Application {
                function,
                arguments,
            } => {
                let value = match function {
                    VdHirApplicationFunction::IntAdd => "separator list int add".to_string(),
                    VdHirApplicationFunction::TrivialEq => "separator list eq".to_string(),
                };
                DisplayTree::new(
                    value,
                    arguments
                        .into_iter()
                        .map(|arg| self.render_expr(arg))
                        .collect(),
                )
            }
        }
    }
}
