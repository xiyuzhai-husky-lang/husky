use std::iter::zip;

use ast::*;

use dev_utils::dev_src;
use entity_route::EntityRoutePtr;
use infer_error::*;
use text::BindTextRangeInto;
use text::RangedCustomIdentifier;
use text::TextRange;
use vm::*;

use super::*;
use crate::*;

impl<'a> ContractSheetBuilder<'a> {
    pub(super) fn infer_eager_stmts(&mut self, ast_iter: AstIter, arena: &RawExprArena) {
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.variant {
                    AstVariant::Stmt(ref stmt) => self.infer_eager_stmt(stmt, arena),
                    _ => (),
                }
            }
            if let Some(children) = item.opt_children {
                self.infer_eager_stmts(children, arena)
            }
        }
    }

    fn infer_eager_stmt(&mut self, stmt: &RawStmt, arena: &RawExprArena) {
        match stmt.variant {
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
            RawStmtVariant::ConditionBranch {
                condition_branch_kind,
            } => match condition_branch_kind {
                RawConditionBranchKind::If { condition } => {
                    self.infer_eager_condition(condition, arena)
                }
                RawConditionBranchKind::Elif { condition } => {
                    self.infer_eager_condition(condition, arena)
                }
                RawConditionBranchKind::Else => (),
            },
            RawStmtVariant::PatternBranch {
                ref pattern_branch_variant,
            } => match pattern_branch_variant {
                RawPatternBranchVariant::Case { pattern } => self.infer_eager_pattern(pattern),
                RawPatternBranchVariant::Default => (),
            },
            RawStmtVariant::Exec { expr, discard } => self.infer_eager_expr(
                expr,
                if discard {
                    EagerContract::Pure
                } else {
                    EagerContract::Exec
                },
                arena,
            ),
            RawStmtVariant::Init { initial_value, .. } => {
                self.infer_eager_expr(initial_value, EagerContract::UseForLetInit, arena);
            }
            RawStmtVariant::Return(result) => {
                self.infer_eager_expr(result, EagerContract::Return, arena);
                should!(!self
                    .contract_sheet
                    .eager_expr_contract_results
                    .get(result)
                    .is_none())
            }
            RawStmtVariant::Assert(condition) => self.infer_eager_condition(condition, arena),
            RawStmtVariant::Break => (),
            RawStmtVariant::Match {
                match_expr,
                match_liason,
            } => {
                self.infer_eager_expr(match_expr, EagerContract::from_match(match_liason), arena);
            }
            RawStmtVariant::ReturnXml(ref xml_expr) => match xml_expr.variant {
                RawXmlExprVariant::Value(raw_expr_idx) => {
                    self.infer_eager_expr(raw_expr_idx, EagerContract::Pure, arena);
                }
                RawXmlExprVariant::Tag { ident, ref props } => {
                    props.iter().for_each(|(_, argument)| {
                        self.infer_eager_expr(*argument, EagerContract::Pure, arena);
                    })
                }
            },
        }
    }

    fn infer_eager_pattern(&mut self, pattern: &CasePattern) {
        match pattern.variant {
            CasePatternVariant::PrimitiveLiteral(_) => (),
            CasePatternVariant::OneOf { .. } => (),
            CasePatternVariant::EnumLiteral(_) => (),
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
        raw_expr_idx: RawExprIdx,
        contract: EagerContract,
        arena: &RawExprArena,
    ) {
        let raw_expr = &arena[raw_expr_idx];
        let infer_result = match raw_expr.variant {
            RawExprVariant::Variable {
                varname,
                init_range,
            } => Ok(()),
            RawExprVariant::FrameVariable { .. }
            | RawExprVariant::Unrecognized(_)
            | RawExprVariant::Entity { .. }
            | RawExprVariant::CopyableLiteral(_)
            | RawExprVariant::ThisValue { .. }
            | RawExprVariant::ThisField { .. } => Ok(()),
            RawExprVariant::Bracketed(expr) => {
                self.infer_eager_expr(expr, contract, arena);
                Ok(())
            }
            RawExprVariant::Opn {
                opn_variant: ref opr,
                ref opds,
            } => self.infer_eager_opn(
                opr,
                opds,
                contract,
                arena,
                arena[raw_expr_idx].range,
                raw_expr_idx,
            ),
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
        opr: &RawOpnVariant,
        opds: &RawExprRange,
        contract: EagerContract,
        arena: &RawExprArena,
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        match opr {
            RawOpnVariant::Binary(opr) => {
                self.infer_eager_binary_opn(*opr, opds, contract, arena, raw_expr_idx)
            }
            RawOpnVariant::Prefix(opr) => {
                self.infer_eager_prefix_opn(*opr, opds.start, contract, arena)
            }
            RawOpnVariant::Suffix(opr) => {
                self.infer_eager_suffix(*opr, opds.start, contract, arena, raw_expr_idx)
            }
            RawOpnVariant::List(opr) => {
                self.infer_eager_list_opn(opr, opds, contract, arena, range, raw_expr_idx)
            }
            RawOpnVariant::FieldAccess(field_ident) => self.infer_eager_field_access(
                *field_ident,
                opds.start,
                contract,
                arena,
                raw_expr_idx,
            ),
        }
    }

    fn infer_eager_binary_opn(
        &mut self,
        opr: BinaryOpr,
        opds: &RawExprRange,
        contract: EagerContract,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        let lopd = opds.start;
        let ropd = opds.start + 1;
        let lopd_ty = self.raw_expr_ty(opds.start)?;
        let is_lopd_copyable = self.db.is_copyable(lopd_ty)?;
        match opr {
            BinaryOpr::Pure(pure_binary_opr) => {
                match contract {
                    EagerContract::Pure | EagerContract::Move | EagerContract::Return => (),
                    EagerContract::TempRefMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForLetInit => (),
                    EagerContract::UseForVarInit => (),
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::UseForAssignRvalue => todo!(),
                    EagerContract::EvalRef => todo!(),
                }
                self.infer_eager_expr(lopd, EagerContract::Pure, arena);
                self.infer_eager_expr(ropd, EagerContract::Pure, arena);
            }
            BinaryOpr::Assign(opt_opr) => {
                match contract {
                    EagerContract::Exec => (),
                    EagerContract::Pure => {
                        throw_derived!(format!("can't use value of type `void` as argument"))
                    }
                    _ => {
                        p!(contract, arena[opds.start]);
                        todo!()
                    }
                }
                self.infer_eager_expr(lopd, EagerContract::TempRefMut, arena);
                self.infer_eager_expr(
                    ropd,
                    match (opt_opr, is_lopd_copyable) {
                        (None, false) => EagerContract::UseForAssignRvalue,
                        _ => EagerContract::Pure,
                    },
                    arena,
                );
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
        let opd_contract = match opr {
            PrefixOpr::Minus | PrefixOpr::Not | PrefixOpr::BitNot => {
                match contract {
                    EagerContract::Pure => (),
                    EagerContract::Move => todo!(),
                    EagerContract::UseForLetInit => todo!(),
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::TempRefMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForAssignRvalue => todo!(),
                    EagerContract::EvalRef => todo!(),
                }
                EagerContract::Pure
            }
            PrefixOpr::Shared => todo!(),
            PrefixOpr::Move => todo!(),
        };
        self.infer_eager_expr(opd, opd_contract, arena);
        Ok(())
    }

    fn infer_eager_suffix(
        &mut self,
        opr: SuffixOpr,
        opd: RawExprIdx,
        contract: EagerContract,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        match opr {
            SuffixOpr::Incr | SuffixOpr::Decr => {
                self.infer_eager_expr(opd, EagerContract::TempRefMut, arena);
                match contract {
                    EagerContract::Exec => Ok(()),
                    _ => todo!(),
                }
            }
            SuffixOpr::WithTy(_) => todo!(),
            SuffixOpr::AsTy(_) => {
                self.infer_eager_expr(
                    opd,
                    match contract {
                        EagerContract::Pure | EagerContract::Return => contract,
                        EagerContract::Move => todo!(),
                        EagerContract::UseForLetInit => todo!(),
                        EagerContract::UseForVarInit => todo!(),
                        EagerContract::UseMemberForLetInit => todo!(),
                        EagerContract::UseMemberForVarInit => todo!(),
                        EagerContract::TempRefMut => todo!(),
                        EagerContract::Exec => todo!(),
                        EagerContract::UseForAssignRvalue => todo!(),
                        EagerContract::EvalRef => todo!(),
                    },
                    arena,
                );
                Ok(())
            }
        }
    }

    fn infer_eager_field_access(
        &mut self,
        field_ident: RangedCustomIdentifier,
        opd: RawExprIdx,
        contract: EagerContract,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        let this_ty_decl = self.raw_expr_ty_decl(opd)?;
        let field_decl = this_ty_decl.field_decl(field_ident)?;
        let this_contract = EagerContract::from_field_access(
            field_decl.liason,
            contract,
            self.db.is_copyable(field_decl.ty)?,
            arena[raw_expr_idx].range,
        )?;
        self.infer_eager_expr(opd, this_contract, arena);
        Ok(())
    }

    fn infer_eager_list_opn(
        &mut self,
        opr: &ListOpr,
        opds: &RawExprRange,
        contract: EagerContract,
        arena: &RawExprArena,
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        match opr {
            ListOpr::TupleInit => {
                p!(range);
                todo!()
            }
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.infer_eager_call(opds, contract, arena, range, raw_expr_idx),
            ListOpr::Index => self.eager_element_access(arena, opds, contract, raw_expr_idx),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
            ListOpr::MethodCall { ranged_ident, .. } => self.eager_method_call(
                arena,
                opds.start,
                *ranged_ident,
                (opds.start + 1)..(opds.end),
                contract,
                range,
                raw_expr_idx,
            ),
        }
    }

    fn infer_eager_call(
        &mut self,
        total_opds: &RawExprRange,
        contract: EagerContract,
        arena: &RawExprArena,
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        let call_expr = &arena[total_opds.start];
        let output_ty = self.raw_expr_ty(raw_expr_idx)?;
        let is_output_ty_copyable = self.db.is_copyable(output_ty)?;
        match call_expr.variant {
            RawExprVariant::Entity { route, .. } => {
                let call_decl = derived_unwrap!(self.db.call_decl(route));
                for (argument, parameter) in zip(
                    ((total_opds.start + 1)..total_opds.end).into_iter(),
                    call_decl.primary_parameters.iter(),
                ) {
                    let argument_contract = EagerContract::from_parameter(
                        parameter.ty,
                        parameter.liason,
                        call_decl.output.liason,
                        contract,
                        is_output_ty_copyable,
                        arena[argument].range,
                    )?;
                    self.infer_eager_expr(argument, argument_contract, arena)
                }
                Ok(())
            }
            RawExprVariant::Opn {
                opn_variant: ref opr,
                ref opds,
            } => todo!(),
            RawExprVariant::Variable {
                varname,
                init_range: init_row,
            } => todo!(),
            RawExprVariant::ThisValue {
                opt_this_ty: opt_ty,
                ..
            } => todo!(),
            RawExprVariant::Unrecognized(_) => throw_derived!("unrecognized caller"),
            RawExprVariant::CopyableLiteral(_) => {
                throw_derived!("a primitive literal can't be a caller")
            }
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::FrameVariable {
                varname,
                init_range: init_row,
            } => todo!(),
            RawExprVariant::ThisField { .. } => todo!(),
        }
    }

    fn eager_method_call(
        &mut self,
        arena: &RawExprArena,
        this: RawExprIdx,
        ranged_ident: RangedCustomIdentifier,
        parameters: RawExprRange,
        contract: EagerContract,
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        let method_decl = self.method_decl(raw_expr_idx)?;
        let is_output_ty_copyable = self.db.is_copyable(method_decl.output.ty)?;
        let this_contract = EagerContract::from_this(
            method_decl.this_liason,
            method_decl.output.liason,
            contract,
            is_output_ty_copyable,
            arena[this].range,
        )?;
        self.infer_eager_expr(this, this_contract, arena);
        for (argument, parameter) in zip(parameters.into_iter(), method_decl.parameters.iter()) {
            let argument_contract = EagerContract::from_parameter(
                parameter.ty,
                parameter.liason,
                method_decl.output.liason,
                contract,
                is_output_ty_copyable,
                arena[argument].range,
            )?;
            self.infer_eager_expr(argument, argument_contract, arena)
        }
        Ok(())
    }

    fn eager_element_access(
        &mut self,
        arena: &RawExprArena,
        total_opds: &RawExprRange,
        contract: EagerContract,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<()> {
        let this_contract = match contract {
            EagerContract::Pure => EagerContract::Pure,
            EagerContract::Move => todo!(),
            EagerContract::UseForLetInit => EagerContract::UseMemberForLetInit,
            EagerContract::UseForVarInit => todo!(),
            EagerContract::Return => {
                let ty = self.raw_expr_ty(raw_expr_idx)?;
                if self.db.is_copyable(ty)? {
                    EagerContract::Pure
                } else {
                    EagerContract::Move
                }
            }
            EagerContract::TempRefMut => EagerContract::TempRefMut,
            EagerContract::Exec => Err(InferError {
                variant: InferErrorVariant::Derived {
                    message: "can't exec element access".to_string(),
                },
                dev_src: dev_src!(),
            })?,
            EagerContract::UseMemberForLetInit => todo!(),
            EagerContract::UseMemberForVarInit => todo!(),
            EagerContract::UseForAssignRvalue => {
                throw!(
                    format!("can't use noncopyable element for assignment without moving"),
                    arena[raw_expr_idx].range
                )
            }
            EagerContract::EvalRef => todo!(),
        };
        self.infer_eager_expr(total_opds.start, this_contract, arena);
        for opd in (total_opds.start + 1)..total_opds.end {
            self.infer_eager_expr(opd, EagerContract::Pure, arena)
        }
        Ok(())
    }
}
