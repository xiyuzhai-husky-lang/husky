use ast::{
    RawBoundary, RawExprArena, RawExprKind, RawExprRange, RawLoopKind, RawStmt, RawStmtKind,
};
use common::{msg_once, p, should};
use infer_error::err;
use scope::{InputPlaceholder, ScopeKind, ScopePtr, ScopeRoute};
use syntax_types::{ListOpr, Opr, PrefixOpr, SuffixOpr};
use vm::{BinaryOpr, MembVarContract, PureBinaryOpr};
use word::BuiltinIdentifier;

use super::*;
use crate::*;

impl<'a> ContractSheetBuilder<'a> {
    pub(crate) fn infer_routine(
        &mut self,
        line_group_idx: usize,
        inputs: &[InputPlaceholder],
        output_ty: ScopePtr,
        ast_iter: AstIter,
        arena: &RawExprArena,
    ) {
        self.infer_eager_stmts(ast_iter.clone(), output_ty, arena);
    }

    pub(super) fn infer_eager_stmts(
        &mut self,
        ast_iter: AstIter,
        output_ty: ScopePtr,
        arena: &RawExprArena,
    ) {
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.kind {
                    AstKind::Stmt(ref stmt) => self.infer_eager_stmt(stmt, output_ty, arena),
                    _ => (),
                }
            }
            if let Some(children) = item.children {
                self.infer_eager_stmts(children, output_ty, arena)
            }
        }
    }

    fn infer_eager_stmt(&mut self, stmt: &RawStmt, output_ty: ScopePtr, arena: &RawExprArena) {
        match stmt.kind {
            RawStmtKind::Loop(raw_loop_kind) => match raw_loop_kind {
                RawLoopKind::For {
                    frame_var,
                    initial_boundary,
                    final_boundary,
                    ..
                } => {
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
            RawStmtKind::Exec(expr) => self.infer_eager_expr(expr, EagerContract::Exec, arena),
            RawStmtKind::Init {
                varname,
                initial_value,
                ..
            } => {
                self.infer_eager_expr(initial_value, EagerContract::Take, arena);
            }
            RawStmtKind::Return(result) => {
                self.infer_eager_expr(result, EagerContract::Take, arena)
            }
            RawStmtKind::Assert(_) => todo!(),
        }
    }

    fn infer_loop_bound(&mut self, boundary: RawBoundary, arena: &RawExprArena) {
        if let Some(bound) = boundary.opt_bound {
            self.infer_eager_expr(bound, EagerContract::Pure, arena)
        }
    }

    fn infer_condition(&mut self, condition: RawExprIdx, arena: &RawExprArena) {
        self.infer_eager_expr(condition, EagerContract::Pure, arena)
    }

    pub(super) fn infer_eager_expr(
        &mut self,
        expr_idx: RawExprIdx,
        contract: EagerContract,
        arena: &RawExprArena,
    ) {
        let infer_result = match arena[expr_idx].kind {
            RawExprKind::Variable { .. }
            | RawExprKind::Unrecognized(_)
            | RawExprKind::Scope { .. }
            | RawExprKind::PrimitiveLiteral(_) => Ok(()),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, ref opds } => self.infer_eager_opn(opr, opds, contract, arena),
            RawExprKind::Lambda(_, _) => todo!(),
        };
        match self.contract_sheet.eager_expr_contract_results.insert(
            expr_idx,
            match infer_result {
                Ok(_) => Ok(contract),
                Err(e) => Err(e),
            },
        ) {
            Some(_) => {
                p!(expr_idx);
                p!(self.file);
                p!(arena);
                panic!()
            }
            None => (),
        }
    }

    fn infer_eager_opn(
        &mut self,
        opr: Opr,
        opds: &RawExprRange,
        contract: EagerContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        match opr {
            Opr::Binary(opr) => self.infer_eager_binary_opn(opr, opds, contract, arena),
            Opr::Prefix(opr) => self.infer_eager_prefix_opn(opr, opds.start, contract, arena),
            Opr::Suffix(opr) => self.infer_eager_suffix(opr, opds.start, contract, arena),
            Opr::List(opr) => self.infer_eager_list_opn(opr, opds, contract, arena),
        }
    }

    fn infer_eager_binary_opn(
        &mut self,
        opr: BinaryOpr,
        opds: &RawExprRange,
        contract: EagerContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        let lopd = opds.start;
        let ropd = opds.start + 1;
        match opr {
            BinaryOpr::Pure(pure_binary_opr) => {
                match contract {
                    EagerContract::Pure => (),
                    EagerContract::Ref => todo!(),
                    EagerContract::Take => todo!(),
                    EagerContract::BorrowMut => todo!(),
                    EagerContract::TakeMut => todo!(),
                    EagerContract::Exec => todo!(),
                }
                self.infer_eager_expr(lopd, EagerContract::Pure, arena);
                self.infer_eager_expr(ropd, EagerContract::Pure, arena);
            }
            BinaryOpr::Assign(_) => {
                match contract {
                    EagerContract::Exec => (),
                    _ => todo!(),
                }
                self.infer_eager_expr(lopd, EagerContract::BorrowMut, arena);
                self.infer_eager_expr(ropd, EagerContract::Pure, arena);
            }
        }
        Ok(())
    }

    fn infer_eager_prefix_opn(
        &mut self,
        opr: PrefixOpr,
        opd: RawExprIdx,
        contract: EagerContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        todo!()
    }

    fn infer_eager_suffix(
        &mut self,
        opr: SuffixOpr,
        opd: RawExprIdx,
        contract: EagerContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        match opr {
            SuffixOpr::Incr => todo!(),
            SuffixOpr::Decr => todo!(),
            SuffixOpr::MayReturn => panic!("should handle this case in parse return statement"),
            SuffixOpr::MembVarAccess(ident) => {
                let this_ty_signature = self.db.expr_ty_signature(self.file, opd)?;
                let this_contract = match this_ty_signature.memb_var_signature(ident).contract {
                    MembVarContract::Own => match contract {
                        EagerContract::Pure => EagerContract::Pure,
                        EagerContract::Ref => todo!(),
                        EagerContract::Take => EagerContract::Take,
                        EagerContract::BorrowMut => EagerContract::BorrowMut,
                        EagerContract::TakeMut => todo!(),
                        EagerContract::Exec => todo!(),
                    },
                    MembVarContract::Ref => todo!(),
                };
                self.infer_eager_expr(opd, this_contract, arena);
                Ok(())
            }
            SuffixOpr::WithType(_) => todo!(),
        }
    }

    fn infer_eager_list_opn(
        &mut self,
        opr: ListOpr,
        opds: &RawExprRange,
        contract: EagerContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        match opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.infer_eager_call(opds, contract, arena),
            ListOpr::Index => todo!(),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
        }
    }

    fn infer_eager_call(
        &mut self,
        opds: &RawExprRange,
        contract: EagerContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        let call_expr = &arena[opds.start];
        let call_signature = match call_expr.kind {
            RawExprKind::Scope { scope, .. } => self.db.call_signature(scope)?,
            _ => todo!(),
        };
        match contract {
            EagerContract::Pure => (),
            EagerContract::Take => (),
            EagerContract::Ref => todo!(),
            EagerContract::BorrowMut => todo!(),
            EagerContract::TakeMut => todo!(),
            EagerContract::Exec => todo!(),
        }
        for i in 0..call_signature.inputs.len() {
            self.infer_eager_expr(opds.start + i, call_signature.inputs[i].contract, arena)
        }
        Ok(())
    }
}
