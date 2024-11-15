use super::*;
use crate::{
    expr::{LnMirExprArena, LnMirExprArenaRef, LnMirExprData, LnMirExprIdx},
    item_defn::{LnItemDefnArenaRef, LnItemDefnData, LnItemDefnIdx, LnItemDefnIdxRange},
    stmt::{LnMirStmtArena, LnMirStmtArenaRef},
    tactic::{LnMirTacticArena, LnMirTacticArenaRef},
};
use husky_tree_utils::display::DisplayTree;

pub struct LnMirExprDisplayTreeBuilder<'a> {
    db: &'a ::salsa::Db,
    expr_arena: LnMirExprArenaRef<'a>,
    stmt_arena: LnMirStmtArenaRef<'a>,
    tactic_arena: LnMirTacticArenaRef<'a>,
    defn_arena: LnItemDefnArenaRef<'a>,
}

impl<'a> LnMirExprDisplayTreeBuilder<'a> {
    pub fn new(
        db: &'a ::salsa::Db,
        expr_arena: LnMirExprArenaRef<'a>,
        stmt_arena: LnMirStmtArenaRef<'a>,
        tactic_arena: LnMirTacticArenaRef<'a>,
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

impl<'a> LnMirExprDisplayTreeBuilder<'a> {
    pub fn render_expr(&self, expr: LnMirExprIdx) -> DisplayTree {
        let db = self.db;
        let value = match self.expr_arena[expr] {
            LnMirExprData::Literal(literal) => format!("literal: `{}`", literal.data(db)),
            LnMirExprData::ItemPath(item_path) => format!("item path: `{}`", item_path.show(db)),
            LnMirExprData::Variable { ident } => todo!(),
            LnMirExprData::Lambda {
                ref parameters,
                body,
            } => todo!(),
            LnMirExprData::Application {
                function,
                arguments,
            } => format!("application"),
            LnMirExprData::Sorry => "sorry".to_string(),
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
