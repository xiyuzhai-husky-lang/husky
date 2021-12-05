use syntax::ast::{self, BinExpr};

use crate::{AssistContext, AssistId, AssistKind, Assists};

// Assist: flip_binexpr
//
// Flips operands of a binary expression.
//
// ```
// fn main() {
//     let _ = 90 +$0 2;
// }
// ```
// ->
// ```
// fn main() {
//     let _ = 2 + 90;
// }
// ```
pub(crate) fn flip_binexpr(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

enum FlipAction {
    // Flip the expression
    Flip,
    // Flip the expression and replace the operator with this string
    FlipAndReplaceOp(&'static str),
    // Do not flip the expression
    DontFlip,
}

impl From<ast::BinaryOp> for FlipAction {
    fn from(op_kind: ast::BinaryOp) -> Self {
        todo!()
    }
}
