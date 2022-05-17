use std::iter::zip;

use ast::*;

use infer_error::*;
use text::BindTextRangeInto;
use text::RangedCustomIdentifier;
use text::TextRange;
use vm::*;

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
            if let Some(children) = item.opt_children {
                self.infer_lazy_stmts(children, arena)
            }
        }
    }

    fn infer_lazy_stmt(&mut self, stmt: &RawStmt, arena: &RawExprArena) {
        match stmt.variant {
            RawStmtVariant::Loop(raw_loop_kind) => panic!(),
            RawStmtVariant::ConditionBranch {
                condition_branch_kind,
            } => match condition_branch_kind {
                RawConditionBranchKind::If { condition } => todo!(),
                RawConditionBranchKind::Elif { condition } => todo!(),
                RawConditionBranchKind::Else => todo!(),
            },
            RawStmtVariant::PatternBranch {
                ref pattern_branch_variant,
            } => match pattern_branch_variant {
                RawPatternBranchVariant::Case { pattern } => self.infer_lazy_pattern(pattern),
                RawPatternBranchVariant::Default => (),
            },
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
            RawStmtVariant::Match {
                match_expr,
                match_contract,
            } => self.infer_lazy_expr(match_expr, match_contract.lazy(), arena),
        }
    }

    fn infer_lazy_condition(&mut self, condition: RawExprIdx, arena: &RawExprArena) {
        self.infer_lazy_expr(condition, LazyContract::Pure, arena)
    }

    fn infer_lazy_pattern(&mut self, pattern: &CasePattern) {
        match pattern.variant {
            CasePatternVariant::PrimitiveLiteral(_) => (),
            CasePatternVariant::OneOf { .. } => (),
            CasePatternVariant::EnumLiteral(_) => (),
        }
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
            RawExprVariant::Opn { ref opr, ref opds } => {
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
        opr: &Opr,
        opds: &RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        match opr {
            Opr::Binary(opr) => self.infer_lazy_binary_opn(*opr, opds, contract, arena),
            Opr::Prefix(opr) => self.infer_lazy_prefix_opn(*opr, opds.start, contract, arena),
            Opr::Suffix(opr) => self.infer_lazy_suffix(*opr, opds.start, contract, arena),
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
            SuffixOpr::WithTy(_) => todo!(),
            SuffixOpr::AsTy(_) => todo!(),
        }
    }

    fn infer_lazy_list_opn(
        &mut self,
        opr: &ListOpr,
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
            ListOpr::Call => self.infer_lazy_call(opds, contract, arena, range, raw_expr_idx),
            ListOpr::Index => self.infer_lazy_element_access(arena, opds, contract, raw_expr_idx),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
            ListOpr::MethodCall { ranged_ident, .. } => self.infer_lazy_method_call(
                arena,
                opds.start,
                *ranged_ident,
                (opds.start + 1)..(opds.end),
                contract,
                raw_expr_idx,
            ),
        }
    }

    fn infer_lazy_call(
        &mut self,
        total_opds: &RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        let call_expr = &arena[total_opds.start];
        match call_expr.variant {
            RawExprVariant::Entity { route, .. } => {
                let call_decl = derived_unwrap!(self.db.call_decl(route));
                for (argument, parameter) in zip(
                    ((total_opds.start + 1)..total_opds.end).into_iter(),
                    call_decl.parameters.iter(),
                ) {
                    let argument_contract_result: InferResult<_> = parameter
                        .contract
                        .lazy(call_decl.output.liason)
                        .bind_into(&arena[argument]);
                    self.infer_lazy_expr(argument, argument_contract_result?, arena)
                }
                Ok(())
            }
            RawExprVariant::Unrecognized(_) => throw_derived!("unrecognized caller"),
            RawExprVariant::PrimitiveLiteral(_) | RawExprVariant::FrameVariable { .. } => {
                throw_derived!("a primitive literal can't be a caller")
            }
            RawExprVariant::Variable { .. }
            | RawExprVariant::Bracketed(_)
            | RawExprVariant::Opn { .. }
            | RawExprVariant::This { .. } => todo!(),
            RawExprVariant::Lambda(_, _) => todo!(),
        }
    }

    fn infer_lazy_method_call(
        &mut self,
        arena: &RawExprArena,
        this: RawExprIdx,
        ranged_ident: RangedCustomIdentifier,
        inputs: RawExprRange,
        contract: LazyContract,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        let method_decl = self.method_decl(raw_expr_idx)?;
        match contract {
            LazyContract::Move => (),
            LazyContract::GlobalRef => todo!(),
            LazyContract::Pure => (),
        }
        let this_contract_result: InferResult<_> = method_decl
            .this_contract
            .lazy(method_decl.output.liason)
            .bind_into(&arena[this]);
        self.infer_lazy_expr(this, this_contract_result?, arena);
        for (argument, parameter) in zip(inputs.into_iter(), method_decl.parameters.iter()) {
            let argument_contract_result: InferResult<_> = parameter
                .contract
                .lazy(method_decl.output.liason)
                .bind_into(&arena[argument]);
            self.infer_lazy_expr(argument, argument_contract_result?, arena)
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
                let this_contract = if self.db.is_copyable(ty)? {
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
