pub fn entity_from_ast(
    ast_text: &ast::AstText,
    token_group_index: usize,
) -> SemanticResultArc<Entity> {
    let FoldIterItem {
        value, children, ..
    } = ast_text
        .folded_results
        .fold_iter(token_group_index)
        .next()
        .unwrap();
    let head = value.as_ref()?;
    match head {
        ast::Ast::TypeDef {
            ident,
            kind,
            generics,
        } => todo!(),
        ast::Ast::MainDef => todo!(),
        ast::Ast::DatasetConfig => todo!(),
        ast::Ast::FuncDef { kind, decl } => todo!(),
        ast::Ast::PatternDef => todo!(),
        ast::Ast::Use { ident, scope } => todo!(),
        ast::Ast::MembDef { ident, kind } => todo!(),
        ast::Ast::Stmt(_) => todo!(),
    }
}
