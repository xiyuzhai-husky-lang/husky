//! Various helper functions to work with SyntaxNodes.
use syntax::ast::{self, PathSegmentKind, VisibilityKind};

pub fn expr_as_name_ref(expr: &ast::Expr) -> Option<ast::NameRef> {
    todo!()
}

pub fn block_as_lone_tail(block: &ast::BlockExpr) -> Option<ast::Expr> {
    todo!()
}

/// Preorder walk all the expression's child expressions.
pub fn walk_expr(expr: &ast::Expr, cb: &mut dyn FnMut(ast::Expr)) {
    todo!()
}

/// Preorder walk all the expression's child expressions preserving events.
/// If the callback returns true on an [`WalkEvent::Enter`], the subtree of the expression will be skipped.
/// Note that the subtree may already be skipped due to the context analysis this function does.
// pub fn preorder_expr(start: &ast::Expr, cb: &mut dyn FnMut(WalkEvent<ast::Expr>) -> bool) {
//     todo!()
// }

/// Preorder walk all the expression's child patterns.
pub fn walk_patterns_in_expr(start: &ast::Expr, cb: &mut dyn FnMut(ast::Pat)) {
    todo!()
}

/// Preorder walk all the pattern's sub patterns.
pub fn walk_pat(pat: &ast::Pat, cb: &mut dyn FnMut(ast::Pat)) {
    todo!()
}

/// Preorder walk all the type's sub types.
pub fn walk_ty(ty: &ast::Type, cb: &mut dyn FnMut(ast::Type)) {
    todo!()
}

pub fn vis_eq(this: &ast::Visibility, other: &ast::Visibility) -> bool {
    todo!()
}
