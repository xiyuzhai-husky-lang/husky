use super::*;

pub(super) fn allocate_asts(
    asts: Seq<Option<AstData>>,
    new_asts: Seq<Option<AstData>>,
) -> Seq<Option<AstData>> {
    allocate_ast.apply(asts, new_asts)
}

fn allocate_ast(ast: Option<AstData>, new_ast: Option<AstData>) -> Option<AstData> {
    ast.or(new_ast)
}
