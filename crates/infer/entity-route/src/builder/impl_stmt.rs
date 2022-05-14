use ast::{RawBoundary, RawLoopKind};
use text::TextRanged;

use super::*;

impl<'a> EntityRouteSheetBuilder<'a> {
    pub(super) fn infer_stmts(
        &mut self,
        ast_iter: AstIter,
        opt_output_ty: Option<EntityRoutePtr>,
        arena: &RawExprArena,
    ) {
        self.enter_block();
        for item in ast_iter {
            if let Ok(ref value) = item.value {
                match value.kind {
                    AstKind::Stmt(ref stmt) => match stmt.variant {
                        RawStmtVariant::Match { match_expr, .. } => {
                            let opt_match_expr_ty =
                                self.infer_expr(match_expr, opt_output_ty, arena);
                            if let Some(children) = item.opt_children {
                                self.infer_match_branches(
                                    arena,
                                    children,
                                    opt_output_ty,
                                    opt_match_expr_ty,
                                )
                            }
                        }
                        _ => {
                            self.infer_stmt(stmt, opt_output_ty, arena);
                            if let Some(children) = item.opt_children {
                                self.infer_stmts(children, opt_output_ty, arena)
                            }
                        }
                    },
                    _ => todo!(),
                }
            }
        }
        self.exit_block()
    }

    fn infer_stmt(
        &mut self,
        stmt: &RawStmt,
        opt_output_ty: Option<EntityRoutePtr>,
        arena: &RawExprArena,
    ) {
        match stmt.variant {
            RawStmtVariant::Loop(raw_loop_kind) => match raw_loop_kind {
                RawLoopKind::For {
                    frame_var,
                    initial_boundary,
                    final_boundary,
                    ..
                } => {
                    should!(self
                        .entity_route_sheet
                        .variable_tys
                        .insert((frame_var.ident, stmt.row()), RootIdentifier::I32.into())
                        .is_none());
                    self.infer_loop_bound(initial_boundary, arena);
                    self.infer_loop_bound(final_boundary, arena);
                }
                RawLoopKind::ForExt { final_boundary, .. } => {
                    self.infer_loop_bound(final_boundary, arena)
                }
                RawLoopKind::While { condition } => self.infer_condition(condition, arena),
                RawLoopKind::DoWhile { condition } => self.infer_condition(condition, arena),
            },
            RawStmtVariant::Branch(ref branch_kind) => match branch_kind {
                RawBranchVariant::If { condition } => self.infer_condition(*condition, arena),
                RawBranchVariant::Elif { condition } => self.infer_condition(*condition, arena),
                RawBranchVariant::Else => (),
                RawBranchVariant::Case { pattern } => todo!(),
                RawBranchVariant::Default => todo!(),
            },
            RawStmtVariant::Exec(expr) => {
                self.infer_expr(expr, Some(RootIdentifier::Void.into()), arena);
            }
            RawStmtVariant::Init {
                varname,
                initial_value,
                ..
            } => {
                if let Some(ty) = self.infer_expr(initial_value, None, arena) {
                    should!(self
                        .entity_route_sheet
                        .variable_tys
                        .insert((varname.ident, stmt.row()), ty)
                        .is_none())
                }
            }
            RawStmtVariant::Return(result) => {
                self.infer_expr(result, opt_output_ty, arena);
            }
            RawStmtVariant::Assert(condition) => self.infer_condition(condition, arena),
            RawStmtVariant::Break => emsg_once!("ensure break is inside a loop"),
            RawStmtVariant::Match { match_expr, .. } => panic!("shouldn't be here"),
        }
    }

    fn infer_match_branches(
        &mut self,
        arena: &RawExprArena,
        branch_ast_iter: AstIter,
        opt_output_ty: Option<EntityRoutePtr>,
        opt_match_expr_ty: Option<EntityRoutePtr>,
    ) {
        for item in branch_ast_iter {
            if let Ok(ref ast) = item.value.as_ref() {
                match ast.kind {
                    AstKind::Stmt(RawStmt {
                        variant: RawStmtVariant::Branch(RawBranchVariant::Case { ref pattern }),
                        ..
                    }) => {
                        if let Some(match_expr_ty) = opt_match_expr_ty {
                            if match_expr_ty != pattern.ty {
                                todo!()
                            }
                        }
                    }
                    AstKind::Stmt(RawStmt {
                        variant: RawStmtVariant::Branch(RawBranchVariant::Default),
                        ..
                    }) => (),
                    _ => {
                        p!(ast.kind);
                        panic!()
                    }
                }
            }
            if let Some(children) = item.opt_children {
                self.infer_stmts(children, opt_output_ty, arena)
            }
        }
    }

    fn infer_loop_bound(&mut self, boundary: RawBoundary, arena: &RawExprArena) {
        if let Some(bound) = boundary.opt_bound {
            self.infer_expr(bound, Some(RootIdentifier::I32.into()), arena);
        }
    }

    fn infer_condition(&mut self, condition: RawExprIdx, arena: &RawExprArena) {
        self.infer_expr(condition, Some(RootIdentifier::Bool.into()), arena);
    }
}
