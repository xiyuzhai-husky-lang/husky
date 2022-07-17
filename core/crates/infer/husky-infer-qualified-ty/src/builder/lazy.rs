use check_utils::should;
use defn_head::Parameter;
use entity_kind::EntityKind;
use husky_ast::*;
use husky_text::RangedCustomIdentifier;
use husky_text::TextRanged;
use infer_error::derived;
use infer_error::derived_not_none;
use infer_error::derived_unwrap;
use infer_error::throw_derived;
use print_utils::p;

use super::*;

impl<'a> QualifiedTySheetBuilder<'a> {
    pub(super) fn infer_lazy_call_form(
        &mut self,
        arena: &RawExprArena,
        inputs: &[Parameter],
        ast_iter: AstIter,
        opt_output_ty: Option<EntityRoutePtr>,
        output_liason: OutputLiason,
    ) {
        self.add_lazy_inputs(inputs);
        self.infer_lazy_stmts(arena, ast_iter, opt_output_ty, output_liason)
    }

    fn add_lazy_inputs(&mut self, inputs: &[Parameter]) {
        for input in inputs {
            let ty = input.ranged_ty.route;
            self.qualified_ty_sheet
                .lazy_variable_qualified_tys
                .insert_new((
                    (input.ranged_ident.ident.into(), input.ranged_ident.range),
                    self.db.is_copyable(ty).map(|is_copyable| {
                        LazyVariableQualifiedTy::new(
                            LazyVariableQualifier::parameter(
                                input.ranged_liason.liason,
                                is_copyable,
                            ),
                            ty,
                        )
                    }),
                ));
        }
    }

