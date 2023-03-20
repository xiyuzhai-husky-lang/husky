use super::*;

impl TomlAst for TomlExpr {
    type Visitor<'a> = TomlExprVisitor<'a>;
}

pub struct TomlExprVisitor<'a> {
    toml_ast_sheet: &'a TomlAstSheet,
}
