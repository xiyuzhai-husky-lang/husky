use ast::{RawBoundary, RawLoopKind};
use map_utils::insert_new;
use text::TextRanged;

use super::*;

impl<'a> TySheetBuilder<'a> {
    pub(super) fn infer_stmts(
        &mut self,
        ast_iter: AstIter,
        opt_output_ty: Option<EntityRoutePtr>,
        arena: &RawExprArena,
    ) {
        self.enter_block();
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.kind {
                    AstKind::Stmt(ref stmt) => self.infer_stmt(stmt, opt_output_ty, arena),
                    _ => (),
                }
            }
            if let Some(children) = item.children {
                self.infer_stmts(children, opt_output_ty, arena)
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
        match stmt.kind {
            RawStmtVariant::Loop(raw_loop_kind) => match raw_loop_kind {
                RawLoopKind::For {
                    frame_var,
                    initial_boundary,
                    final_boundary,
                    ..
                } => {
                    should!(self
                        .ty_sheet
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
            RawStmtVariant::Branch(branch_kind) => match branch_kind {
                RawBranchKind::If { condition } => self.infer_condition(condition, arena),
                RawBranchKind::Elif { condition } => self.infer_condition(condition, arena),
                RawBranchKind::Else => (),
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
                    insert_new!(self.ty_sheet.variable_tys, (varname.ident, stmt.row()), ty)
                }
            }
            RawStmtVariant::Return(result) => {
                self.infer_expr(result, opt_output_ty, arena);
            }
            RawStmtVariant::Assert(condition) => self.infer_condition(condition, arena),
            RawStmtVariant::Break => msg_once!("ensure break is inside a loop"),
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
