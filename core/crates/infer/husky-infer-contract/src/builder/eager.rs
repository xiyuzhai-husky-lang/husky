use std::iter::zip;

use husky_ast::*;

use husky_entity_route::EntityRoutePtr;
use husky_text::RangedCustomIdentifier;
use husky_text::TextRange;
use infer_error::*;
use vm::*;

use super::*;
use crate::*;

impl<'a> ContractSheetBuilder<'a> {
    pub(super) fn infer_eager_stmts(&mut self, ast_iter: AstIter, output_ty: EntityRoutePtr) {
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.variant {
                    AstVariant::Stmt(ref stmt) => self.infer_eager_stmt(stmt, output_ty),
                    _ => (),
                }
            }
            if let Some(children) = item.opt_children {
                self.infer_eager_stmts(children, output_ty)
            }
        }
    }

    fn infer_eager_stmt(&mut self, stmt: &RawStmt, output_ty: EntityRoutePtr) {
        match stmt.variant {
            RawStmtVariant::Loop(raw_loop_kind) => match raw_loop_kind {
                RawLoopKind::For {
                    initial_boundary,
                    final_boundary,
                    ..
                } => {
                    self.infer_loop_bound(initial_boundary);
                    self.infer_loop_bound(final_boundary);
                }
                RawLoopKind::ForExt { final_boundary, .. } => self.infer_loop_bound(final_boundary),
                RawLoopKind::While { condition } => self.infer_eager_condition(condition),
                RawLoopKind::DoWhile { condition } => self.infer_eager_condition(condition),
            },
            RawStmtVariant::ConditionBranch {
                condition_branch_kind,
            } => match condition_branch_kind {
                RawConditionBranchKind::If { condition } => self.infer_eager_condition(condition),
                RawConditionBranchKind::Elif { condition } => self.infer_eager_condition(condition),
                RawConditionBranchKind::Else => (),
            },
            RawStmtVariant::PatternBranch {
                ref pattern_branch_variant,
            } => match pattern_branch_variant {
                RawPatternBranchVariant::Case { pattern } => self.infer_eager_pattern(pattern),
                RawPatternBranchVariant::Default => (),
            },
            RawStmtVariant::Exec { expr, .. } => self.infer_eager_expr(expr, EagerContract::Pure),
            RawStmtVariant::Init { initial_value, .. } => {
                if let Ok(ty) = self.raw_expr_ty(initial_value) {
                    if let Ok(contract) = EagerContract::init_contract(self.db.upcast(), ty) {
                        self.infer_eager_expr(initial_value, contract);
                    }
                }
            }
            RawStmtVariant::Return {
                result,
                return_kind,
            } => {
                if let Ok(return_ty) = self.raw_expr_ty(result) {
                    if let Ok(contract) = EagerContract::ret_contract(
                        self.db.upcast(),
                        output_ty,
                        return_ty,
                        return_kind,
                    ) {
                        self.infer_eager_expr(result, contract);
                    }
                }
            }
            RawStmtVariant::Assert(condition) => self.infer_eager_condition(condition),
            RawStmtVariant::Break => (),
            RawStmtVariant::Match {
                match_expr,
                match_liason,
            } => {
                self.infer_eager_expr(match_expr, EagerContract::from_match(match_liason));
            }
            RawStmtVariant::ReturnXml(_) => panic!(),
        }
    }

    fn infer_eager_pattern(&mut self, pattern: &CasePattern) {
        match pattern.variant {
            CasePatternVariant::PrimitiveLiteral(_) => (),
            CasePatternVariant::OneOf { .. } => (),
            CasePatternVariant::EnumLiteral(_) => (),
        }
    }

    fn infer_loop_bound(&mut self, boundary: RawBoundary) {
        if let Some(bound) = boundary.opt_bound {
            self.infer_eager_expr(bound, EagerContract::Pure)
        }
    }

    fn infer_eager_condition(&mut self, condition: RawExprIdx) {
        self.infer_eager_expr(condition, EagerContract::Pure)
    }

    pub(super) fn infer_eager_expr(&mut self, raw_expr_idx: RawExprIdx, contract: EagerContract) {
        let raw_expr = &self.arena[raw_expr_idx];
        let infer_result = match raw_expr.variant {
            RawExprVariant::Variable { .. } => Ok(()),
            RawExprVariant::FrameVariable { .. }
            | RawExprVariant::Unrecognized(_)
            | RawExprVariant::Entity { .. }
            | RawExprVariant::CopyableLiteral(_)
            | RawExprVariant::ThisValue { .. }
            | RawExprVariant::ThisField { .. } => Ok(()),
            RawExprVariant::Bracketed(expr) => {
                self.infer_eager_expr(expr, contract);
                Ok(())
            }
            RawExprVariant::Opn {
                opn_variant: ref opr,
                ref opds,
            } => self.infer_eager_opn(raw_expr_idx, opr, opds, contract),
            RawExprVariant::Lambda(_, _) => todo!(),
        };
        self.contract_sheet.eager_expr_contract_results.insert_new(
            raw_expr_idx,
            match infer_result {
                Ok(_) => Ok(contract),
                Err(e) => Err(e),
            },
        );
    }

    fn infer_eager_opn(
        &mut self,
        raw_expr_idx: RawExprIdx,
        opr: &RawOpnVariant,
        opds: &RawExprRange,
        contract: EagerContract,
    ) -> InferResult<()> {
        match opr {
            RawOpnVariant::Binary(opr) => {
                self.infer_eager_binary_opn(raw_expr_idx, *opr, opds, contract)
            }
            RawOpnVariant::Prefix(opr) => {
                self.infer_eager_prefix_opn(raw_expr_idx, *opr, opds.start, contract)
            }
            RawOpnVariant::Suffix(opr) => {
                self.infer_eager_suffix(raw_expr_idx, *opr, opds.start, contract)
            }
            RawOpnVariant::List(opr) => {
                self.infer_eager_list_opn(raw_expr_idx, opr, opds, contract)
            }
            RawOpnVariant::Field(field_ident) => {
                self.infer_eager_field_access(raw_expr_idx, *field_ident, opds.start, contract)
            }
        }
    }

    fn infer_eager_binary_opn(
        &mut self,
        raw_expr_idx: RawExprIdx,
        opr: BinaryOpr,
        opds: &RawExprRange,
        contract: EagerContract,
    ) -> InferResult<()> {
        let lopd = opds.start;
        let ropd = opds.start + 1;
        let lopd_ty = self.raw_expr_intrinsic_ty(opds.start)?;
        let is_lopd_copyable = self.db.is_copyable(lopd_ty)?;
        match opr {
            BinaryOpr::Pure(pure_binary_opr) => {
                match contract {
                    EagerContract::Pure | EagerContract::Move => (),
                    EagerContract::Pass => panic!(),
                    _ => {
                        throw!(
                            format!("can't bind output to a ref"),
                            self.arena[raw_expr_idx].range
                        )
                    }
                }
                self.infer_eager_expr(lopd, EagerContract::Pure);
                self.infer_eager_expr(ropd, EagerContract::Pure);
            }
            BinaryOpr::Assign(opt_opr) => {
                match contract {
                    EagerContract::Pure => (),
                    _ => {
                        p!(contract, self.arena[opds.start]);
                        todo!()
                    }
                }
                self.infer_eager_expr(lopd, EagerContract::TempRefMut);
                self.infer_eager_expr(
                    ropd,
                    match (opt_opr, is_lopd_copyable) {
                        (None, false) => EagerContract::Move,
                        _ => EagerContract::Pure,
                    },
                );
            }
        }
        Ok(())
    }

    fn infer_eager_prefix_opn(
        &mut self,
        raw_expr_idx: RawExprIdx,
        opr: PrefixOpr,
        opd: RawExprIdx,
        contract: EagerContract,
    ) -> InferResult<()> {
        let opd_contract = match opr {
            PrefixOpr::Minus | PrefixOpr::Not | PrefixOpr::BitNot => {
                match contract {
                    EagerContract::Pure | EagerContract::Move => (),
                    EagerContract::Pass => panic!(),
                    EagerContract::TempRefMut | EagerContract::EvalRef | EagerContract::TempRef => {
                        throw!(
                            format!("can't bind this to a ref"),
                            self.arena[raw_expr_idx].range
                        )
                    }
                }
                EagerContract::Pure
            }
            PrefixOpr::Shared => todo!(),
            PrefixOpr::Move => todo!(),
        };
        self.infer_eager_expr(opd, opd_contract);
        Ok(())
    }

    fn infer_eager_suffix(
        &mut self,
        raw_expr_idx: RawExprIdx,
        opr: SuffixOpr,
        opd: RawExprIdx,
        contract: EagerContract,
    ) -> InferResult<()> {
        match opr {
            SuffixOpr::Incr | SuffixOpr::Decr => {
                self.infer_eager_expr(opd, EagerContract::TempRefMut);
                match contract {
                    EagerContract::Pure => Ok(()),
                    _ => todo!(),
                }
            }
            SuffixOpr::AsTy(_) => {
                self.infer_eager_expr(opd, contract);
                Ok(())
            }
        }
    }

    fn infer_eager_field_access(
        &mut self,
        raw_expr_idx: RawExprIdx,
        field_ident: RangedCustomIdentifier,
        opd: RawExprIdx,
        contract: EagerContract,
    ) -> InferResult<()> {
        let this_ty_decl = self.raw_expr_deref_ty_decl(opd)?;
        let field_decl = this_ty_decl.field_decl(field_ident)?;
        let this_contract = EagerContract::field_access_this_eager_contract(
            field_decl.liason,
            contract,
            self.db.is_copyable(field_decl.ty)?,
            self.arena[raw_expr_idx].range,
        )?;
        self.infer_eager_expr(opd, this_contract);
        Ok(())
    }

    fn infer_eager_list_opn(
        &mut self,
        idx: RawExprIdx,
        opr: &ListOpr,
        opds: &RawExprRange,
        contract: EagerContract,
    ) -> InferResult<()> {
        match opr {
            ListOpr::TupleInit => {
                p!(self.arena[idx].range);
                todo!()
            }
            ListOpr::NewVec => self.infer_eager_new_vec_from_list(idx, opds.clone(), contract),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.infer_eager_call(idx, opds, contract),
            ListOpr::Index => self.eager_index(idx, opds, contract),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
            ListOpr::MethodCall { .. } => self.infer_eager_method_call(
                opds.start,
                (opds.start + 1)..(opds.end),
                contract,
                idx,
            ),
        }
    }

    fn infer_eager_new_vec_from_list(
        &mut self,
        idx: arena::ArenaIdx<RawExpr>,
        elements: RawExprRange,
        contract: EagerContract,
    ) -> Result<(), infer_error::InferError> {
        let element_ty = self.raw_expr_ty(elements.start)?;
        let element_contract = match self.db.is_copyable(element_ty)? {
            true => EagerContract::Pure,
            false => EagerContract::Move,
        };
        for element in elements {
            self.infer_eager_expr(element, element_contract)
        }
        Ok(())
    }

    fn infer_eager_call(
        &mut self,
        raw_expr_idx: RawExprIdx,
        total_opds: &RawExprRange,
        contract: EagerContract,
    ) -> InferResult<()> {
        let call_expr = &self.arena[total_opds.start];
        let call_decl = self.call_form_decl(total_opds.start).unwrap();
        // match call_expr.variant {
        //     RawExprVariant::Entity { route, .. } => {
        //         derived_unwrap!(self.db.call_form_decl(route))
        //     }
        //     _ => {
        //         msg_once!("supports more than pure functions");
        //         todo!()
        //     }
        // };
        for (argument, parameter) in zip(
            ((total_opds.start + 1)..total_opds.end).into_iter(),
            call_decl.primary_parameters.iter(),
        ) {
            let argument_contract = EagerContract::argument_eager_contract(
                parameter.ty,
                parameter.liason,
                call_decl.output.liason,
                self.arena[argument].range,
            );
            self.infer_eager_expr(argument, argument_contract)
        }
        Ok(())
    }

    fn infer_eager_method_call(
        &mut self,
        this: RawExprIdx,
        parameters: RawExprRange,
        contract: EagerContract,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        let call_form_decl = self.call_form_decl(raw_expr_idx)?;
        let this_contract = EagerContract::method_call_this_eager_contract(
            call_form_decl.opt_this_liason.unwrap(),
            call_form_decl.output.liason,
            contract,
        );
        self.infer_eager_expr(this, this_contract);
        for (argument, parameter) in zip(
            parameters.into_iter(),
            call_form_decl.primary_parameters.iter(),
        ) {
            let argument_contract = EagerContract::argument_eager_contract(
                parameter.ty,
                parameter.liason,
                call_form_decl.output.liason,
                self.arena[argument].range,
            );
            self.infer_eager_expr(argument, argument_contract)
        }
        Ok(())
    }

    fn eager_index(
        &mut self,
        raw_expr_idx: RawExprIdx,
        total_opds: &RawExprRange,
        contract: EagerContract,
    ) -> InferResult<()> {
        self.infer_eager_expr(total_opds.start, contract);
        for opd in (total_opds.start + 1)..total_opds.end {
            self.infer_eager_expr(opd, EagerContract::Pure)
        }
        Ok(())
    }
}
