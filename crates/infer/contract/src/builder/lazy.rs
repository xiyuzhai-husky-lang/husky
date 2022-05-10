use ast::*;

use infer_error::*;
use text::TextRange;
use vm::*;
use word::RangedCustomIdentifier;

use super::*;
use crate::*;

impl<'a> ContractSheetBuilder<'a> {
    pub(crate) fn infer_morphism(&mut self, ast_iter: AstIter, arena: &RawExprArena) {
        self.infer_lazy_stmts(ast_iter.clone(), arena);
    }
    pub(super) fn infer_lazy_stmts(&mut self, ast_iter: AstIter, arena: &RawExprArena) {
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.kind {
                    AstKind::Stmt(ref stmt) => self.infer_lazy_stmt(stmt, arena),
                    _ => (),
                }
            }
            if let Some(children) = item.children {
                self.infer_lazy_stmts(children, arena)
            }
        }
    }

    fn infer_lazy_stmt(&mut self, stmt: &RawStmt, arena: &RawExprArena) {
        match stmt.variant {
            RawStmtVariant::Loop(raw_loop_kind) => panic!(),
            RawStmtVariant::Branch(_) => todo!(),
            RawStmtVariant::Exec(expr) => panic!(),
            RawStmtVariant::Init {
                varname,
                initial_value,
                ..
            } => {
                self.infer_lazy_expr(initial_value, LazyContract::Move, arena);
            }
            RawStmtVariant::Return(result) => {
                self.infer_lazy_expr(result, LazyContract::Move, arena)
            }
            RawStmtVariant::Assert(condition) => self.infer_lazy_condition(condition, arena),
            RawStmtVariant::Break => todo!(),
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
        let infer_result = match arena[expr_idx].variant {
            RawExprVariant::Variable { .. }
            | RawExprVariant::Unrecognized(_)
            | RawExprVariant::Entity { .. }
            | RawExprVariant::PrimitiveLiteral(_)
            | RawExprVariant::This { .. } => Ok(()),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn { opr, ref opds } => {
                self.infer_lazy_opn(opr, opds, contract, arena, arena[expr_idx].range, expr_idx)
            }
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::FrameVariable { varname, init_row } => todo!(),
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
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        match opr {
            Opr::Binary(opr) => self.infer_lazy_binary_opn(opr, opds, contract, arena),
            Opr::Prefix(opr) => self.infer_lazy_prefix_opn(opr, opds.start, contract, arena),
            Opr::Suffix(opr) => self.infer_lazy_suffix(opr, opds.start, contract, arena),
            Opr::List(opr) => {
                self.infer_lazy_list_opn(opr, opds, contract, arena, range, raw_expr_idx)
            }
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
                    LazyContract::Move => (),
                    LazyContract::GlobalRef => todo!(),
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
            SuffixOpr::FieldAccess(ranged_ident) => {
                let this_ty_decl = self.raw_expr_ty_decl(opd)?;
                let this_contract = match this_ty_decl.field_decl(ranged_ident)?.contract {
                    FieldContract::Own => match contract {
                        LazyContract::Move => LazyContract::Move,
                        LazyContract::GlobalRef => todo!(),
                        LazyContract::Pure => LazyContract::Pure,
                    },
                    FieldContract::GlobalRef => todo!(),
                    FieldContract::LazyOwn => match contract {
                        LazyContract::Move => todo!(),
                        LazyContract::GlobalRef => todo!(),
                        LazyContract::Pure => LazyContract::Pure,
                    },
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
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        match opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.infer_lazy_list_call(opds, contract, arena, range, raw_expr_idx),
            ListOpr::Index => self.infer_lazy_element_access(arena, opds, contract, raw_expr_idx),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
        }
    }

    fn infer_lazy_list_call(
        &mut self,
        all_opds: &RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        let call_expr = &arena[all_opds.start];
        match call_expr.variant {
            RawExprVariant::Entity { route: scope, .. } => {
                let call_decl = self.db.call_decl(scope)?;
                for i in 0..call_decl.inputs.len() {
                    self.infer_lazy_expr(
                        all_opds.start + 1 + i,
                        call_decl.inputs[i]
                            .contract
                            .lazy(call_decl.output.contract)?,
                        arena,
                    )
                }
                Ok(())
            }
            RawExprVariant::Variable { .. } => todo!(),
            RawExprVariant::Unrecognized(_) => todo!(),
            RawExprVariant::PrimitiveLiteral(_) => todo!(),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn { opr, ref opds } => match opr {
                Opr::Binary(_) => todo!(),
                Opr::Prefix(_) => todo!(),
                Opr::Suffix(suffix_opr) => match suffix_opr {
                    SuffixOpr::Incr => todo!(),
                    SuffixOpr::Decr => todo!(),
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::FieldAccess(ranged_ident) => self.infer_lazy_method(
                        opds.start,
                        ranged_ident,
                        (all_opds.start + 1)..all_opds.end,
                        contract,
                        arena,
                        raw_expr_idx,
                    ),
                    SuffixOpr::WithType(_) => todo!(),
                },
                Opr::List(_) => todo!(),
            },
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::This { .. } => todo!(),
            RawExprVariant::FrameVariable { varname, init_row } => todo!(),
        }
    }

    fn infer_lazy_method(
        &mut self,
        this: RawExprIdx,
        ranged_ident: RangedCustomIdentifier,
        inputs: RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        let method_decl = self.method_decl(raw_expr_idx)?;
        match contract {
            LazyContract::Move => (),
            LazyContract::GlobalRef => todo!(),
            LazyContract::Pure => (),
        }
        self.infer_lazy_expr(
            this,
            method_decl
                .this_contract
                .lazy(method_decl.output.contract)?,
            arena,
        );
        if inputs.end - inputs.start != method_decl.inputs.len() {
            todo!()
        }
        for i in 0..method_decl.inputs.len() {
            self.infer_lazy_expr(
                inputs.start + 1,
                method_decl.inputs[i]
                    .contract
                    .lazy(method_decl.output.contract)?,
                arena,
            )
        }
        Ok(())
    }

    fn infer_lazy_element_access(
        &mut self,
        arena: &RawExprArena,
        total_opds: &RawExprRange,
        contract: LazyContract,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        match contract {
            LazyContract::Move => {
                let ty = self.raw_expr_ty(raw_expr_idx)?;
                let this_contract = if self.db.is_copyable(ty) {
                    LazyContract::Pure
                } else {
                    LazyContract::Move
                };
                self.infer_lazy_expr(total_opds.start, this_contract, arena)
            }
            LazyContract::GlobalRef => todo!(),
            LazyContract::Pure => todo!(),
        }
        for opd in (total_opds.start + 1)..total_opds.end {
            self.infer_lazy_expr(opd, LazyContract::Pure, arena)
        }
        Ok(())
    }
}
