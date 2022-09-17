use husky_dev_utils::dev_src;
use husky_primitive_literal_syntax::PrimitiveLiteralData;
use husky_text::{RangedCustomIdentifier, TextRange};
use husky_word::CustomIdentifier;

use super::*;

pub trait AstBottomUpDeductor {
    type Expectation: Default + Copy;
    type Output: Copy;
    fn frame_var_output() -> Self::Output;
    fn loop_bound_expectation() -> Expectation<Self>;
    fn condition_expectation() -> Expectation<Self>;
    fn infer_primitive_literal(
        &self,
        expectation: Expectation<Self>,
        value: PrimitiveLiteralData,
    ) -> Self::Output;
    fn infer_entity_ty(
        &self,
        entity_route: EntityRoutePtr,
        husky_entity_kind: EntityKind,
    ) -> InferResult<Self::Output>;
    fn root_pure_binary_opn(
        &self,
        pure_binary_opr: PureBinaryOpr,
        lopd_builtin_ty: RootIdentifier,
        ropd_builtin_ty: RootIdentifier,
        range: TextRange,
    ) -> InferResult<Self::Output>;
    fn nodiscard_expectation() -> Expectation<Self>;
    fn this_value_output() -> InferResult<Self::Output>;
    fn this_field_output() -> InferResult<Self::Output>;
}

type Expectation<B: AstBottomUpDeductor> = Option<B::Output>;

pub struct AstBottomUpCacher<'a, D>
where
    D: AstBottomUpDeductor,
{
    arena: &'a RawExprArena,
    deductor: D,
}

