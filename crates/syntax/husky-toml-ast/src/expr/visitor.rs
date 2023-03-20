use super::*;

impl TomlAst for TomlExpr {
    type Visitor<'a> = TomlExprVisitor<'a>;
}

pub struct TomlExprVisitor<'a> {
    toml_ast_sheet: &'a TomlAstSheet,
    expr_idx: TomlExprIdx,
    expr: &'a TomlExpr,
}

impl<'a> TomlExprVisitor<'a> {
    pub(crate) fn new(toml_ast_sheet: &'a TomlAstSheet, expr_idx: TomlExprIdx) -> Self {
        Self {
            toml_ast_sheet,
            expr_idx,
            expr: &toml_ast_sheet.expr_arena[expr_idx],
        }
    }

    pub(crate) fn toml_ast_sheet(&self) -> &'a TomlAstSheet {
        self.toml_ast_sheet
    }

    pub(crate) fn expr_idx(&self) -> ArenaIdx<TomlExpr> {
        self.expr_idx
    }

    pub fn expr(&self) -> &TomlExpr {
        self.expr
    }
}
