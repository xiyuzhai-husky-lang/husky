use super::*;
use crate::{
    expr::{LnHirExprArena, LnHirExprArenaRef, LnHirExprData, LnHirExprIdx},
    item_defn::{LnItemDefnArenaRef, LnItemDefnData, LnItemDefnIdx, LnItemDefnIdxRange},
    stmt::{LnHirStmtArena, LnHirStmtArenaRef},
    tactic::{LnHirTacticArena, LnHirTacticArenaRef},
};
use husky_tree_utils::display::DisplayTree;

pub struct LnHirExprDisplayTreeBuilder<'a> {
    db: &'a ::salsa::Db,
    expr_arena: LnHirExprArenaRef<'a>,
    stmt_arena: LnHirStmtArenaRef<'a>,
    tactic_arena: LnHirTacticArenaRef<'a>,
    defn_arena: LnItemDefnArenaRef<'a>,
}

impl<'a> LnHirExprDisplayTreeBuilder<'a> {
    pub fn new(
        db: &'a ::salsa::Db,
        expr_arena: LnHirExprArenaRef<'a>,
        stmt_arena: LnHirStmtArenaRef<'a>,
        tactic_arena: LnHirTacticArenaRef<'a>,
        defn_arena: LnItemDefnArenaRef<'a>,
    ) -> Self {
        Self {
            db,
            expr_arena,
            stmt_arena,
            tactic_arena,
            defn_arena,
        }
    }
}

impl<'a> LnHirExprDisplayTreeBuilder<'a> {
    pub fn render_expr(&self, expr: LnHirExprIdx) -> DisplayTree {
        let db = self.db;
        let value = match self.expr_arena[expr] {
            LnHirExprData::Literal(literal) => format!("literal: `{}`", literal.data(db)),
            LnHirExprData::ItemPath(item_path) => format!("item path: `{}`", item_path.show(db)),
            LnHirExprData::Variable { ident } => todo!(),
            LnHirExprData::Prefix { opr, opd } => todo!(),
            LnHirExprData::Suffix { opd, opr } => todo!(),
            LnHirExprData::Binary { lopd, opr, ropd } => format!("binary: `{}`", opr),
            LnHirExprData::Lambda {
                ref parameters,
                body,
            } => todo!(),
            LnHirExprData::Application {
                function_and_arguments,
            } => todo!(),
            LnHirExprData::Sorry => "sorry".to_string(),
        };
        let children = self.expr_arena[expr].children();
        DisplayTree::new(
            value,
            children.into_iter().map(|c| self.render_expr(c)).collect(),
        )
    }

    pub fn render_defns_together(&self, defns: LnItemDefnIdxRange) -> DisplayTree {
        let db = self.db;
        let children = self.render_defns(defns);
        DisplayTree::new("defns".to_string(), children)
    }

    pub fn render_defns(&self, defns: LnItemDefnIdxRange) -> Vec<DisplayTree> {
        defns
            .into_iter()
            .map(|defn| self.render_defn(defn))
            .collect()
    }

    pub fn render_defn(&self, defn: LnItemDefnIdx) -> DisplayTree {
        let db = self.db;
        let defn_data = &self.defn_arena[defn];
        let value = match defn_data {
            LnItemDefnData::Variable { symbol, ty } => format!("variable: `{}`", symbol.data(db)),
            LnItemDefnData::Group { defns, ref meta } => format!("group: `{}`", meta),
        };
        let children = defn_data.children();
        DisplayTree::new(
            value,
            children.into_iter().map(|c| self.render_defn(c)).collect(),
        )
    }
}
