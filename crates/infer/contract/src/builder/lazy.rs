use ast::*;

use infer_error::*;
use scope::{InputPlaceholder, ScopeKind, ScopePtr};
use syntax_types::{ListOpr, Opr, PrefixOpr, SuffixOpr};
use vm::{BinaryOpr, MembVarContract};
use word::CustomIdentifier;

use super::*;
use crate::*;

impl<'a> ContractSheetBuilder<'a> {
    pub(crate) fn infer_def(
        &mut self,
        output_ty: ScopePtr,
        ast_iter: AstIter,
        arena: &RawExprArena,
    ) {
        self.infer_lazy_stmts(ast_iter.clone(), output_ty, arena);
    }
    pub(super) fn infer_lazy_stmts(
        &mut self,
        ast_iter: AstIter,
        output_ty: ScopePtr,
        arena: &RawExprArena,
    ) {
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.kind {
                    AstKind::Stmt(ref stmt) => self.infer_lazy_stmt(stmt, output_ty, arena),
                    _ => (),
                }
            }
            if let Some(children) = item.children {
                self.infer_lazy_stmts(children, output_ty, arena)
            }
        }
    }

    fn infer_lazy_stmt(&mut self, stmt: &RawStmt, output_ty: ScopePtr, arena: &RawExprArena) {
        match stmt.kind {
            RawStmtKind::Loop(raw_loop_kind) => panic!(),
            RawStmtKind::Branch(_) => todo!(),
            RawStmtKind::Exec(expr) => panic!(),
            RawStmtKind::Init {
                varname,
                initial_value,
                ..
            } => {
                self.infer_lazy_expr(initial_value, LazyContract::Take, arena);
            }
            RawStmtKind::Return(result) => self.infer_lazy_expr(result, LazyContract::Take, arena),
            RawStmtKind::Assert(condition) => self.infer_lazy_condition(condition, arena),
        }
    }

    fn infer_lazy_condition(&mut self, condition: RawExprIdx, arena: &RawExprArena) {
        self.infer_lazy_expr(condition, LazyContract::Pure, arena)
    }

    pub(super) fn infer_lazy_expr(
        &mut self,
        expr_idx: RawExprIdx,
        contract: LazyContract,
        arena: &RawExprArena,
    ) {
        let infer_result = match arena[expr_idx].kind {
            RawExprKind::Variable { .. }
            | RawExprKind::Unrecognized(_)
            | RawExprKind::Scope { .. }
            | RawExprKind::PrimitiveLiteral(_) => Ok(()),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, ref opds } => self.infer_lazy_opn(opr, opds, contract, arena),
            RawExprKind::Lambda(_, _) => todo!(),
            RawExprKind::This { .. } => todo!(),
        };
        should!(self
            .contract_sheet
            .lazy_expr_contract_results
            .insert(expr_idx, infer_result.map(|_| contract))
            .is_none());
    }

    fn infer_lazy_opn(
        &mut self,
        opr: Opr,
        opds: &RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        match opr {
            Opr::Binary(opr) => self.infer_lazy_binary_opn(opr, opds, contract, arena),
            Opr::Prefix(opr) => self.infer_lazy_prefix_opn(opr, opds.start, contract, arena),
            Opr::Suffix(opr) => self.infer_lazy_suffix(opr, opds.start, contract, arena),
            Opr::List(opr) => self.infer_lazy_list_opn(opr, opds, contract, arena),
        }
    }

    fn infer_lazy_binary_opn(
        &mut self,
        opr: BinaryOpr,
        opds: &RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        let lopd = opds.start;
        let ropd = opds.start + 1;
        match opr {
            BinaryOpr::Pure(pure_binary_opr) => {
                match contract {
                    LazyContract::Take => todo!(),
                    LazyContract::Ref => todo!(),
                    LazyContract::Pure => (),
                }
                self.infer_lazy_expr(lopd, LazyContract::Pure, arena);
                self.infer_lazy_expr(ropd, LazyContract::Pure, arena);
            }
            BinaryOpr::Assign(_) => panic!(),
        }
        Ok(())
    }

    fn infer_lazy_prefix_opn(
        &mut self,
        opr: PrefixOpr,
        opd: RawExprIdx,
        contract: LazyContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        todo!()
    }

    fn infer_lazy_suffix(
        &mut self,
        opr: SuffixOpr,
        opd: RawExprIdx,
        contract: LazyContract,
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
                        LazyContract::Take => LazyContract::Take,
                        LazyContract::Ref => todo!(),
                        LazyContract::Pure => LazyContract::Pure,
                    },
                    MembVarContract::Ref => todo!(),
                };
                self.infer_lazy_expr(opd, this_contract, arena);
                Ok(())
            }
            SuffixOpr::WithType(_) => todo!(),
        }
    }

    fn infer_lazy_list_opn(
        &mut self,
        opr: ListOpr,
        opds: &RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        match opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.infer_lazy_list_call(opds, contract, arena),
            ListOpr::Index => todo!(),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
        }
    }

    fn infer_lazy_list_call(
        &mut self,
        all_opds: &RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        let call_expr = &arena[all_opds.start];
        match call_expr.kind {
            RawExprKind::Scope { scope, .. } => {
                let call_signature = self.db.call_signature(scope)?;
                for i in 0..call_signature.inputs.len() {
                    self.infer_lazy_expr(
                        all_opds.start + 1 + i,
                        call_signature.inputs[i].contract.lazy()?,
                        arena,
                    )
                }
                Ok(())
            }
            RawExprKind::Variable { .. } => todo!(),
            RawExprKind::Unrecognized(_) => todo!(),
            RawExprKind::PrimitiveLiteral(_) => todo!(),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, ref opds } => match opr {
                Opr::Binary(_) => todo!(),
                Opr::Prefix(_) => todo!(),
                Opr::Suffix(suffix_opr) => match suffix_opr {
                    SuffixOpr::Incr => todo!(),
                    SuffixOpr::Decr => todo!(),
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::MembVarAccess(ident) => self.infer_lazy_memb_call(
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
            RawExprKind::Lambda(_, _) => todo!(),
            RawExprKind::This { .. } => todo!(),
        }
    }

    fn infer_lazy_memb_call(
        &mut self,
        this: RawExprIdx,
        ident: CustomIdentifier,
        inputs: RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        let this_ty_signature = derived_ok!(self.db.expr_ty_signature(self.file, this));
        let memb_call_signature = this_ty_signature.memb_call_signature(ident)?;
        match contract {
            LazyContract::Take => (),
            LazyContract::Ref => todo!(),
            LazyContract::Pure => (),
        }
        self.infer_lazy_expr(this, memb_call_signature.this.lazy()?, arena);
        if inputs.end - inputs.start != memb_call_signature.inputs.len() {
            todo!()
        }
        for i in 0..memb_call_signature.inputs.len() {
            self.infer_lazy_expr(
                inputs.start + 1,
                memb_call_signature.inputs[i].contract.lazy()?,
                arena,
            )
        }
        Ok(())
    }
}
