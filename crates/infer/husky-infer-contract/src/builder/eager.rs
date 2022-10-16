use super::*;
use crate::*;
use husky_ast::*;
use husky_entity_route::EntityRoutePtr;
use husky_infer_error::*;
use husky_pattern_syntax::{RawPattern, RawPatternVariant};
use husky_text::RangedCustomIdentifier;
use infer_decl::VariadicParametersDecl;

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
            RawStmtVariant::IfElseBranch {
                condition_branch_kind,
            } => match condition_branch_kind {
                RawConditionBranchKind::If { condition } => self.infer_eager_condition(condition),
                RawConditionBranchKind::Elif { condition } => self.infer_eager_condition(condition),
                RawConditionBranchKind::Else => (),
            },
            RawStmtVariant::Require { condition, .. } => self.infer_eager_condition(condition),
            RawStmtVariant::MatchBranch {
                ref pattern_branch_variant,
            } => match pattern_branch_variant {
                RawPatternBranchVariant::Case { pattern } => self.infer_eager_pattern(pattern),
                RawPatternBranchVariant::Default => (),
            },
            RawStmtVariant::Exec { expr, .. } => self.infer_eager_expr(expr, EagerContract::Pure),
            RawStmtVariant::Init { initial_value, .. } => {
                if let Ok(ty) = self.expr_raw_ty(initial_value) {
                    if let Ok(contract) = EagerContract::init_contract(self.db.upcast(), ty.into())
                    {
                        self.infer_eager_expr(initial_value, contract);
                    }
                }
            }
            RawStmtVariant::Return {
                result,
                return_context,
            } => {
                if let Ok(expr_ty) = self.expr_raw_ty(result) {
                    if let Ok(contract) = EagerContract::ret_contract(
                        self.db.upcast(),
                        output_ty,
                        expr_ty,
                        return_context,
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

    fn infer_eager_pattern(&mut self, pattern: &RawPattern) {
        match pattern.variant {
            RawPatternVariant::PrimitiveLiteral(_) => (),
            RawPatternVariant::OneOf { .. } => (),
            RawPatternVariant::EnumLiteral(_) => (),
            RawPatternVariant::Some => todo!(),
            RawPatternVariant::None => todo!(),
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

    pub(super) fn infer_eager_expr(&mut self, idx: RawExprIdx, contract: EagerContract) {
        let raw_expr = &self.arena[idx];
        let infer_result = match raw_expr.variant {
            RawExprVariant::Variable { .. } => Ok(()),
            RawExprVariant::FrameVariable { .. }
            | RawExprVariant::Unrecognized(_)
            | RawExprVariant::Entity { .. }
            | RawExprVariant::PrimitiveLiteral(_)
            | RawExprVariant::ThisValue { .. }
            | RawExprVariant::ThisField { .. } => Ok(()),
            RawExprVariant::Bracketed(expr) => {
                self.infer_eager_expr(expr, contract);
                Ok(())
            }
            RawExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.infer_eager_opn(idx, opn_variant, opds, contract),
            RawExprVariant::Lambda(_, _) => todo!(),
        };
        self.contract_sheet.eager_expr_contract_results.insert_new(
            idx,
            match infer_result {
                Ok(_) => Ok(contract),
                Err(e) => Err(e),
            },
        );
    }

    fn infer_eager_opn(
        &mut self,
        idx: RawExprIdx,
        opn_variant: &RawOpnVariant,
        opds: &RawExprRange,
        contract: EagerContract,
    ) -> InferResult<()> {
        match opn_variant {
            RawOpnVariant::Binary(opr) => self.infer_eager_binary_opn(idx, *opr, opds, contract),
            RawOpnVariant::Prefix(opr) => {
                self.infer_eager_prefix_opn(idx, *opr, opds.start, contract)
            }
            RawOpnVariant::Suffix(opr) => self.infer_eager_suffix(opr, opds.start, contract),
            RawOpnVariant::List(opr) => self.infer_eager_list_opn(idx, opr, opds, contract),
            RawOpnVariant::Field(field_ident) => {
                self.infer_eager_field_access(idx, *field_ident, opds.start, contract)
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
        let intrinsic_lopd_ty = self.expr_raw_ty(opds.start)?.intrinsic();
        let is_lopd_copyable = self.db.is_copyable(intrinsic_lopd_ty)?;
        match opr {
            BinaryOpr::Pure(_) => {
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
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::Curry => todo!(),
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
        suffix: &RawSuffixOpr,
        opd: RawExprIdx,
        contract: EagerContract,
    ) -> InferResult<()> {
        match suffix {
            RawSuffixOpr::Incr | RawSuffixOpr::Decr => {
                self.infer_eager_expr(opd, EagerContract::TempRefMut);
                match contract {
                    EagerContract::Pure => Ok(()),
                    _ => todo!(),
                }
            }
            RawSuffixOpr::AsTy(_) => {
                self.infer_eager_expr(opd, contract);
                Ok(())
            }
            RawSuffixOpr::BePattern(_) => todo!(),
            RawSuffixOpr::Unveil => {
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
        let this_ty_decl = self.expr_ty_decl(opd)?;
        let field_decl = this_ty_decl.field_decl(field_ident)?;
        let this_contract = EagerContract::member_self_eager_contract(
            field_decl.modifier,
            contract,
            field_decl.ty,
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
            ListOpr::NewTuple => {
                p!(self.arena[idx].range);
                todo!()
            }
            ListOpr::NewVec => self.infer_eager_new_vec_from_list(opds.clone()),
            ListOpr::NewDict => todo!(),
            ListOpr::FunctionCall => self.infer_eager_function_call(idx, opds),
            ListOpr::Index => self.eager_index(idx, opds, contract),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
            ListOpr::MethodCall { .. } => self.infer_eager_method_call(idx, opds, contract),
        }
    }

    fn infer_eager_new_vec_from_list(
        &mut self,
        elements: RawExprRange,
    ) -> Result<(), husky_infer_error::InferError> {
        let element_ty = self.expr_raw_ty(elements.start)?;
        let element_contract = match self.db.is_copyable(element_ty)? {
            true => EagerContract::Pure,
            false => EagerContract::Move,
        };
        for element in elements {
            self.infer_eager_expr(element, element_contract)
        }
        Ok(())
    }

    fn infer_eager_function_call(
        &mut self,
        idx: RawExprIdx,
        opds: &RawExprRange,
    ) -> InferResult<()> {
        let decl = derived_unwrap!(self.function_call_form_decl(opds.start));
        msg_once!("other contracts on call form");
        self.infer_eager_expr(opds.start, EagerContract::Pure);
        for (argument, parameter) in decl.match_primary(opds)? {
            let argument_contract = EagerContract::parameter_eager_contract(
                self.db,
                parameter.modifier,
                parameter.ty(),
                decl.output.liason(),
            )?;
            self.infer_eager_expr(argument, argument_contract)
        }
        for argument in decl.match_variadics(opds)? {
            match decl.variadic_parameters {
                VariadicParametersDecl::None => Err(derived!("unexpected"))?,
                VariadicParametersDecl::RepeatSingle { ref parameter } => todo!(),
            }
        }
        Ok(())
    }

    fn infer_eager_method_call(
        &mut self,
        idx: RawExprIdx,
        opds: &RawExprRange,
        contract: EagerContract,
    ) -> InferResult<()> {
        let this = opds.start;
        let decl = self.method_call_form_decl(this)?;
        let this_contract = EagerContract::method_call_this_eager_contract(
            decl.opt_this_liason.unwrap(),
            decl.output.liason(),
            contract,
        );
        self.infer_eager_expr(this, this_contract);
        for (argument, parameter) in decl.match_primary(opds)? {
            let argument_contract = EagerContract::parameter_eager_contract(
                self.db,
                parameter.modifier,
                parameter.ty(),
                decl.output.liason(),
            )?;
            self.infer_eager_expr(argument, argument_contract)
        }
        for argument in decl.match_variadics(opds)? {
            match decl.variadic_parameters {
                VariadicParametersDecl::None => todo!(),
                VariadicParametersDecl::RepeatSingle { ref parameter } => todo!(),
            }
        }
        Ok(())
    }

    fn eager_index(
        &mut self,
        idx: RawExprIdx,
        total_opds: &RawExprRange,
        contract: EagerContract,
    ) -> InferResult<()> {
        let elem_ty = self.expr_raw_ty(idx)?;
        self.infer_eager_expr(
            total_opds.start,
            EagerContract::member_self_eager_contract(
                MemberModifier::Mutable, // ad hoc
                contract,
                elem_ty,
                self.arena[idx].range,
            )?,
        );
        for opd in (total_opds.start + 1)..total_opds.end {
            self.infer_eager_expr(opd, EagerContract::Pure)
        }
        Ok(())
    }
}
