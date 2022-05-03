use ast::{
    RawBoundary, RawBranchKind, RawExprArena, RawExprRange, RawExprVariant, RawLoopKind, RawStmt,
    RawStmtVariant,
};

use dev_utils::dev_src;
use entity_route::EntityRoutePtr;
use infer_error::*;
use syntax_types::{ListOpr, Opr, PrefixOpr, SuffixOpr};
use text::TextRange;
use vm::{BinaryOpr, FieldContract};
use word::RangedCustomIdentifier;

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
        self.enter_block();
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
        self.exit_block()
    }

    fn infer_eager_stmt(
        &mut self,
        stmt: &RawStmt,
        output_ty: EntityRoutePtr,
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
                    self.infer_loop_bound(initial_boundary, arena);
                    self.infer_loop_bound(final_boundary, arena);
                }
                RawLoopKind::ForExt { final_boundary, .. } => {
                    self.infer_loop_bound(final_boundary, arena)
                }
                RawLoopKind::While { condition } => self.infer_eager_condition(condition, arena),
                RawLoopKind::DoWhile { condition } => self.infer_eager_condition(condition, arena),
            },
            RawStmtVariant::Branch(branch_kind) => match branch_kind {
                RawBranchKind::If { condition } => self.infer_eager_condition(condition, arena),
                RawBranchKind::Elif { condition } => self.infer_eager_condition(condition, arena),
                RawBranchKind::Else => (),
            },
            RawStmtVariant::Exec(expr) => self.infer_eager_expr(expr, EagerContract::Exec, arena),
            RawStmtVariant::Init { initial_value, .. } => {
                self.infer_eager_expr(initial_value, EagerContract::LetInit, arena);
            }
            RawStmtVariant::Return(result) => {
                self.infer_eager_expr(result, EagerContract::Return, arena);
                should!(!self
                    .contract_sheet
                    .eager_expr_contract_results
                    .get(&result)
                    .is_none())
            }
            RawStmtVariant::Assert(condition) => self.infer_eager_condition(condition, arena),
            RawStmtVariant::Break => (),
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
        let infer_result = match arena[expr_idx].variant {
            RawExprVariant::Variable { .. }
            | RawExprVariant::FrameVariable { .. }
            | RawExprVariant::Unrecognized(_)
            | RawExprVariant::Entity { .. }
            | RawExprVariant::PrimitiveLiteral(_)
            | RawExprVariant::This { .. } => Ok(()),
            RawExprVariant::Bracketed(expr) => return self.infer_eager_expr(expr, contract, arena),
            RawExprVariant::Opn { opr, ref opds } => {
                self.infer_eager_opn(opr, opds, contract, arena, arena[expr_idx].range, expr_idx)
            }
            RawExprVariant::Lambda(_, _) => todo!(),
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
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        match opr {
            Opr::Binary(opr) => self.infer_eager_binary_opn(opr, opds, contract, arena),
            Opr::Prefix(opr) => self.infer_eager_prefix_opn(opr, opds.start, contract, arena),
            Opr::Suffix(opr) => self.infer_eager_suffix(opr, opds.start, contract, arena),
            Opr::List(opr) => {
                self.infer_eager_list_opn(opr, opds, contract, arena, range, raw_expr_idx)
            }
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
                    EagerContract::LetInit => (),
                    EagerContract::VarInit => (),
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
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
        match opr {
            PrefixOpr::Minus => (),
            PrefixOpr::Not => (),
            PrefixOpr::BitNot => match contract {
                EagerContract::Pure => (),
                EagerContract::GlobalRef => todo!(),
                EagerContract::Move => todo!(),
                EagerContract::LetInit => todo!(),
                EagerContract::VarInit => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::BorrowMut => todo!(),
                EagerContract::TakeMut => todo!(),
                EagerContract::Exec => todo!(),
            },
            PrefixOpr::Shared => todo!(),
            PrefixOpr::Exclusive => todo!(),
        }
        Ok(())
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
            SuffixOpr::MembAccess(ranged_ident) => {
                let this_ty_decl = self.expr_ty_decl(opd)?;
                let field_var_decl = this_ty_decl.field_decl(ranged_ident)?;
                let this_contract = match field_var_decl.contract {
                    FieldContract::Own => match contract {
                        EagerContract::Pure => EagerContract::Pure,
                        EagerContract::GlobalRef => todo!(),
                        EagerContract::Move => EagerContract::Move,
                        EagerContract::Return => {
                            if self.db.is_copy_constructible(field_var_decl.ty) {
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
                        EagerContract::UseMemberForLetInit => todo!(),
                        EagerContract::UseMemberForVarInit => todo!(),
                    },
                    FieldContract::GlobalRef => todo!(),
                    FieldContract::LazyOwn => todo!(),
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
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        match opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.infer_eager_list_call(opds, contract, arena, range),
            ListOpr::Index => self.infer_eager_element_access(arena, opds, contract, raw_expr_idx),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
        }
    }

    fn infer_eager_list_call(
        &mut self,
        all_opds: &RawExprRange,
        contract: EagerContract,
        arena: &RawExprArena,
        range: TextRange,
    ) -> InferResult<()> {
        let call_expr = &arena[all_opds.start];
        match call_expr.variant {
            RawExprVariant::Entity { route: scope, .. } => {
                let call_decl = self.db.call_decl(scope)?;
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
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                }
                for i in 0..call_decl.inputs.len() {
                    self.infer_eager_expr(
                        all_opds.start + 1 + i,
                        call_decl.inputs[i]
                            .contract
                            .eager(call_decl.output.contract)?,
                        arena,
                    )
                }
                Ok(())
            }
            RawExprVariant::Opn { opr, ref opds } => match opr {
                Opr::Binary(_) => todo!(),
                Opr::Prefix(_) => todo!(),
                Opr::Suffix(suffix_opr) => match suffix_opr {
                    SuffixOpr::Incr => todo!(),
                    SuffixOpr::Decr => todo!(),
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::MembAccess(ident) => self.infer_eager_method(
                        opds.start,
                        ident,
                        (all_opds.start + 1)..all_opds.end,
                        contract,
                        arena,
                        range,
                    ),
                    SuffixOpr::WithType(_) => todo!(),
                },
                Opr::List(_) => todo!(),
            },
            RawExprVariant::Variable { varname, init_row } => todo!(),
            RawExprVariant::This { ty } => todo!(),
            RawExprVariant::Unrecognized(_) => todo!(),
            RawExprVariant::PrimitiveLiteral(_) => todo!(),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::FrameVariable { varname, init_row } => todo!(),
        }
    }

    fn infer_eager_method(
        &mut self,
        this: RawExprIdx,
        ranged_ident: RangedCustomIdentifier,
        inputs: RawExprRange,
        contract: EagerContract,
        arena: &RawExprArena,
        range: TextRange,
    ) -> InferResult<()> {
        let this_ty_decl = self.expr_ty_decl(this)?;
        let method_call_decl = this_ty_decl.method(ranged_ident, &self.trait_uses)?;
        match contract {
            EagerContract::Pure => (),
            EagerContract::Move => (),
            EagerContract::Return => (),
            EagerContract::LetInit => (),
            EagerContract::VarInit => (),
            EagerContract::Exec => (),
            EagerContract::GlobalRef => todo!(),
            EagerContract::BorrowMut => todo!(),
            EagerContract::TakeMut => todo!(),
            EagerContract::UseMemberForLetInit => todo!(),
            EagerContract::UseMemberForVarInit => todo!(),
        }
        self.infer_eager_expr(
            this,
            method_call_decl
                .this_contract
                .eager(method_call_decl.output.contract)?,
            arena,
        );
        if inputs.end - inputs.start != method_call_decl.inputs.len() {
            todo!()
        }
        for i in 0..method_call_decl.inputs.len() {
            self.infer_eager_expr(
                inputs.start + i,
                method_call_decl.inputs[i]
                    .contract
                    .eager(method_call_decl.output.contract)?,
                arena,
            )
        }
        Ok(())
    }

    fn infer_eager_element_access(
        &mut self,
        arena: &RawExprArena,
        total_opds: &RawExprRange,
        contract: EagerContract,
        expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        let this_contract = match contract {
            EagerContract::Pure => EagerContract::Pure,
            EagerContract::GlobalRef => todo!(),
            EagerContract::Move => todo!(),
            EagerContract::LetInit => EagerContract::UseMemberForLetInit,
            EagerContract::VarInit => todo!(),
            EagerContract::Return => {
                let ty = self.expr_ty_result(expr_idx)?;
                if self.db.is_copy_constructible(ty) {
                    EagerContract::Pure
                } else {
                    EagerContract::Move
                }
            }
            EagerContract::BorrowMut => EagerContract::BorrowMut,
            EagerContract::TakeMut => EagerContract::TakeMut,
            EagerContract::Exec => Err(InferError {
                variant: InferErrorVariant::Derived {
                    message: "can't exec element access".to_string(),
                },
                dev_src: dev_src!(),
            })?,
            EagerContract::UseMemberForLetInit => todo!(),
            EagerContract::UseMemberForVarInit => todo!(),
        };
        self.infer_eager_expr(total_opds.start, this_contract, arena);
        for opd in (total_opds.start + 1)..total_opds.end {
            self.infer_eager_expr(opd, EagerContract::Pure, arena)
        }
        Ok(())
    }
}
