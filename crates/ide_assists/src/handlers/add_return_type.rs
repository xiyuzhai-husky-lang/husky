use hir::HirDisplay;
use syntax::{ast, TextRange, TextSize};

use crate::{AssistContext, AssistId, AssistKind, Assists};

// Assist: add_return_type
//
// Adds the return type to a function or closure inferred from its tail expression if it doesn't have a return
// type specified. This assists is useable in a functions or closures tail expression or return type position.
//
// ```
// fn foo() { 4$02i32 }
// ```
// ->
// ```
// fn foo() -> i32 { 42i32 }
// ```
pub(crate) fn add_return_type(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

enum InsertOrReplace {
    Insert(TextSize),
    Replace(TextRange),
}

/// Check the potentially already specified return type and reject it or turn it into a builder command
/// if allowed.
fn ret_ty_to_action(ret_ty: Option<ast::RetType>, insert_pos: TextSize) -> Option<InsertOrReplace> {
    todo!()
}

enum FnType {
    Function,
    Closure { wrap_expr: bool },
}

fn extract_tail(ctx: &AssistContext) -> Option<(FnType, ast::Expr, InsertOrReplace)> {
    todo!()
}
