use husky_tree_utils::display::DisplayTree;

use crate::{
    expr::{application::VdHirApplicationFunction, VdHirExprArenaRef, VdHirExprData, VdHirExprIdx},
    stmt::VdHirStmtArenaRef,
};

pub struct VdHirExprDisplayTreeBuilder<'a> {
    db: &'a salsa::Db,
    expr_arena: VdHirExprArenaRef<'a>,
    stmt_arena: VdHirStmtArenaRef<'a>,
}

impl<'a> VdHirExprDisplayTreeBuilder<'a> {
    pub fn new(
        db: &'a salsa::Db,
        expr_arena: VdHirExprArenaRef<'a>,
        stmt_arena: VdHirStmtArenaRef<'a>,
    ) -> Self {
        Self {
            db,
            expr_arena,
            stmt_arena,
        }
    }
}

impl<'a> VdHirExprDisplayTreeBuilder<'a> {
    pub fn render_expr(&self, expr: VdHirExprIdx) -> DisplayTree {
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