impl<'a, D> AstBottomUpCacher<'a, D>
where
    D: AstBottomUpDeductor,
{
    fn save_var_output(
        &mut self,
        varname: CustomIdentifier,
        init_range: TextRange,
        output: D::Output,
    ) {
        todo!()
    }
    fn get_var_output(
        &mut self,
        varname: CustomIdentifier,
        init_range: TextRange,
    ) -> InferResult<D::Output> {
        todo!()
    }
    fn save_expr_result(&mut self, idx: RawExprIdx, result: InferResult<D::Output>) {
        todo!()
    }

    fn build_stmts(&mut self, ast_iter: AstIter, return_expectation: Expectation<D>) {
        self.enter_block();
        for item in ast_iter {
            if let Ok(ref value) = item.value {
                match value.variant {
                    AstVariant::Stmt(ref stmt) => match stmt.variant {
                        RawStmtVariant::Match { match_expr, .. } => {
                            let match_case_expectation =
                                self.infer_expr(match_expr, Default::default());
                            if let Some(children) = item.opt_children {
                                self.build_match_branches(
                                    children,
                                    return_expectation,
                                    match_case_expectation,
                                )
                            }
                        }
                        _ => {
                            self.build_stmt(stmt, return_expectation);
                            if let Some(children) = item.opt_children {
                                self.build_stmts(children, return_expectation)
                            }
                        }
                    },
                    _ => todo!(),
                }
            } else {
                if let Some(children) = item.opt_children {
                    self.build_stmts(children, return_expectation)
                }
            }
        }
        self.exit_block()
    }

    fn build_stmt(&mut self, stmt: &RawStmt, return_expectation: Expectation<D>) {
        match stmt.variant {
            RawStmtVariant::Loop(raw_loop_kind) => match raw_loop_kind {
                RawLoopKind::For {
                    frame_var,
                    initial_boundary,
                    final_boundary,
                    ..
                } => {
                    self.save_var_output(frame_var.ident, frame_var.range, D::frame_var_output());
                    self.build_loop_bound(initial_boundary);
                    self.build_loop_bound(final_boundary);
                }
                RawLoopKind::ForExt { final_boundary, .. } => self.build_loop_bound(final_boundary),
                RawLoopKind::While { condition } => self.build_condition(condition),
                RawLoopKind::DoWhile { condition } => self.build_condition(condition),
            },
            RawStmtVariant::ConditionBranch {
                condition_branch_kind,
            } => match condition_branch_kind {
                RawConditionBranchKind::If { condition } => self.build_condition(condition),
                RawConditionBranchKind::Elif { condition } => self.build_condition(condition),
                RawConditionBranchKind::Else => (),
            },
            RawStmtVariant::Require { condition, .. } => self.build_condition(condition),
            RawStmtVariant::PatternBranch {
                ref pattern_branch_variant,
            } => match pattern_branch_variant {
                RawPatternBranchVariant::Case { .. } => todo!(),
                RawPatternBranchVariant::Default => todo!(),
            },
            RawStmtVariant::Exec { expr, discard } => {
                self.infer_expr(
                    expr,
                    if discard {
                        Default::default()
                    } else {
                        D::nodiscard_expectation()
                    },
                );
            }
            RawStmtVariant::Init {
                varname,
                initial_value,
                ..
            } => {
                if let Some(output) = self.infer_expr(initial_value, Default::default()) {
                    self.save_var_output(varname.ident, varname.range, output)
                }
            }
            RawStmtVariant::Return { result, .. } => {
                self.infer_expr(result, return_expectation);
            }
            RawStmtVariant::Assert(condition) => self.build_condition(condition),
            RawStmtVariant::Break => (),
            RawStmtVariant::Match { .. } => unreachable!(),
            RawStmtVariant::ReturnXml(ref xml_expr) => match xml_expr.variant {
                RawXmlExprVariant::Value(raw_expr_idx) => {
                    self.infer_expr(raw_expr_idx, None);
                }
                RawXmlExprVariant::Tag { ref props, .. } => {
                    props.iter().for_each(|(_, argument)| {
                        self.infer_expr(*argument, None);
                    })
                }
            },
        }
    }

    fn enter_block(&mut self) {
        todo!()
    }
    fn exit_block(&mut self) {
        todo!()
    }
    fn build_match_branches(
        &mut self,
        branch_ast_iter: AstIter,
        opt_output_ty: Expectation<D>,
        opt_match_expr_ty: Expectation<D>,
    ) {
        todo!()
    }
    fn build_loop_bound(&mut self, boundary: RawBoundary) {
        if let Some(bound) = boundary.opt_bound {
            self.infer_expr(bound, D::loop_bound_expectation());
        }
    }

    fn build_condition(&mut self, condition: RawExprIdx) {
        self.infer_expr(condition, D::condition_expectation());
    }

    fn infer_expr(&mut self, idx: RawExprIdx, expectation: Expectation<D>) -> Expectation<D> {
        let ty_result = self.expr_ty_result(idx, expectation);
        let opt_ty = match ty_result {
            Ok(opd_ty) => Some(opd_ty),
            Err(_) => None,
        };
        self.save_expr_result(idx, ty_result);
        opt_ty
    }

    fn expr_ty_result(
        &mut self,
        idx: RawExprIdx,
        expectation: Expectation<D>,
    ) -> InferResult<D::Output> {
        let expr = &self.arena[idx];
        let ty = match expr.variant {
            RawExprVariant::Variable {
                varname,
                init_range,
            } => self.get_var_output(varname, init_range)?,
            RawExprVariant::Unrecognized(ident) => Err(InferError {
                variant: InferErrorVariant::Original {
                    message: format!("Unrecognized identifier `{}`", &ident),
                    range: self.arena[idx].range,
                },
                dev_src: dev_src!(),
            })?,
            RawExprVariant::Entity { route, kind } => self.deductor.infer_entity_ty(route, kind)?,
            RawExprVariant::PrimitiveLiteral(value) => {
                self.deductor.infer_primitive_literal(expectation, value)
            }
            RawExprVariant::Bracketed(expr) => {
                derived_not_none!(self.infer_expr(expr, expectation))?
            }
            RawExprVariant::Opn {
                opn_variant: ref opr,
                ref opds,
            } => self.infer_opn(idx, expectation, opr, opds)?,
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::ThisValue { opt_this_ty, .. } => D::this_value_output()?,
            RawExprVariant::FrameVariable { .. } => D::frame_var_output(),
            RawExprVariant::ThisField { opt_field_ty, .. } => D::this_field_output()?,
        };
        if let Some(expected_ty) = expectation {
            todo!()
            // if !self.db.is_implicitly_castable(ty, expected_ty) {
            //     throw!(
            //         format!("expect `{:?}` but get `{:?}` instead", expected_ty, ty),
            //         self.arena[idx].range
            //     )
            // }
        }
        Ok(ty)
    }

    fn infer_opn(
        &mut self,
        idx: RawExprIdx,
        expectation: Expectation<D>,
        opr: &RawOpnVariant,
        opds: &RawExprRange,
    ) -> InferResult<D::Output> {
        let range = self.arena[idx].range;
        match opr {
            RawOpnVariant::Binary(opr) => self.binary_opn(*opr, opds.start, opds.start + 1, range),
            RawOpnVariant::Prefix(opr) => self.infer_prefix(*opr, opds.start),
            RawOpnVariant::Suffix(opr) => self.infer_suffix(opr, opds.start),
            RawOpnVariant::List(opr) => self.list_opn_ty_result(idx, expectation, opr, opds, range),
            RawOpnVariant::Field(field_ident) => self.infer_field_access(*field_ident, opds.start),
        }
    }

    fn binary_opn(
        &mut self,
        opr: BinaryOpr,
        lopd: RawExprIdx,
        ropd: RawExprIdx,
        range: TextRange,
    ) -> InferResult<D::Output> {
        let lopd_ty = derived_not_none!(self.infer_expr(lopd, None))?;
        let ropd_ty = derived_not_none!(self.infer_expr(ropd, None))?;
        self.deductor.deduce_binary_opn(lopd_ty, ropd_ty)
    }

    fn infer_prefix(&mut self, opr: PrefixOpr, opd: RawExprIdx) -> InferResult<D::Output> {
        let opd_ty = derived_not_none!(self.infer_expr(opd, None,))?;
        self.deductor.deduce_prefix()
    }

    fn infer_suffix(&mut self, opr: &RawSuffixOpr, opd: RawExprIdx) -> InferResult<D::Output> {
        let opd_ty = derived_not_none!(self.infer_expr(opd, None,))?;
        self.deductor.deduce_suffix()
    }

    fn infer_field_access(
        &mut self,
        field_ident: RangedCustomIdentifier,
        opd: RawExprIdx,
    ) -> InferResult<D::Output> {
        let opd_ty = derived_not_none!(self.infer_expr(opd, None))?;
        derived_unwrap!(self.db.ty_decl(opd_ty.intrinsic())).field_ty_result(field_ident)
    }

    fn list_opn_ty_result(
        &mut self,
        idx: RawExprIdx,
        expectation: Expectation<D>,
        opr: &ListOpr,
        opds: &RawExprRange,
        range: TextRange,
    ) -> InferResult<D::Output> {
        msg_once!("expectation");
        match opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => self.infer_new_vec_from_list(expectation, opds),
            ListOpr::NewDict => todo!(),
            ListOpr::FunctionCall => self.infer_function_call(idx, opds),
            ListOpr::Index => self.infer_index(opds, range),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
            ListOpr::MethodCall { ranged_ident, .. } => {
                self.infer_method_call(opds.start, *ranged_ident, (opds.start + 1)..opds.end, idx)
            }
        }
    }

    fn infer_new_vec_from_list(
        &mut self,
        expectation: Expectation<D>,
        opds: &RawExprRange,
    ) -> InferResult<D::Output> {
        msg_once!("expectation");
        if opds.start == opds.end {
            // empty vec
            if let Some(expectation) = expectation {
                if expectation.variant
                    != (EntityRouteVariant::Root {
                        ident: RootIdentifier::Vec,
                    })
                {
                    todo!()
                }
                return Ok(expectation);
            } else {
                todo!()
            }
        }
        let opt_first_elem_ty = self.infer_expr(opds.start, None);
        let elem_ty = derived_not_none!(opt_first_elem_ty)?;
        for opd in (opds.start + 1)..opds.end {
            self.infer_expr(opd, Some(elem_ty));
        }
        Ok(self.db.vec(elem_ty))
    }

    fn infer_function_call(
        &mut self,
        idx: RawExprIdx,
        opds: &RawExprRange,
    ) -> InferResult<D::Output> {
        let caller = &self.arena[opds.start];
        let call_decl = match caller.variant {
            RawExprVariant::Entity { route, .. } => {
                self.entity_route_sheet
                    .function_call_routes
                    .insert_new(opds.start, Ok(route));
                self.db.entity_call_form_decl(route)
            }
            _ => self
                .db
                .value_call_form_decl(derived_not_none!(self.infer_expr(opds.start, None,))?),
        }
        .bind_with_text_ranged(caller)?;
        for (argument, parameter) in call_decl
            .match_primary(opds)
            .bind_with_text_ranged(caller)?
        {
            self.infer_expr(argument, Some(parameter.ty()));
        }
        for argument in call_decl
            .match_variadics(opds)
            .bind_with_text_ranged(caller)?
        {
            match call_decl.variadic_parameters {
                VariadicParametersDecl::None => panic!(),
                VariadicParametersDecl::RepeatSingle { ref parameter } => {
                    self.infer_expr(argument, Some(parameter.ty()));
                }
            }
        }
        Ok(call_decl.output.ty())
    }

    fn infer_method_call(
        &mut self,
        this: RawExprIdx,
        method_ident: RangedCustomIdentifier,
        arguments: RawExprRange,
        idx: RawExprIdx,
    ) -> InferResult<D::Output> {
        let this_deref_ty = derived_not_none!(self.infer_expr(this, None))?.deref_route();
        let this_deref_ty_decl = derived_unwrap!(self.db.ty_decl(this_deref_ty));
        let call_form_decl = this_deref_ty_decl.method(method_ident, &self.trait_uses)?;
        msg_once!("handle variadics");
        if call_form_decl.primary_parameters.len() != arguments.end - arguments.start {
            self.entity_route_sheet.extra_errors.push(error!(
                format!(
                    "expect {} parameters, but got {}",
                    call_form_decl.primary_parameters.len(),
                    arguments.end - arguments.start
                ),
                self.arena[idx].range
            ));
        }
        for (argument, parameter) in zip(
            arguments.into_iter(),
            call_form_decl.primary_parameters.iter(),
        ) {
            self.infer_expr(argument, Some(parameter.ty()));
        }
        let spatial_arguments: ThinVec<SpatialArgument> =
            if call_form_decl.spatial_parameters.len() > 0 {
                todo!()
            } else {
                thin_vec![]
            };
        msg_once!("spatial_arguments");
        self.entity_route_sheet.method_call_routes.insert_new(
            this,
            Ok(self
                .db
                .route_call(call_form_decl.opt_route.unwrap(), spatial_arguments)),
        );
        Ok(call_form_decl.output.ty())
    }

    fn infer_index(
        &mut self,
        total_opds: &RawExprRange,
        total_range: TextRange,
    ) -> InferResult<D::Output> {
        todo!()
        // if total_opds.end - total_opds.start < 2 {
        //     panic!()
        //     // throw!(format!("expect indices inside `[]`"), total_range);
        // }
        // if total_opds.end - total_opds.start > 2 {}
        // let this_ty = derived_not_none!(self.infer_expr(total_opds.start, None))?;
        // let index_ty = derived_not_none!(self.infer_expr(total_opds.start + 1, None))?;
        // let this_ty_decl = derived_unwrap!(self.db.ty_decl(this_ty));
        // let index_trai = self.db.intern_entity_route(EntityRoute {
        //     variant: self
        //         .db
        //         .entity_route_menu()
        //         .std_ops_index_trai
        //         .variant
        //         .clone(),
        //     temporal_arguments: thin_vec![],
        //     spatial_arguments: thin_vec![SpatialArgument::EntityRoute(index_ty)],
        // });
        // let trai_impl = ok_or!(
        //     this_ty_decl.trait_impl(index_trai),
        //     format!(
        //         "can't index by `{:?}` into type `{:?}`,\nit doesn't satisfy trait `{:?}`",
        //         index_ty, this_ty, index_trai
        //     ),
        //     total_range
        // )?;
        // Ok(match trai_impl.member_impls()[0] {
        //     TraitMemberImplDecl::AssociatedType { ty, .. } => ty,
        //     _ => panic!(),
        // })
    }
}
