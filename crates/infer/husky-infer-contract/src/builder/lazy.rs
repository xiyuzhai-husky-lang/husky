use husky_ast::*;
use infer_decl::{CallFormDecl, ParameterDecl, VariadicParametersDecl};

use super::*;
use crate::*;
use husky_infer_error::*;
use husky_pattern_syntax::{RawPattern, RawPatternVariant};
use husky_text::TextRange;
use husky_text::{RangedCustomIdentifier, TextRanged};

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
            RawStmtVariant::Loop(_) => panic!(),
            RawStmtVariant::IfElseBranch {
                condition_branch_kind,
            } => match condition_branch_kind {
                RawConditionBranchKind::If { condition } => self.infer_lazy_condition(condition),
                RawConditionBranchKind::Elif { condition } => self.infer_lazy_condition(condition),
                RawConditionBranchKind::Else => (),
            },
            RawStmtVariant::MatchBranch {
                ref pattern_branch_variant,
            } => match pattern_branch_variant {
                RawPatternBranchVariant::Case { pattern } => self.infer_lazy_pattern(pattern),
                RawPatternBranchVariant::Default => (),
            },
            RawStmtVariant::Exec { .. } => panic!(),
            RawStmtVariant::Init { initial_value, .. } => {
                if let Ok(ty) = self.expr_raw_ty(initial_value) {
                    LazyContract::init_contract(self.db, ty.into())
                        .ok()
                        .map(|contract| self.infer_lazy_expr(initial_value, contract));
                }
            }
            RawStmtVariant::Return { result, .. } => {
                if let Ok(ty) = self.expr_raw_ty(result) {
                    LazyContract::init_contract(self.db, ty.into())
                        .ok()
                        .map(|contract| self.infer_lazy_expr(result, contract));
                }
            }
            RawStmtVariant::Assert(condition) => self.infer_lazy_condition(condition),
            RawStmtVariant::Require { condition, .. } => self.infer_lazy_condition(condition),
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

    fn infer_lazy_pattern(&mut self, pattern: &RawPattern) {
        match pattern.variant {
            RawPatternVariant::PrimitiveLiteral(_) => (),
            RawPatternVariant::OneOf { .. } => (),
            RawPatternVariant::EnumLiteral(_) => (),
            RawPatternVariant::Some => todo!(),
            RawPatternVariant::None => todo!(),
        }
    }

    pub(super) fn infer_lazy_expr(&mut self, raw_expr_idx: RawExprIdx, contract: LazyContract) {
        let infer_result = match self.arena[raw_expr_idx].variant {
            RawExprVariant::Variable { .. }
            | RawExprVariant::Unrecognized(_)
            | RawExprVariant::Entity { .. }
            | RawExprVariant::PrimitiveLiteral(_)
            | RawExprVariant::ThisValue { .. }
            | RawExprVariant::ThisField { .. } => Ok(()),
            RawExprVariant::Bracketed(bracketed_expr) => {
                self.infer_lazy_expr(bracketed_expr, contract);
                Ok(())
            }
            RawExprVariant::Opn {
                opn_variant: ref opr,
                ref opds,
            } => self.infer_lazy_opn(opr, opds, contract, raw_expr_idx),
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
        idx: RawExprIdx,
    ) -> InferResult<()> {
        match opr {
            RawOpnVariant::Binary(opr) => self.infer_lazy_binary_opn(idx, *opr, opds, contract),
            RawOpnVariant::Prefix(opr) => self.infer_lazy_prefix_opn(*opr, opds.start, contract),
            RawOpnVariant::Suffix(opr) => self.lazy_suffix(idx, opr, opds.start, contract),
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
            BinaryOpr::Pure(_) => {
                match contract {
                    LazyContract::EvalRef => todo!(),
                    LazyContract::Pure => (),
                    LazyContract::Pass => (),
                    LazyContract::Move => todo!(),
                }
                self.infer_lazy_expr(lopd, LazyContract::Pure);
                self.infer_lazy_expr(ropd, LazyContract::Pure);
            }
            BinaryOpr::Assign(_) => {
                throw!(
                    format!("mutation not allowed in lazy functional context"),
                    self.arena[idx].range
                )
            }
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::Curry => todo!(),
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
        opr: &RawSuffixOpr,
        opd: RawExprIdx,
        contract: LazyContract,
    ) -> InferResult<()> {
        match opr {
            RawSuffixOpr::Incr | RawSuffixOpr::Decr => throw!(
                format!("mutation not allowed in lazy functional context"),
                self.arena[raw_expr_idx].range
            ),
            RawSuffixOpr::AsTy(_) => {
                self.infer_lazy_expr(opd, contract);
                Ok(())
            }
            RawSuffixOpr::BePattern(_) => {
                self.infer_lazy_expr(opd, LazyContract::Pure);
                Ok(())
            }
            RawSuffixOpr::Unveil => {
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
        let this_ty_decl = self.expr_ty_decl(opd)?;
        let field_decl = this_ty_decl.field_decl(field_ident)?;
        let this_contract =
            LazyContract::member_self_lazy_contract(field_decl.modifier, contract, field_decl.ty)?;
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
            ListOpr::NewTuple => todo!(),
            ListOpr::NewVec => self.infer_lazy_new_vec_from_list(opds.clone()),
            ListOpr::NewDict => todo!(),
            ListOpr::Index => self.infer_lazy_index(opds, contract, idx),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
            ListOpr::FunctionCall => self.infer_lazy_function_call(idx, opds),
            ListOpr::MethodCall { .. } => self.lazy_method_call(idx, opds),
        }
    }

    fn infer_lazy_new_vec_from_list(&mut self, elements: RawExprRange) -> InferResult<()> {
        let element_ty = self.expr_raw_ty(elements.start)?;
        let element_contract = match self.db.is_copyable(element_ty)? {
            true => LazyContract::Pure,
            false => LazyContract::Move,
        };
        for element in elements {
            self.infer_lazy_expr(element, element_contract)
        }
        Ok(())
    }

    fn infer_lazy_function_call(
        &mut self,
        idx: RawExprIdx,
        opds: &RawExprRange,
    ) -> InferResult<()> {
        let decl = self.function_call_form_decl(opds.start)?;
        self.infer_lazy_expr(opds.start, LazyContract::Pure);
        self.infer_lazy_arguments(&decl, idx, opds)
    }

    fn infer_lazy_arguments(
        &mut self,
        decl: &CallFormDecl,
        idx: RawExprIdx,
        opds: &RawExprRange,
    ) -> InferResult<()> {
        for (argument, parameter) in decl.match_primary(opds)? {
            self.infer_lazy_argument(decl, argument, parameter)?
        }
        for argument in decl.match_variadics(opds)? {
            match decl.variadic_parameters {
                VariadicParametersDecl::None => todo!(),
                VariadicParametersDecl::RepeatSingle { ref parameter } => {
                    self.infer_lazy_argument(decl, argument, parameter)?
                }
            }
        }
        Ok(())
    }

    fn infer_lazy_argument(
        &mut self,
        decl: &CallFormDecl,
        argument: RawExprIdx,
        parameter: &ParameterDecl,
    ) -> InferResult<()> {
        let contract = LazyContract::parameter_lazy_contract(
            self.db,
            parameter.modifier,
            parameter.ty(),
            decl.output.liason(),
        )?;
        self.infer_lazy_expr(argument, contract);
        Ok(())
    }

    fn infer_lazy_this_argument(
        &mut self,
        decl: &CallFormDecl,
        argument: RawExprIdx,
    ) -> InferResult<()> {
        let contract = LazyContract::parameter_lazy_contract(
            self.db,
            decl.this_liason(),
            decl.opt_route.unwrap().parent(),
            decl.output.liason(),
        )?;
        self.infer_lazy_expr(argument, contract);
        Ok(())
    }

    fn lazy_method_call(&mut self, idx: RawExprIdx, opds: &RawExprRange) -> InferResult<()> {
        let this = opds.start;
        let decl = self.method_call_form_decl(this)?;
        self.infer_lazy_this_argument(&decl, this)?;
        self.infer_lazy_arguments(&decl, idx, opds)
    }

    fn infer_lazy_index(
        &mut self,
        total_opds: &RawExprRange,
        contract: LazyContract,
        idx: RawExprIdx,
    ) -> InferResult<()> {
        let elem_ty = self.expr_raw_ty(idx)?;
        self.infer_lazy_expr(
            total_opds.start,
            LazyContract::member_self_lazy_contract(
                MemberModifier::Mutable, // ad hoc
                contract,
                elem_ty,
            )?,
        );
        for opd in (total_opds.start + 1)..total_opds.end {
            self.infer_lazy_expr(opd, LazyContract::Pure)
        }
        Ok(())
    }
}
