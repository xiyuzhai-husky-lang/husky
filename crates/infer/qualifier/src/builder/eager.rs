use std::iter::zip;

use ast::*;
use check_utils::should;
use defn_head::InputParameter;
use entity_kind::EntityKind;
use entity_route::EntityRoutePtr;
use infer_error::{
    derived, derived_not_none, derived_unwrap, throw, throw_derived, InferError, InferErrorVariant,
};
use print_utils::{emsg_once, epin, p};
use text::{BindTextRangeInto, RangedCustomIdentifier};
use text::{TextRange, TextRanged};

use super::*;

impl<'a> QualifiedTySheetBuilder<'a> {
    pub(super) fn infer_eager_call_form(
        &mut self,
        arena: &RawExprArena,
        inputs: &[InputParameter],
        ast_iter: AstIter,
        opt_output_ty: Option<EntityRoutePtr>,
        output_liason: OutputLiason,
    ) {
        self.add_eager_inputs(inputs);
        self.infer_eager_stmts(arena, ast_iter, opt_output_ty, output_liason)
    }

    fn add_eager_inputs(&mut self, inputs: &[InputParameter]) {
        for input in inputs {
            let ty = input.ranged_ty.route;
            self.qualified_ty_sheet
                .eager_variable_qualified_tys
                .insert_new((
                    (input.ranged_ident.ident.into(), input.ranged_ident.range),
                    (|| -> InferResult<_> {
                        (Ok(EagerQualifiedTy::new(
                            EagerQualifier::from_parameter(input.liason, self.db.is_copyable(ty)?),
                            ty,
                        )))
                    })(),
                ));
        }
    }

