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
        ast::AstKind::TypeDef {
            ident,
            kind,
            generics,
        } => todo!(),
        ast::AstKind::MainDef => todo!(),
        ast::AstKind::DatasetConfig => todo!(),
        ast::AstKind::FuncDef { kind, decl } => todo!(),
        ast::AstKind::PatternDef => todo!(),
        ast::AstKind::Use { ident, scope } => todo!(),
        ast::AstKind::MembDef { ident, kind } => todo!(),
        ast::AstKind::Stmt(_) => todo!(),
    }
}
