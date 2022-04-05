use ast::{
    RawBoundary, RawExprArena, RawExprKind, RawExprRange, RawLoopKind, RawStmt, RawStmtKind,
};

use entity_route::EntityRoutePtr;
use infer_error::*;
use syntax_types::{ListOpr, Opr, PrefixOpr, SuffixOpr};
use vm::{BinaryOpr, MembAccessContract};
use word::CustomIdentifier;

use super::*;
use crate::*;

impl<'a> ContractSheetBuilder<'a> {
    pub(crate) fn infer_routine(
        &mut self,
        output_ty: EntityRoutePtr,
        ast_iter: AstIter,
        arena: &RawExprArena,
    ) {
        self.infer_eager_stmts(ast_iter.clone(), output_ty, arena);
    }

    pub(super) fn infer_eager_stmts(
        &mut self,
        ast_iter: AstIter,
        output_ty: EntityRoutePtr,
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

    fn infer_eager_stmt(
        &mut self,
        stmt: &RawStmt,
        output_ty: EntityRoutePtr,
        arena: &RawExprArena,
    ) {
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
                RawLoopKind::While { condition } => self.infer_eager_condition(condition, arena),
                RawLoopKind::DoWhile { condition } => self.infer_eager_condition(condition, arena),
            },
            RawStmtKind::Branch(_) => todo!(),
            RawStmtKind::Exec(expr) => self.infer_eager_expr(expr, EagerContract::Exec, arena),
            RawStmtKind::Init { initial_value, .. } => {
                self.infer_eager_expr(initial_value, EagerContract::LetInit, arena);
            }
            RawStmtKind::Return(result) => {
                self.infer_eager_expr(result, EagerContract::Return, arena);
                should!(!self
                    .contract_sheet
                    .eager_expr_contract_results
                    .get(&result)
                    .is_none())
            }
            RawStmtKind::Assert(condition) => self.infer_eager_condition(condition, arena),
        }
    }

    fn infer_loop_bound(&mut self, boundary: RawBoundary, arena: &RawExprArena) {
        if let Some(bound) = boundary.opt_bound {
            self.infer_eager_expr(bound, EagerContract::Pure, arena)
        }
    }

    fn infer_eager_condition(&mut self, condition: RawExprIdx, arena: &RawExprArena) {
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
            | RawExprKind::PrimitiveLiteral(_)
            | RawExprKind::This { .. } => Ok(()),
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
                    EagerContract::Pure | EagerContract::Move | EagerContract::Return => (),
                    EagerContract::GlobalRef => todo!(),
                    EagerContract::BorrowMut => todo!(),
                    EagerContract::TakeMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::LetInit => todo!(),
                    EagerContract::VarInit => todo!(),
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
            SuffixOpr::MembAccess(ident) => {
                let this_ty_signature = self.db.expr_ty_signature(self.file, opd)?;
                let memb_var_signature = this_ty_signature.memb_access_signature(ident);
                let this_contract = match memb_var_signature.contract {
                    MembAccessContract::Own => match contract {
                        EagerContract::Pure => EagerContract::Pure,
                        EagerContract::GlobalRef => todo!(),
                        EagerContract::Move => EagerContract::Move,
                        EagerContract::Return => {
                            let is_copyable = self.db.is_copyable(memb_var_signature.ty);
                            if is_copyable {
                                EagerContract::Pure
                            } else {
                                todo!()
                            }
                        }
                        EagerContract::BorrowMut => EagerContract::BorrowMut,
                        EagerContract::TakeMut => todo!(),
                        EagerContract::Exec => todo!(),
                        EagerContract::LetInit => todo!(),
                        EagerContract::VarInit => todo!(),
                    },
                    MembAccessContract::Ref => todo!(),
                    MembAccessContract::LazyOwn => todo!(),
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
            ListOpr::Call => self.infer_eager_list_call(opds, contract, arena),
            ListOpr::Index => todo!(),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
        }
    }

    fn infer_eager_list_call(
        &mut self,
        all_opds: &RawExprRange,
        contract: EagerContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        let call_expr = &arena[all_opds.start];
        match call_expr.kind {
            RawExprKind::Scope { scope, .. } => {
                let call_signature = self.db.call_signature(scope)?;
                match contract {
                    EagerContract::Pure => (),
                    EagerContract::Move => (),
                    EagerContract::Return => (),
                    EagerContract::LetInit => (),
                    EagerContract::VarInit => (),
                    EagerContract::GlobalRef => todo!(),
                    EagerContract::BorrowMut => todo!(),
                    EagerContract::TakeMut => todo!(),
                    EagerContract::Exec => todo!(),
                }
                for i in 0..call_signature.inputs.len() {
                    self.infer_eager_expr(
                        all_opds.start + 1 + i,
                        call_signature.inputs[i].contract.eager()?,
                        arena,
                    )
                }
                Ok(())
            }
            RawExprKind::Opn { opr, ref opds } => match opr {
                Opr::Binary(_) => todo!(),
                Opr::Prefix(_) => todo!(),
                Opr::Suffix(suffix_opr) => match suffix_opr {
                    SuffixOpr::Incr => todo!(),
                    SuffixOpr::Decr => todo!(),
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::MembAccess(ident) => self.infer_eager_memb_call(
                        opds.start,
                        ident,
                        (all_opds.start + 1)..all_opds.end,
                        contract,
                        arena,
                    ),
                    SuffixOpr::WithType(_) => todo!(),
                },
                Opr::List(_) => todo!(),
            },
            RawExprKind::Variable { varname, init_row } => todo!(),
            RawExprKind::This { ty } => todo!(),
            RawExprKind::Unrecognized(_) => todo!(),
            RawExprKind::PrimitiveLiteral(_) => todo!(),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Lambda(_, _) => todo!(),
        }
    }

    fn infer_eager_memb_call(
        &mut self,
        this: RawExprIdx,
        ident: CustomIdentifier,
        inputs: RawExprRange,
        contract: EagerContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        let this_ty_signature = derived_ok!(self.db.expr_ty_signature(self.file, this));
        let memb_call_signature = this_ty_signature.memb_call_signature(ident)?;
        match contract {
            EagerContract::Pure => (),
            EagerContract::Move => (),
            EagerContract::GlobalRef => todo!(),
            EagerContract::BorrowMut => todo!(),
            EagerContract::TakeMut => todo!(),
            EagerContract::Exec => match memb_call_signature.output {
                EntityRoutePtr::Builtin(BuiltinIdentifier::Void) => (),
                _ => err!("no discard"),
            },
            EagerContract::LetInit => todo!(),
            EagerContract::VarInit => todo!(),
            EagerContract::Return => todo!(),
        }
        self.infer_eager_expr(this, memb_call_signature.this_contract.eager()?, arena);
        if inputs.end - inputs.start != memb_call_signature.inputs.len() {
            todo!()
        }
        for i in 0..memb_call_signature.inputs.len() {
            self.infer_eager_expr(
                inputs.start + 1,
                memb_call_signature.inputs[i].contract.eager()?,
                arena,
            )
        }
        Ok(())
    }
}