    fn infer_eager_stmts(
        &mut self,
        arena: &RawExprArena,
        ast_iter: AstIter,
        opt_output_ty: Option<EntityRoutePtr>,
        output_liason: OutputLiason,
    ) {
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.variant {
                    AstVariant::Stmt(ref stmt) => {
                        self.infer_eager_stmt(arena, stmt, opt_output_ty, output_liason)
                    }
                    _ => (),
                }
            }
            if let Some(children) = item.opt_children {
                self.infer_eager_stmts(arena, children, opt_output_ty, output_liason)
            }
        }
    }

    fn infer_eager_stmt(
        &mut self,
        arena: &RawExprArena,
        stmt: &RawStmt,
        opt_output_ty: Option<EntityRoutePtr>,
        output_liason: OutputLiason,
    ) {
        match stmt.variant {
            RawStmtVariant::Loop(loop_kind) => match loop_kind {
                RawLoopKind::For {
                    frame_var,
                    initial_boundary,
                    final_boundary,
                    ..
                } => {
                    self.qualified_ty_sheet
                        .eager_variable_qualified_tys
                        .insert_new((
                            (frame_var.ident.into(), frame_var.range),
                            Ok(EagerQualifiedTy {
                                qual: EagerQualifier::Copyable,
                                ty: EntityRoutePtr::Root(RootIdentifier::I32),
                            }),
                        ));
                    if let Some(bound) = initial_boundary.opt_bound {
                        self.infer_eager_expr(arena, bound);
                    }
                    if let Some(bound) = final_boundary.opt_bound {
                        self.infer_eager_expr(arena, bound);
                    }
                }
                RawLoopKind::ForExt { final_boundary, .. } => {
                    if let Some(bound) = final_boundary.opt_bound {
                        self.infer_eager_expr(arena, bound);
                    }
                }
                RawLoopKind::While { condition } => {
                    self.infer_eager_expr(arena, condition);
                }
                RawLoopKind::DoWhile { condition } => {
                    self.infer_eager_expr(arena, condition);
                }
            },
            RawStmtVariant::ConditionBranch {
                condition_branch_kind,
            } => match condition_branch_kind {
                RawConditionBranchKind::If { condition } => {
                    self.infer_eager_expr(arena, condition);
                }
                RawConditionBranchKind::Elif { condition } => {
                    self.infer_eager_expr(arena, condition);
                }
                RawConditionBranchKind::Else => (),
            },
            RawStmtVariant::PatternBranch {
                ref pattern_branch_variant,
            } => match pattern_branch_variant {
                RawPatternBranchVariant::Case { pattern } => self.infer_eager_case_pattern(pattern),
                RawPatternBranchVariant::Default => (),
            },
            RawStmtVariant::Exec {
                expr,
                discard: silent,
            } => {
                self.infer_eager_expr(arena, expr);
            }
            RawStmtVariant::Init {
                init_kind,
                varname,
                initial_value,
            } => {
                if let Some(initial_value_qualified_ty) =
                    self.infer_eager_expr(arena, initial_value)
                {
                    self.qualified_ty_sheet
                        .eager_variable_qualified_tys
                        .insert_new((
                            (varname.ident.into(), varname.range),
                            initial_value_qualified_ty.init_variable_qualified_ty(init_kind),
                        ))
                }
            }
            RawStmtVariant::Return(expr) => {
                match (opt_output_ty, self.infer_eager_expr(arena, expr)) {
                    (Some(output_ty), Some(qualified_ty)) => {
                        if !qualified_ty.is_implicitly_castable_to_output(
                            self.db,
                            output_liason,
                            output_ty,
                        ) {
                            self.qualified_ty_sheet.extra_errors.push(InferError {
                                variant: InferErrorVariant::Original {
                                    message: format!(
                                        "expect return type to be `{:?}`,\n  but got `{:?}` instead",
                                   output_ty,qualified_ty),
                                    range: stmt.range,
                                },
                                dev_src: dev_utils::dev_src!(),
                            })
                        }
                    }
                    _ => (),
                }
            }
            RawStmtVariant::Assert(condition) => {
                self.infer_eager_expr(arena, condition);
            }
            RawStmtVariant::Break => (),
            RawStmtVariant::Match { match_expr, .. } => {
                self.infer_eager_expr(arena, match_expr);
            }
            RawStmtVariant::ReturnXml(ref xml_expr) => match xml_expr.variant {
                RawXmlExprVariant::Value(raw_expr_idx) => {
                    self.infer_eager_expr(arena, raw_expr_idx);
                }
                RawXmlExprVariant::Tag { ident, ref props } => {
                    props.iter().for_each(|(_, argument)| {
                        self.infer_eager_expr(arena, *argument);
                    })
                }
            },
        }
    }

    fn infer_eager_case_pattern(&mut self, pattern: &CasePattern) {
        match pattern.variant {
            CasePatternVariant::PrimitiveLiteral(_) => (),
            CasePatternVariant::OneOf { ref patterns } => (),
            CasePatternVariant::EnumLiteral(_) => (),
        }
    }

    pub(super) fn infer_eager_expr(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
    ) -> Option<EagerQualifiedTy> {
        let qualified_qualified_ty_result = self.eager_expr(raw_expr_idx, arena);
        let opt_qualified_ty = qualified_qualified_ty_result
            .as_ref()
            .map(|qualified_ty| *qualified_ty)
            .ok();
        self.qualified_ty_sheet
            .eager_expr_qualified_tys
            .insert_new(raw_expr_idx, qualified_qualified_ty_result);
        opt_qualified_ty
    }

    fn eager_expr(
        &mut self,
        raw_expr_idx: RawExprIdx,
        arena: &RawExprArena,
    ) -> InferResult<EagerQualifiedTy> {
        let raw_expr = &arena[raw_expr_idx];
        match raw_expr.variant {
            RawExprVariant::Variable {
                varname,
                init_range,
            } => match derived_not_none!(self
                .qualified_ty_sheet
                .eager_variable_qualified_tys
                .get_entry((varname.into(), init_range)))?
            .1
            {
                Ok(variable_qt) => {
                    let variable_contract = self.eager_expr_contract(raw_expr_idx)?;
                    Ok(EagerQualifiedTy {
                        qual: variable_qt.qual.variable_use(variable_contract)?,
                        ty: variable_qt.ty,
                    })
                }
                Err(ref e) => Err(e.derived()),
            },
            RawExprVariant::FrameVariable { .. } => Ok(EagerQualifiedTy::new(
                EagerQualifier::Copyable,
                EntityRoutePtr::Root(RootIdentifier::I32),
            )),
            RawExprVariant::ThisValue {
                opt_this_ty,
                opt_this_liason,
            } => {
                let this_ty = derived_not_none!(opt_this_ty)?;
                let this_liason = derived_not_none!(opt_this_liason)?;
                let this_contract = self.eager_expr_contract(raw_expr_idx)?;
                EagerQualifiedTy::from_parameter_use(self.db, this_liason, this_ty, this_contract)
            }
            RawExprVariant::ThisField {
                opt_this_ty,
                opt_this_liason,
                field_ident: ident,
                field_liason,
                opt_field_ty,
            } => {
                let this_ty = derived_not_none!(opt_this_ty)?;
                let this_liason = derived_not_none!(opt_this_liason)?;
                let field_contract = self.eager_expr_contract(raw_expr_idx)?;
                let field_ty = derived_not_none!(opt_field_ty)?;
                let is_field_copyable = self.db.is_copyable(field_ty.route)?;
                let this_contract_result: InferResult<_> = field_liason
                    .this_eager_contract(field_contract, is_field_copyable)
                    .bind_into(&arena[raw_expr_idx]);
                let this_contract = this_contract_result?;
                let this_qual = EagerQualifier::from_parameter_use(
                    this_liason,
                    self.db.is_copyable(this_ty)?,
                    this_contract,
                )?;
                Ok(EagerQualifiedTy::new(
                    EagerQualifier::from_field(this_qual, field_liason, is_field_copyable)?,
                    field_ty.route,
                ))
            }
            RawExprVariant::Unrecognized(_) => Err(derived!("unrecognized")),
            RawExprVariant::Entity { route, kind } => match kind {
                EntityKind::Module => todo!(),
                EntityKind::Type(_) => Ok(EagerQualifiedTy::ty_qualified_ty()),
                EntityKind::Trait => todo!(),
                EntityKind::Member(_) => todo!(),
                EntityKind::Function { .. } => todo!(),
                EntityKind::Feature => todo!(),
                EntityKind::EnumLiteral => Ok(EagerQualifiedTy::new(
                    EagerQualifier::Copyable,
                    self.raw_expr_ty(raw_expr_idx)?,
                )),
                EntityKind::Main => panic!(),
            },
            RawExprVariant::CopyableLiteral(_) => Ok(EagerQualifiedTy::new(
                EagerQualifier::Copyable,
                self.raw_expr_ty(raw_expr_idx).unwrap(),
            )),
            RawExprVariant::Bracketed(expr) => {
                derived_not_none!(self.infer_eager_expr(arena, expr))
            }
            RawExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.eager_opn(
                arena,
                raw_expr_idx,
                opn_variant,
                opds.clone(),
                raw_expr.range,
            ),
            RawExprVariant::Lambda(_, _) => todo!(),
        }
    }

    fn eager_opn(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        opr: &RawOpnVariant,
        opds: RawExprRange,
        range: TextRange,
    ) -> InferResult<EagerQualifiedTy> {
        match opr {
            RawOpnVariant::Binary(binary_opr) => {
                self.eager_binary(arena, raw_expr_idx, *binary_opr, opds, range)
            }
            RawOpnVariant::Prefix(prefix_opr) => self.eager_prefix(arena, raw_expr_idx, opds),
            RawOpnVariant::Suffix(suffix_opr) => {
                self.eager_suffix(arena, raw_expr_idx, *suffix_opr, opds)
            }
            RawOpnVariant::List(list_opr) => self.eager_list(arena, raw_expr_idx, list_opr, opds),
            RawOpnVariant::FieldAccess(field_ident) => {
                self.eager_field_access(arena, raw_expr_idx, *field_ident, opds)
            }
        }
    }

    fn eager_binary(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        opr: BinaryOpr,
        opds: RawExprRange,
        range: TextRange,
    ) -> InferResult<EagerQualifiedTy> {
        let this_qt = derived_not_none!(self.infer_eager_expr(arena, opds.start))?;
        match opr {
            BinaryOpr::Pure(_) => (),
            BinaryOpr::Assign(_) => match this_qt.qual {
                EagerQualifier::Copyable
                | EagerQualifier::PureRef
                | EagerQualifier::GlobalRef
                | EagerQualifier::LocalRef
                | EagerQualifier::Transient
                | EagerQualifier::Owned => throw!("lopd is not mutable", range),
                EagerQualifier::CopyableMut
                | EagerQualifier::OwnedMut
                | EagerQualifier::LocalRefMut => (),
            },
        }
        self.infer_eager_expr(arena, opds.start + 1);
        let ty = self.raw_expr_ty(raw_expr_idx)?;
        Ok(EagerQualifiedTy::new(
            if self.db.is_copyable(ty)? {
                EagerQualifier::Copyable
            } else {
                EagerQualifier::Transient
            },
            ty,
        ))
    }

    fn eager_prefix(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        opds: RawExprRange,
    ) -> InferResult<EagerQualifiedTy> {
        self.infer_eager_expr(arena, opds.start);
        let ty = self.raw_expr_ty(raw_expr_idx)?;
        Ok(EagerQualifiedTy::new(
            EagerQualifier::transitive(self.db.is_copyable(ty)?),
            self.raw_expr_ty(raw_expr_idx)?,
        ))
    }

    fn eager_suffix(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        opr: SuffixOpr,
        opds: RawExprRange,
    ) -> InferResult<EagerQualifiedTy> {
        let this_qt = derived_not_none!(self.infer_eager_expr(arena, opds.start))?;
        let this_ty_decl = derived_unwrap!(self.db.ty_decl(this_qt.ty));
        match opr {
            SuffixOpr::Incr | SuffixOpr::Decr => Ok(EagerQualifiedTy {
                qual: EagerQualifier::Copyable,
                ty: EntityRoutePtr::Root(RootIdentifier::Void),
            }),
            SuffixOpr::WithTy(_) => todo!(),
            SuffixOpr::AsTy(ranged_ty) => this_qt.as_ty(self.db, ranged_ty.route),
        }
    }

    fn eager_field_access(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        field_ident: RangedCustomIdentifier,
        opds: RawExprRange,
    ) -> InferResult<EagerQualifiedTy> {
        let this_qt = derived_not_none!(self.infer_eager_expr(arena, opds.start))?;
        let this_ty_decl = derived_unwrap!(self.db.ty_decl(this_qt.ty));
        let field_decl = this_ty_decl.field_decl(field_ident)?;
        let qual = EagerQualifier::from_field(
            this_qt.qual,
            field_decl.liason,
            self.db.is_copyable(field_decl.ty)?,
        )?;
        Ok(EagerQualifiedTy::new(qual, field_decl.ty))
    }

    fn eager_list(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        list_opr: &ListOpr,
        opds: RawExprRange,
    ) -> InferResult<EagerQualifiedTy> {
        match list_opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.eager_call(arena, raw_expr_idx, opds),
            ListOpr::Index => self.eager_element_access(arena, raw_expr_idx, opds),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
            ListOpr::MethodCall { ranged_ident, .. } => self.eager_method_call(
                arena,
                opds.start,
                *ranged_ident,
                (opds.start + 1)..opds.end,
                raw_expr_idx,
            ),
        }
    }

    fn eager_call(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        total_opds: RawExprRange,
    ) -> InferResult<EagerQualifiedTy> {
        match arena[total_opds.start].variant {
            RawExprVariant::Entity { route, .. } => {
                let call_decl = derived_unwrap!(self.db.call_decl(route));
                let opt_opd_qualified_tys = zip(
                    ((total_opds.start + 1)..total_opds.end).into_iter(),
                    call_decl.primary_parameters.iter(),
                )
                .map(
                    |(opd_idx, parameter)| -> InferResult<Option<EagerQualifiedTy>> {
                        if let Some(qt) = self.infer_eager_expr(arena, opd_idx) {
                            match parameter.liason {
                                InputLiason::Pure => Ok(Some(qt)),
                                InputLiason::GlobalRef => todo!(),
                                InputLiason::LocalRefMut => todo!(),
                                InputLiason::Move | InputLiason::MoveMut => match qt.qual {
                                    EagerQualifier::Copyable | EagerQualifier::CopyableMut => {
                                        panic!()
                                    }
                                    EagerQualifier::PureRef => {
                                        throw!(
                                            format!("can't move a pure ref to owned"),
                                            arena[opd_idx].range
                                        )
                                    }
                                    EagerQualifier::Owned
                                    | EagerQualifier::OwnedMut
                                    | EagerQualifier::Transient => Ok(Some(qt)),
                                    EagerQualifier::GlobalRef => todo!(),
                                    EagerQualifier::LocalRef => todo!(),
                                    EagerQualifier::LocalRefMut => todo!(),
                                },
                                InputLiason::MemberAccess => todo!(),
                            }
                        } else {
                            Ok(None)
                        }
                    },
                )
                .collect::<InferResult<Vec<_>>>()?;
                match call_decl.output.liason {
                    OutputLiason::Transfer => {
                        emsg_once!("handle ref");
                        Ok(EagerQualifiedTy::new(
                            if self.db.is_copyable(call_decl.output.ty)? {
                                EagerQualifier::Copyable
                            } else {
                                EagerQualifier::Transient
                            },
                            call_decl.output.ty,
                        ))
                    }
                    OutputLiason::MemberAccess { .. } => todo!(),
                }
            }
            RawExprVariant::Opn {
                opn_variant: ref opr,
                ref opds,
            } => todo!(),
            RawExprVariant::Unrecognized(_) => Err(derived!("unrecognized caller")),
            RawExprVariant::CopyableLiteral(_) => {
                throw_derived!("a primitive literal can't be a caller")
            }
            _ => {
                p!(arena[total_opds.start].variant);
                todo!()
            }
        }
    }

    fn eager_element_access(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        total_opds: RawExprRange,
    ) -> InferResult<EagerQualifiedTy> {
        let this_qt = derived_not_none!(self.infer_eager_expr(arena, total_opds.start))?;
        let this_contract = self.eager_expr_contract(total_opds.start)?;
        for opd in (total_opds.start + 1)..total_opds.end {
            self.infer_eager_expr(arena, opd);
        }
        let element_ty = self.raw_expr_ty(raw_expr_idx)?;
        let qual = EagerQualifier::element_access_qual(
            this_qt.qual,
            this_contract,
            self.db.is_copyable(element_ty)?,
        );
        Ok(EagerQualifiedTy::new(qual, element_ty))
    }

    fn eager_method_call(
        &mut self,
        arena: &RawExprArena,
        this: RawExprIdx,
        method_ident: RangedCustomIdentifier,
        inputs: RawExprRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EagerQualifiedTy> {
        let method_decl = self.method_decl(raw_expr_idx)?;
        let this_qt = derived_not_none!(self.infer_eager_expr(arena, this))?;
        let this_contract = self.eager_expr_contract(this)?;
        for input in inputs {
            self.infer_eager_expr(arena, input);
        }
        let is_element_copyable = self.db.is_copyable(method_decl.output.ty)?;
        let qual = match method_decl.output.liason {
            OutputLiason::Transfer => {
                if is_element_copyable {
                    EagerQualifier::Copyable
                } else {
                    EagerQualifier::Transient
                }
            }
            OutputLiason::MemberAccess { .. } => EagerQualifier::element_access_qual(
                this_qt.qual,
                this_contract,
                is_element_copyable,
            ),
        };
        Ok(EagerQualifiedTy::new(qual, method_decl.output.ty))
    }
}
