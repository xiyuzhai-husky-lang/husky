use ast::{RawBoundary, RawLoopKind};
use common::{insert_new, p};
use scope::RangedScope;
use text::TextRanged;

use super::*;

impl<'a> TySheetBuilder<'a> {
    pub(super) fn infer_stmts(
        &mut self,
        ast_iter: AstIter,
        output_ty: ScopePtr,
        arena: &RawExprArena,
    ) {
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.kind {
                    AstKind::Stmt(ref stmt) => self.infer_stmt(stmt, output_ty, arena),
                    _ => (),
                }
            }
            if let Some(children) = item.children {
                self.infer_stmts(children, output_ty, arena)
            }
        }
    }

    fn infer_stmt(&mut self, stmt: &RawStmt, output_ty: ScopePtr, arena: &RawExprArena) {
        match stmt.kind {
            RawStmtKind::Loop(raw_loop_kind) => match raw_loop_kind {
                RawLoopKind::For {
                    frame_var,
                    initial_boundary,
                    final_boundary,
                    ..
                } => {
                    should!(self
                        .ty_sheet
                        .variables
                        .insert((frame_var, stmt.row()), Some(BuiltinIdentifier::I32.into()))
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
            RawStmtKind::Branch(_) => todo!(),
            RawStmtKind::Exec(expr) => {
                self.infer_expr(expr, Some(BuiltinIdentifier::Void.into()), arena);
            }
            RawStmtKind::Init {
                varname,
                initial_value,
                ..
            } => {
                let opt_ty = self.infer_expr(initial_value, None, arena);
                insert_new!(self.ty_sheet.variables, (varname, stmt.row()), opt_ty);
            }
            RawStmtKind::Return(result) => {
                self.infer_expr(result, Some(output_ty), arena);
            }
            RawStmtKind::Assert(condition) => self.infer_condition(condition, arena),
        }
    }

    fn infer_loop_bound(&mut self, boundary: RawBoundary, arena: &RawExprArena) {
        if let Some(bound) = boundary.opt_bound {
            self.infer_expr(bound, Some(BuiltinIdentifier::I32.into()), arena);
        }
    }

    fn infer_condition(&mut self, condition: RawExprIdx, arena: &RawExprArena) {
        self.infer_expr(condition, Some(BuiltinIdentifier::Bool.into()), arena);
    }
}
