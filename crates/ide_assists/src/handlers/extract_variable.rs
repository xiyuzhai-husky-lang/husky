use stdx::format_to;
use syntax::{ast, NodeOrToken, SyntaxNode};

use crate::{utils::suggest_name, AssistContext, AssistId, AssistKind, Assists};

// Assist: extract_variable
//
// Extracts subexpression into a variable.
//
// ```
// fn main() {
//     $0(1 + 2)$0 * 4;
// }
// ```
// ->
// ```
// fn main() {
//     let $0var_name = (1 + 2);
//     var_name * 4;
// }
// ```
pub(crate) fn extract_variable(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

/// Check whether the node is a valid expression which can be extracted to a variable.
/// In general that's true for any expression, but in some cases that would produce invalid code.
fn valid_target_expr(node: SyntaxNode) -> Option<ast::Expr> {
    todo!()
}

#[derive(Debug)]
enum Anchor {
    Before(SyntaxNode),
    Replace(ast::ExprStmt),
    WrapInBlock(SyntaxNode),
}

impl Anchor {
    fn from(to_extract: &ast::Expr) -> Option<Anchor> {
        todo!()
    }

    fn syntax(&self) -> &SyntaxNode {
        todo!()
    }
}
