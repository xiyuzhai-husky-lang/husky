use ast::{
    RawBoundary, RawExprArena, RawExprRange, RawExprVariant, RawLoopKind, RawStmt, RawStmtKind,
};

use entity_route::EntityRoutePtr;
use infer_error::*;
use syntax_types::{ListOpr, Opr, PrefixOpr, SuffixOpr};
use text::TextRange;
use vm::{BinaryOpr, MembAccessContract};
use word::{CustomIdentifier, RangedCustomIdentifier};

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
            RawExprVariant::Variable { .. }
            | RawExprVariant::Unrecognized(_)
            | RawExprVariant::Scope { .. }
            | RawExprVariant::PrimitiveLiteral(_)
            | RawExprVariant::This { .. } => Ok(()),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn { opr, ref opds } => {
                self.infer_eager_opn(opr, opds, contract, arena, arena[expr_idx].range)
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
    ) -> InferResult<()> {
        match opr {
            Opr::Binary(opr) => self.infer_eager_binary_opn(opr, opds, contract, arena),
            Opr::Prefix(opr) => self.infer_eager_prefix_opn(opr, opds.start, contract, arena),
            Opr::Suffix(opr) => self.infer_eager_suffix(opr, opds.start, contract, arena),
            Opr::List(opr) => self.infer_eager_list_opn(opr, opds, contract, arena, range),
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
            SuffixOpr::MembAccess(ranged_ident) => {
                let this_ty_decl = self.db.expr_ty_decl(self.file, opd)?;
                let field_var_decl = this_ty_decl.field_decl(ranged_ident.ident);
                let this_contract = match field_var_decl.contract {
                    MembAccessContract::Own => match contract {
                        EagerContract::Pure => EagerContract::Pure,
                        EagerContract::GlobalRef => todo!(),
                        EagerContract::Move => EagerContract::Move,
                        EagerContract::Return => {
                            let is_copyable = self.db.is_copyable(field_var_decl.ty);
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
        range: TextRange,
    ) -> InferResult<()> {
        match opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.infer_eager_list_call(opds, contract, arena, range),
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
        range: TextRange,
    ) -> InferResult<()> {
        let call_expr = &arena[all_opds.start];
        match call_expr.kind {
            RawExprVariant::Scope { scope, .. } => {
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
                }
                for i in 0..call_decl.inputs.len() {
                    self.infer_eager_expr(
                        all_opds.start + 1 + i,
                        call_decl.inputs[i].contract.eager()?,
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
        let this_ty_decl = derived_ok!(self.db.expr_ty_decl(self.file, this));
        let method_call_decl = derived_ok!(this_ty_decl.method_decl(ranged_ident));
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
        }
        self.infer_eager_expr(this, method_call_decl.this_contract.eager()?, arena);
        if inputs.end - inputs.start != method_call_decl.inputs.len() {
            todo!()
        }
        for i in 0..method_call_decl.inputs.len() {
            self.infer_eager_expr(
                inputs.start + i,
                method_call_decl.inputs[i].contract.eager()?,
                arena,
            )
        }
        Ok(())
    }
}
