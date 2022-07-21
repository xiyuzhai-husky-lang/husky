use std::iter::zip;

use husky_ast::*;

use husky_text::RangedCustomIdentifier;
use husky_text::TextRange;
use infer_error::*;
use vm::*;

use super::*;
use crate::*;

impl<'a> ContractSheetBuilder<'a> {
    pub(super) fn infer_lazy_stmts(&mut self, ast_iter: AstIter) {
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.variant {
                    AstVariant::Stmt(ref stmt) => self.infer_lazy_stmt(stmt),
                    _ => (),
                }
            }
            if let Some(children) = item.opt_children {
                self.infer_lazy_stmts(children)
            }
        }
    }

    fn infer_lazy_stmt(&mut self, stmt: &RawStmt) {
        match stmt.variant {
            RawStmtVariant::Loop(raw_loop_kind) => panic!(),
            RawStmtVariant::ConditionBranch {
                condition_branch_kind,
            } => match condition_branch_kind {
                RawConditionBranchKind::If { condition } => self.infer_lazy_condition(condition),
                RawConditionBranchKind::Elif { condition } => self.infer_lazy_condition(condition),
                RawConditionBranchKind::Else => (),
            },
            RawStmtVariant::PatternBranch {
                ref pattern_branch_variant,
            } => match pattern_branch_variant {
                RawPatternBranchVariant::Case { pattern } => self.infer_lazy_pattern(pattern),
                RawPatternBranchVariant::Default => (),
            },
            RawStmtVariant::Exec { .. } => panic!(),
            RawStmtVariant::Init { initial_value, .. } => {
                if let Ok(ty) = self.raw_expr_ty(initial_value) {
                    LazyContract::pure_or_pass(self.db, ty)
                        .ok()
                        .map(|contract| self.infer_lazy_expr(initial_value, contract));
                }
            }
            RawStmtVariant::Return { result, .. } => {
                if let Ok(ty) = self.raw_expr_ty(result) {
                    LazyContract::pure_or_pass(self.db, ty)
                        .ok()
                        .map(|contract| self.infer_lazy_expr(result, contract));
                }
            }
            RawStmtVariant::Assert(condition) => self.infer_lazy_condition(condition),
            RawStmtVariant::Break => (),
            RawStmtVariant::Match {
                match_expr,
                match_liason,
            } => self.infer_lazy_expr(match_expr, LazyContract::from_match(match_liason)),
            RawStmtVariant::ReturnXml(ref xml_expr) => match xml_expr.variant {
                RawXmlExprVariant::Value(raw_expr_idx) => {
                    self.infer_lazy_expr(raw_expr_idx, LazyContract::Pass);
                }
                RawXmlExprVariant::Tag { ref props, .. } => {
                    props.iter().for_each(|(_, argument)| {
                        self.infer_lazy_expr(*argument, LazyContract::Pass);
                    })
                }
            },
        }
    }

    fn infer_lazy_condition(&mut self, condition: RawExprIdx) {
        self.infer_lazy_expr(condition, LazyContract::Pure)
    }

    fn infer_lazy_pattern(&mut self, pattern: &CasePattern) {
        match pattern.variant {
            CasePatternVariant::PrimitiveLiteral(_) => (),
            CasePatternVariant::OneOf { .. } => (),
            CasePatternVariant::EnumLiteral(_) => (),
        }
    }

    pub(super) fn infer_lazy_expr(&mut self, raw_expr_idx: RawExprIdx, contract: LazyContract) {
        let infer_result = match self.arena[raw_expr_idx].variant {
            RawExprVariant::Variable { .. }
            | RawExprVariant::Unrecognized(_)
            | RawExprVariant::Entity { .. }
            | RawExprVariant::CopyableLiteral(_)
            | RawExprVariant::ThisValue { .. }
            | RawExprVariant::ThisField { .. } => Ok(()),
            RawExprVariant::Bracketed(bracketed_expr) => {
                self.infer_lazy_expr(bracketed_expr, contract);
                Ok(())
            }
            RawExprVariant::Opn {
                opn_variant: ref opr,
                ref opds,
            } => self.infer_lazy_opn(
                opr,
                opds,
                contract,
                self.arena[raw_expr_idx].range,
                raw_expr_idx,
            ),
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::FrameVariable { .. } => panic!(),
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
        range: TextRange,
        idx: RawExprIdx,
    ) -> InferResult<()> {
        match opr {
            RawOpnVariant::Binary(opr) => self.infer_lazy_binary_opn(idx, *opr, opds, contract),
            RawOpnVariant::Prefix(opr) => self.infer_lazy_prefix_opn(*opr, opds.start, contract),
            RawOpnVariant::Suffix(opr) => self.lazy_suffix(idx, *opr, opds.start, contract),
            RawOpnVariant::Field(ident) => self.infer_lazy_field(*ident, opds.start, contract),
            RawOpnVariant::List(opr) => self.infer_lazy_list_opn(idx, opr, opds, contract),
        }
    }

    fn infer_lazy_binary_opn(
        &mut self,
        idx: RawExprIdx,
        opr: BinaryOpr,
        opds: &RawExprRange,
        contract: LazyContract,
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
                self.infer_lazy_expr(lopd, LazyContract::Pure);
                self.infer_lazy_expr(ropd, LazyContract::Pure);
            }
            BinaryOpr::Assign(opr) => {
                throw!(
                    format!("mutation not allowed in lazy functional context"),
                    self.arena[idx].range
                )
            }
        }
        Ok(())
    }

    fn infer_lazy_prefix_opn(
        &mut self,
        opr: PrefixOpr,
        opd: RawExprIdx,
        contract: LazyContract,
    ) -> InferResult<()> {
        todo!()
    }

    fn lazy_suffix(
        &mut self,
        raw_expr_idx: RawExprIdx,
        opr: SuffixOpr,
        opd: RawExprIdx,
        contract: LazyContract,
    ) -> InferResult<()> {
        match opr {
            SuffixOpr::Incr | SuffixOpr::Decr => throw!(
                format!("mutation not allowed in lazy functional context"),
                self.arena[raw_expr_idx].range
            ),
            SuffixOpr::AsTy(expr) => {
                self.infer_lazy_expr(opd, contract);
                Ok(())
            }
        }
    }

    fn infer_lazy_field(
        &mut self,
        field_ident: RangedCustomIdentifier,
        opd: RawExprIdx,
        contract: LazyContract,
    ) -> InferResult<()> {
        let this_ty_decl = self.raw_expr_deref_ty_decl(opd)?;
        let field_decl = this_ty_decl.field_decl(field_ident)?;
        let this_contract = LazyContract::field_access_lazy_contract(
            field_decl.liason,
            contract,
            self.db.is_copyable(field_decl.ty)?,
            self.arena[opd].range,
        )?;
        self.infer_lazy_expr(opd, this_contract);
        Ok(())
    }

    fn infer_lazy_list_opn(
        &mut self,
        idx: RawExprIdx,
        opr: &ListOpr,
        opds: &RawExprRange,
        contract: LazyContract,
    ) -> InferResult<()> {
        match opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => self.infer_lazy_new_vec_from_list(idx, opds.clone(), contract),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.infer_lazy_call(idx, opds, contract),
            ListOpr::Index => self.infer_lazy_element_access(opds, contract, idx),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
            ListOpr::MethodCall { ranged_ident, .. } => self.lazy_method_call(
                idx,
                opds.start,
                *ranged_ident,
                (opds.start + 1)..(opds.end),
                contract,
            ),
        }
    }

    fn infer_lazy_new_vec_from_list(
        &mut self,
        idx: RawExprIdx,
        elements: RawExprRange,
        contract: LazyContract,
    ) -> InferResult<()> {
        let element_ty = self.raw_expr_ty(elements.start)?;
        let element_contract = match self.db.is_copyable(element_ty)? {
            true => LazyContract::Pure,
            false => LazyContract::Move,
        };
        for element in elements {
            self.infer_lazy_expr(element, element_contract)
        }
        Ok(())
    }

    fn infer_lazy_call(
        &mut self,
        raw_expr_idx: RawExprIdx,
        total_opds: &RawExprRange,
        contract: LazyContract,
    ) -> InferResult<()> {
        let call_expr = &self.arena[total_opds.start];
        let call_decl = match call_expr.variant {
            RawExprVariant::Entity { route, .. } => {
                derived_unwrap!(self.db.call_form_decl(route))
            }
            RawExprVariant::Unrecognized(_) => throw_derived!("unrecognized caller"),
            _ => todo!(),
        };
        self.infer_lazy_expr(total_opds.start, LazyContract::Pure);
        for (argument, parameter) in zip(
            ((total_opds.start + 1)..total_opds.end).into_iter(),
            call_decl.primary_parameters.iter(),
        ) {
            let argument_contract = LazyContract::parameter_lazy_contract(
                parameter.liason,
                call_decl.output.liason,
                self.arena[argument].range,
            )?;
            self.infer_lazy_expr(argument, argument_contract)
        }
        Ok(())
    }

    fn lazy_method_call(
        &mut self,
        idx: RawExprIdx,
        this: RawExprIdx,
        ranged_ident: RangedCustomIdentifier,
        inputs: RawExprRange,
        contract: LazyContract,
    ) -> InferResult<()> {
        let call_form_decl = self.call_form_decl(idx)?;
        let this_contract = LazyContract::parameter_lazy_contract(
            call_form_decl.this_liason(),
            call_form_decl.output.liason,
            self.arena[this].range,
        )?;
        self.infer_lazy_expr(this, this_contract);
        for (argument, parameter) in
            zip(inputs.into_iter(), call_form_decl.primary_parameters.iter())
        {
            let argument_contract = LazyContract::parameter_lazy_contract(
                parameter.liason,
                call_form_decl.output.liason,
                self.arena[argument].range,
            )?;
            self.infer_lazy_expr(argument, argument_contract)
        }
        Ok(())
    }

    fn infer_lazy_element_access(
        &mut self,

        total_opds: &RawExprRange,
        contract: LazyContract,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        self.infer_lazy_expr(total_opds.start, contract);
        for opd in (total_opds.start + 1)..total_opds.end {
            self.infer_lazy_expr(opd, LazyContract::Pure)
        }
        Ok(())
    }
}
