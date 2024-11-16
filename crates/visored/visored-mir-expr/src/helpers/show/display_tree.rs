use husky_tree_utils::display::DisplayTree;

use crate::{
    expr::{application::VdMirFunc, VdMirExprArenaRef, VdMirExprData, VdMirExprIdx},
    stmt::VdMirStmtArenaRef,
};

pub struct VdMirExprDisplayTreeBuilder<'a> {
    db: &'a salsa::Db,
    expr_arena: VdMirExprArenaRef<'a>,
    stmt_arena: VdMirStmtArenaRef<'a>,
}

impl<'a> VdMirExprDisplayTreeBuilder<'a> {
    pub fn new(
        db: &'a salsa::Db,
        expr_arena: VdMirExprArenaRef<'a>,
        stmt_arena: VdMirStmtArenaRef<'a>,
    ) -> Self {
        Self {
            db,
            expr_arena,
            stmt_arena,
        }
    }
}

impl<'a> VdMirExprDisplayTreeBuilder<'a> {
    pub fn render_expr(&self, expr: VdMirExprIdx) -> DisplayTree {
        let db = self.db;
        let (value, children) = match self.expr_arena[expr] {
            VdMirExprData::Literal(literal) => (literal.data(db).as_str().to_string(), vec![]),
            VdMirExprData::Variable(ref variable) => ("variable".to_string(), vec![]),
            VdMirExprData::Application {
                function,
                arguments,
            } => {
                let value = match function {
                    VdMirFunc::NormalBasePrefixOpr(signature) => {
                        format!("prefix opr")
                    }
                    VdMirFunc::NormalBaseSeparator { .. } => "separator".to_string(),
                    VdMirFunc::NormalBaseBinaryOpr { .. } => "binary opr".to_string(),
                    VdMirFunc::InSet => "in set".to_string(),
                    VdMirFunc::Power(vd_power_signature) => {
                        format!("power")
                    }
                    VdMirFunc::NormalBaseSqrt(signature) => {
                        format!("sqrt")
                    }
                    VdMirFunc::NormalBaseFrac(signature) => {
                        format!("frac")
                    }
                };
                (
                    value,
                    arguments
                        .into_iter()
                        .map(|arg| self.render_expr(arg))
                        .collect(),
                )
            }
            VdMirExprData::ItemPath(item_path) => todo!(),
        };
        DisplayTree::new(value, children)
    }
}