    fn infer_lazy_stmts(
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
                        self.infer_lazy_stmt(arena, stmt, opt_output_ty, output_liason)
                    }
                    _ => (),
                }
            }
            if let Some(children) = item.opt_children {
                self.infer_lazy_stmts(arena, children, opt_output_ty, output_liason)
            }
        }
    }

    fn infer_lazy_stmt(
        &mut self,
        arena: &RawExprArena,
        stmt: &RawStmt,
        opt_output_ty: Option<EntityRoutePtr>,
        output_liason: OutputLiason,
    ) {
        match stmt.variant {
            RawStmtVariant::Loop(_) => (),
            RawStmtVariant::ConditionBranch {
                condition_branch_kind,
            } => match condition_branch_kind {
                RawConditionBranchKind::If { condition } => {
                    self.infer_lazy_expr(arena, condition);
                }
                RawConditionBranchKind::Elif { condition } => {
                    self.infer_lazy_expr(arena, condition);
                }
                RawConditionBranchKind::Else => (),
            },
            RawStmtVariant::PatternBranch {
                ref pattern_branch_variant,
            } => match pattern_branch_variant {
                RawPatternBranchVariant::Case { ref pattern } => {
                    self.infer_lazy_case_pattern(pattern)
                }
                RawPatternBranchVariant::Default => (),
            },
            RawStmtVariant::Exec { .. } => (),
            RawStmtVariant::Init {
                init_kind,
                varname,
                initial_value,
            } => {
                if let Some(qt) = self.infer_lazy_expr(arena, initial_value) {
                    self.qualified_ty_sheet
                        .lazy_variable_qualified_tys
                        .insert_new((
                            (varname.ident.into(), varname.range),
                            qt.use_for_init(init_kind),
                        ));
                }
            }
            RawStmtVariant::Return { result, .. } => {
                match (opt_output_ty, self.infer_lazy_expr(arena, result)) {
                    (Some(output_ty), Some(qualified_ty)) => {
                        if !qualified_ty.is_implicitly_convertible_to_output(
                            self.db,
                            output_liason,
                            output_ty,
                        ) {
                            todo!()
                        }
                    }
                    _ => (),
                }
            }
            RawStmtVariant::Assert(condition) => {
                self.infer_lazy_expr(arena, condition);
            }
            RawStmtVariant::Break => todo!(),
            RawStmtVariant::Match { match_expr, .. } => {
                self.infer_lazy_expr(arena, match_expr);
            }
            RawStmtVariant::ReturnXml(ref xml_expr) => match xml_expr.variant {
                RawXmlExprVariant::Value(raw_expr_idx) => {
                    self.infer_lazy_expr(arena, raw_expr_idx);
                }
                RawXmlExprVariant::Tag { ident, ref props } => {
                    props.iter().for_each(|(_, argument)| {
                        self.infer_lazy_expr(arena, *argument);
                    })
                }
            },
        }
    }

    fn infer_lazy_case_pattern(&mut self, pattern: &CasePattern) {
        match pattern.variant {
            CasePatternVariant::PrimitiveLiteral(_) => (),
            CasePatternVariant::OneOf { ref patterns } => (),
            CasePatternVariant::EnumLiteral(_) => (),
        }
    }

    fn infer_lazy_expr(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
    ) -> Option<LazyValueQualifiedTy> {
        let qualified_qualified_ty_result = self.lazy_expr(arena, raw_expr_idx);
        let opt_qualified_ty = qualified_qualified_ty_result
            .as_ref()
            .map(|qualified_ty| *qualified_ty)
            .ok();
        self.qualified_ty_sheet
            .lazy_expr_qualified_tys
            .insert_new(raw_expr_idx, qualified_qualified_ty_result);
        opt_qualified_ty
    }

    fn lazy_expr(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<LazyValueQualifiedTy> {
        let raw_expr = &arena[raw_expr_idx];
        let ty = self.raw_expr_deref_ty(raw_expr_idx)?;
        match raw_expr.variant {
            RawExprVariant::Variable {
                varname,
                init_range,
            } => match derived_not_none!(self
                .qualified_ty_sheet
                .lazy_variable_qualified_tys
                .get_entry((varname.into(), init_range)))?
            .1
            {
                Ok(variable_qt) => {
                    let variable_contract = self.lazy_expr_contract(raw_expr_idx)?;
                    Ok(LazyValueQualifiedTy {
                        qual: variable_qt.qual.variable_use(variable_contract)?,
                        ty: variable_qt.ty,
                    })
                }
                Err(ref e) => Err(e.derived()),
            },
            RawExprVariant::FrameVariable { .. } => throw_derived!("no frame variable in lazy"),
            RawExprVariant::ThisValue {
                opt_this_ty: opt_ty,
                opt_this_liason,
            } => {
                let ty = derived_not_none!(opt_ty)?;
                let this_liason = derived_not_none!(opt_this_liason)?;
                let this_contract = self.lazy_expr_contract(raw_expr_idx)?;
                LazyValueQualifiedTy::parameter_use_lazy_qualified_ty(
                    self.db,
                    this_liason,
                    ty,
                    this_contract,
                )
            }
            RawExprVariant::Unrecognized(_) => throw_derived!("unrecognized"),
            RawExprVariant::Entity { route, kind } => match kind {
                EntityKind::Module => Ok(LazyValueQualifiedTy::module_lazy_qualified_ty()),
                EntityKind::Type(_) => Ok(LazyValueQualifiedTy::ty_lazy_qualified_ty()),
                EntityKind::Trait => Ok(LazyValueQualifiedTy::trait_lazy_qualified_ty()),
                EntityKind::Feature => Ok(LazyValueQualifiedTy::new(
                    LazyExprQualifier::feature(self.db.is_copyable(ty)?),
                    ty,
                )),
                EntityKind::Member(_) | EntityKind::Function { .. } => todo!(),
                EntityKind::EnumLiteral => {
                    Ok(LazyValueQualifiedTy::new(LazyExprQualifier::Copyable, ty))
                }
                EntityKind::Main => panic!(),
            },
            RawExprVariant::CopyableLiteral(_) => Ok(LazyValueQualifiedTy::new(
                LazyExprQualifier::Copyable,
                self.raw_expr_deref_ty(raw_expr_idx).unwrap(),
            )),
            RawExprVariant::Bracketed(bracketed_expr) => {
                derived_not_none!(self.infer_lazy_expr(arena, bracketed_expr))
            }
            RawExprVariant::Opn {
                opn_variant: ref opr,
                ref opds,
            } => self.lazy_opn(arena, raw_expr_idx, opr, opds.clone()),
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::ThisField {
                opt_this_ty,
                opt_this_liason,
                field_ident: ident,
                field_liason,
                opt_field_ty,
            } => {
                let this_ty = derived_not_none!(opt_this_ty)?;
                let this_liason = derived_not_none!(opt_this_liason)?;
                let field_contract = self.lazy_expr_contract(raw_expr_idx)?;
                let field_ty = derived_not_none!(opt_field_ty)?;
                let is_field_copyable = self.db.is_copyable(field_ty.route)?;
                let this_contract = LazyContract::field_access_lazy_contract(
                    field_liason,
                    field_contract,
                    is_field_copyable,
                    arena[raw_expr_idx].range,
                )?;
                let this_qual = LazyExprQualifier::parameter_use_lazy_qualifier(
                    this_liason,
                    self.db.is_copyable(this_ty)?,
                    this_contract,
                )?;
                Ok(LazyValueQualifiedTy::member_lazy_qualified_ty(
                    self.db,
                    this_qual,
                    field_ty.route,
                    field_contract,
                )?)
            }
        }
    }

    fn lazy_opn(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        opr: &RawOpnVariant,
        opds: RawExprRange,
    ) -> InferResult<LazyValueQualifiedTy> {
        match opr {
            RawOpnVariant::Binary(binary_opr) => self.lazy_binary(arena, raw_expr_idx, opds),
            RawOpnVariant::Prefix(prefix_opr) => self.lazy_prefix(arena, raw_expr_idx, opds),
            RawOpnVariant::Suffix(suffix_opr) => {
                self.lazy_suffix(arena, raw_expr_idx, *suffix_opr, opds)
            }
            RawOpnVariant::List(list_opr) => self.lazy_list(arena, raw_expr_idx, list_opr, opds),
            RawOpnVariant::FieldAccess(field_ident) => {
                self.lazy_field_access(arena, raw_expr_idx, *field_ident, opds)
            }
        }
    }

    fn lazy_binary(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        opds: RawExprRange,
    ) -> InferResult<LazyValueQualifiedTy> {
        self.infer_lazy_expr(arena, opds.start);
        self.infer_lazy_expr(arena, opds.start + 1);
        Ok(LazyValueQualifiedTy::new(
            LazyExprQualifier::Transient,
            self.raw_expr_deref_ty(raw_expr_idx)?,
        ))
    }

    fn lazy_prefix(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        opds: RawExprRange,
    ) -> InferResult<LazyValueQualifiedTy> {
        self.infer_lazy_expr(arena, opds.start);
        Ok(LazyValueQualifiedTy::new(
            LazyExprQualifier::Transient,
            self.raw_expr_deref_ty(raw_expr_idx)?,
        ))
    }

    fn lazy_suffix(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        opr: SuffixOpr,
        opds: RawExprRange,
    ) -> InferResult<LazyValueQualifiedTy> {
        let this_qt = derived_not_none!(self.infer_lazy_expr(arena, opds.start))?;
        let this_ty_decl = derived_unwrap!(self.db.ty_decl(this_qt.ty));
        match opr {
            SuffixOpr::Incr | SuffixOpr::Decr => {
                throw_derived!(format!("mutation not allowed in lazy functional context"))
            }
            SuffixOpr::AsTy(_) => Ok(this_qt),
        }
    }

    fn lazy_field_access(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        field_ident: RangedCustomIdentifier,
        opds: RawExprRange,
    ) -> InferResult<LazyValueQualifiedTy> {
        let this_qt = derived_not_none!(self.infer_lazy_expr(arena, opds.start))?;
        let this_ty_decl = derived_unwrap!(self.db.ty_decl(this_qt.ty));
        let field_decl = this_ty_decl.field_decl(field_ident)?;
        let field_contract = self.lazy_expr_contract(raw_expr_idx)?;
        let qual = LazyExprQualifier::member_lazy_qualifier(
            this_qt.qual,
            self.db.is_copyable(field_decl.ty)?,
            field_contract,
        )?;
        Ok(LazyValueQualifiedTy::new(qual, field_decl.ty))
    }

    fn lazy_list(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        list_opr: &ListOpr,
        opds: RawExprRange,
    ) -> InferResult<LazyValueQualifiedTy> {
        match list_opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.lazy_call(arena, raw_expr_idx, opds),
            ListOpr::Index | ListOpr::ModuloIndex => {
                self.lazy_element_access(arena, raw_expr_idx, opds)
            }
            ListOpr::StructInit => todo!(),
            ListOpr::MethodCall { ranged_ident, .. } => self.lazy_method_call(
                arena,
                opds.start,
                *ranged_ident,
                (opds.start + 1)..opds.end,
                raw_expr_idx,
            ),
        }
    }

    fn lazy_call(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        total_opds: RawExprRange,
    ) -> InferResult<LazyValueQualifiedTy> {
        match arena[total_opds.start].variant {
            RawExprVariant::Entity { route, .. } => {
                let call_decl = derived_unwrap!(self.db.function_decl(route));
                let opt_opd_qualified_tys: Vec<_> = ((total_opds.start + 1)..total_opds.end)
                    .into_iter()
                    .map(|opd_idx| self.infer_lazy_expr(arena, opd_idx))
                    .collect();
                match call_decl.output.liason {
                    OutputLiason::Transfer => {
                        msg_once!("handle ref");
                        Ok(LazyValueQualifiedTy::new(
                            if self.db.is_copyable(call_decl.output.ty)? {
                                LazyExprQualifier::Copyable
                            } else {
                                LazyExprQualifier::Transient
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
            RawExprVariant::CopyableLiteral(_) => {
                throw_derived!("a primitive literal can't be a caller")
            }
            _ => {
                p!(arena[total_opds.start].variant);
                todo!()
            }
        }
    }

    fn lazy_element_access(
        &mut self,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
        total_opds: RawExprRange,
    ) -> InferResult<LazyValueQualifiedTy> {
        let this_qt = derived_not_none!(self.infer_lazy_expr(arena, total_opds.start))?;
        let this_contract = self.lazy_expr_contract(total_opds.start)?;
        for opd in (total_opds.start + 1)..total_opds.end {
            self.infer_lazy_expr(arena, opd);
        }
        let element_ty = self.raw_expr_deref_ty(raw_expr_idx)?;
        let element_contract = self.lazy_expr_contract(raw_expr_idx)?;
        LazyValueQualifiedTy::member_lazy_qualified_ty(
            self.db,
            this_qt.qual,
            element_ty,
            element_contract,
        )
    }

    fn lazy_method_call(
        &mut self,
        arena: &RawExprArena,
        this: RawExprIdx,
        method_ident: RangedCustomIdentifier,
        inputs: RawExprRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<LazyValueQualifiedTy> {
        let method_decl = self.method_decl(raw_expr_idx)?;
        self.infer_lazy_expr(arena, this);
        for input in inputs {
            self.infer_lazy_expr(arena, input);
        }
        let qual = match method_decl.output.liason {
            OutputLiason::Transfer => {
                if self.db.is_copyable(method_decl.output.ty)? {
                    LazyExprQualifier::Copyable
                } else {
                    LazyExprQualifier::Transient
                }
            }
            OutputLiason::MemberAccess { .. } => todo!(),
        };
        Ok(LazyValueQualifiedTy::new(qual, method_decl.output.ty))
    }
}
