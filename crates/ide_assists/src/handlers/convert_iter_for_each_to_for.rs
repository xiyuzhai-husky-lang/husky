use ide_db::helpers::FamousDefs;
use stdx::format_to;
use syntax::ast;

use crate::{AssistContext, AssistId, AssistKind, Assists};

// Assist: convert_iter_for_each_to_for
//
// Converts an Iterator::for_each function into a for loop.
//
// ```
// # //- minicore: iterators
// # use core::iter;
// fn main() {
//     let iter = iter::repeat((9, 2));
//     iter.for_each$0(|(x, y)| {
//         println!("x: {}, y: {}", x, y);
//     });
// }
// ```
// ->
// ```
// # use core::iter;
// fn main() {
//     let iter = iter::repeat((9, 2));
//     for (x, y) in iter {
//         println!("x: {}, y: {}", x, y);
//     }
// }
// ```
pub(crate) fn convert_iter_for_each_to_for(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

// Assist: convert_for_loop_with_for_each
//
// Converts a for loop into a for_each loop on the Iterator.
//
// ```
// fn main() {
//     let x = vec![1, 2, 3];
//     for$0 v in x {
//         let y = v * 2;
//     }
// }
// ```
// ->
// ```
// fn main() {
//     let x = vec![1, 2, 3];
//     x.into_iter().for_each(|v| {
//         let y = v * 2;
//     });
// }
// ```
pub(crate) fn convert_for_loop_with_for_each(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

/// If iterable is a reference where the expression behind the reference implements a method
/// returning an Iterator called iter or iter_mut (depending on the type of reference) then return
/// the expression behind the reference and the method name
fn is_ref_and_impls_iter_method(
    sema: &hir::Semantics<ide_db::RootDatabase>,
    iterable: &ast::Expr,
) -> Option<(ast::Expr, hir::Name)> {
    todo!()
}

/// Whether iterable implements core::Iterator
fn impls_core_iter(sema: &hir::Semantics<ide_db::RootDatabase>, iterable: &ast::Expr) -> bool {
    todo!()
}

fn validate_method_call_expr(
    ctx: &AssistContext,
    expr: ast::MethodCallExpr,
) -> Option<(ast::Expr, ast::Expr)> {
    todo!()
}
