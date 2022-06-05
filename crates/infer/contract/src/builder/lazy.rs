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
    pub(super) fn infer_lazy_stmts(&mut self, ast_iter: AstIter, arena: &RawExprArena) {
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.variant {
                    AstVariant::Stmt(ref stmt) => self.infer_lazy_stmt(stmt, arena),
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
            RawStmtVariant::Exec {
                expr,
                discard: silent,
            } => panic!(),
            RawStmtVariant::Init {
                varname,
                initial_value,
                ..
            } => {
                if let Ok(ty) = self.raw_expr_ty(initial_value) {
                    LazyContract::pure_or_pass(self.db, ty)
                        .map(|contract| self.infer_lazy_expr(initial_value, contract, arena));
                }
            }
            RawStmtVariant::Return(result) => {
                if let Ok(ty) = self.raw_expr_ty(result) {
                    LazyContract::pure_or_pass(self.db, ty)
                        .map(|contract| self.infer_lazy_expr(result, contract, arena));
                }
            }
            RawStmtVariant::Assert(condition) => self.infer_lazy_condition(condition, arena),
            RawStmtVariant::Break => todo!(),
            RawStmtVariant::Match {
                match_expr,
                match_liason,
            } => self.infer_lazy_expr(match_expr, LazyContract::from_match(match_liason), arena),
            RawStmtVariant::ReturnXml(_) => panic!(),
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
        raw_expr_idx: RawExprIdx,
        contract: LazyContract,
        arena: &RawExprArena,
    ) {
        let infer_result = match arena[raw_expr_idx].variant {
            RawExprVariant::Variable { .. }
            | RawExprVariant::Unrecognized(_)
            | RawExprVariant::Entity { .. }
            | RawExprVariant::CopyableLiteral(_)
            | RawExprVariant::ThisValue { .. }
            | RawExprVariant::ThisField { .. } => Ok(()),
            RawExprVariant::Bracketed(bracketed_expr) => {
                self.infer_lazy_expr(bracketed_expr, contract, arena);
                Ok(())
            }
            RawExprVariant::Opn {
                opn_variant: ref opr,
                ref opds,
            } => self.infer_lazy_opn(
                opr,
                opds,
                contract,
                arena,
                arena[raw_expr_idx].range,
                raw_expr_idx,
            ),
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::FrameVariable {
                varname,
                init_range: init_row,
            } => todo!(),
        };
        self.contract_sheet
            .lazy_expr_contract_results
            .insert_new(raw_expr_idx, infer_result.map(|_| contract));
    }

    fn infer_lazy_opn(
        &mut self,
        opr: &RawOpnVariant,
        opds: &RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        match opr {
            RawOpnVariant::Binary(opr) => self.infer_lazy_binary_opn(*opr, opds, contract, arena),
            RawOpnVariant::Prefix(opr) => {
                self.infer_lazy_prefix_opn(*opr, opds.start, contract, arena)
            }
            RawOpnVariant::Suffix(opr) => {
                self.infer_lazy_suffix(*opr, opds.start, contract, arena, raw_expr_idx)
            }
            RawOpnVariant::FieldAccess(field_ident) => {
                self.infer_lazy_field_access(*field_ident, opds.start, contract, arena)
            }
            RawOpnVariant::List(opr) => {
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
                    LazyContract::EvalRef => todo!(),
                    LazyContract::Pure => (),
                    LazyContract::Pass => (),
                    LazyContract::Move => todo!(),
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
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        match opr {
            SuffixOpr::Incr | SuffixOpr::Decr => throw!(
                format!("mutation not allowed in lazy functional context"),
                arena[raw_expr_idx].range
            ),
            SuffixOpr::AsTy(expr) => {
                self.infer_lazy_expr(opd, contract, arena);
                Ok(())
            }
        }
    }

    fn infer_lazy_field_access(
        &mut self,
        field_ident: RangedCustomIdentifier,
        opd: RawExprIdx,
        contract: LazyContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        let this_ty_decl = self.raw_expr_deref_ty_decl(opd)?;
        let field_decl = this_ty_decl.field_decl(field_ident)?;
        let this_contract = LazyContract::field_access_lazy_contract(
            field_decl.liason,
            contract,
            self.db.is_copyable(field_decl.ty)?,
            arena[opd].range,
        )?;
        self.infer_lazy_expr(opd, this_contract, arena);
        Ok(())
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
            ListOpr::MethodCall { ranged_ident, .. } => self.lazy_method_call(
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
                    call_decl.primary_parameters.iter(),
                ) {
                    let argument_contract = LazyContract::from_parameter(
                        parameter.liason,
                        call_decl.output.liason,
                        arena[argument].range,
                    )?;
                    self.infer_lazy_expr(argument, argument_contract, arena)
                }
                Ok(())
            }
            RawExprVariant::Unrecognized(_) => throw_derived!("unrecognized caller"),
            RawExprVariant::CopyableLiteral(_) | RawExprVariant::FrameVariable { .. } => {
                throw_derived!("a primitive literal can't be a caller")
            }
            RawExprVariant::Variable { .. }
            | RawExprVariant::Bracketed(_)
            | RawExprVariant::Opn { .. }
            | RawExprVariant::ThisValue { .. } => todo!(),
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::ThisField { .. } => todo!(),
        }
    }

    fn lazy_method_call(
        &mut self,
        arena: &RawExprArena,
        this: RawExprIdx,
        ranged_ident: RangedCustomIdentifier,
        inputs: RawExprRange,
        contract: LazyContract,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        let method_decl = self.method_decl(raw_expr_idx)?;
        let this_contract = LazyContract::from_parameter(
            method_decl.this_liason,
            method_decl.output.liason,
            arena[this].range,
        )?;
        self.infer_lazy_expr(this, this_contract, arena);
        for (argument, parameter) in zip(inputs.into_iter(), method_decl.parameters.iter()) {
            let argument_contract = LazyContract::from_parameter(
                parameter.liason,
                method_decl.output.liason,
                arena[argument].range,
            )?;
            self.infer_lazy_expr(argument, argument_contract, arena)
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
        self.infer_lazy_expr(total_opds.start, contract, arena);
        for opd in (total_opds.start + 1)..total_opds.end {
            self.infer_lazy_expr(opd, LazyContract::Pure, arena)
        }
        Ok(())
    }
}
