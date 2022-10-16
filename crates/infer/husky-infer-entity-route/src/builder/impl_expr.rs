use std::iter::zip;

use super::*;
use husky_ast::RawExprRange;
use husky_dev_utils::dev_src;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_text::*;
use infer_decl::{TraitMemberImplDecl, VariadicParametersDecl};
use thin_vec::{thin_vec, ThinVec};

impl<'a> EntityRouteSheetBuilder<'a> {
    pub(super) fn infer_expr(
        &mut self,
        idx: RawExprIdx,
        expectation: Option<EntityRoutePtr>,
    ) -> Option<EntityRoutePtr> {
        let ty_result: InferResult<EntityRoutePtr> = self.expr_ty_result(idx, expectation);
        let opt_ty = match ty_result {
            Ok(opd_ty) => Some(opd_ty),
            Err(_) => None,
        };
        self.entity_route_sheet.expr_tys.insert_new(idx, ty_result);
        opt_ty
    }

    pub(super) fn expr_ty_result(
        &mut self,
        idx: RawExprIdx,
        expectation: Option<EntityRoutePtr>,
    ) -> InferResult<EntityRoutePtr> {
        let expr = &self.arena[idx];
        let ty = match expr.variant {
            RawExprVariant::Variable {
                varname,
                init_range,
            } => derived_not_none!(self
                .entity_route_sheet
                .variable_tys
                .get(&(varname, init_range))
                .map(|route| *route))?,
            RawExprVariant::Unrecognized(ident) => Err(InferError {
                variant: InferErrorVariant::Original {
                    message: format!("Unrecognized identifier `{}`", &ident),
                    range: self.arena[idx].range,
                },
                dev_src: dev_src!(),
            })?,
            RawExprVariant::Entity { route, kind } => self.infer_entity_ty(route, kind)?,
            RawExprVariant::PrimitiveLiteral(value) => {
                self.infer_primitive_literal(expectation, value)
            }
            RawExprVariant::Bracketed(expr) => {
                derived_not_none!(self.infer_expr(expr, expectation))?
            }
            RawExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.infer_opn(idx, expectation, opn_variant, opds)?,
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::ThisValue {
                opt_this_ty: opt_ty,
                ..
            } => derived_not_none!(opt_ty)?,
            RawExprVariant::FrameVariable { .. } => self.db.entity_route_menu().i32_ty,
            RawExprVariant::ThisField { opt_field_ty, .. } => {
                derived_not_none!(opt_field_ty)?.route
            }
        };
        if let Some(expected_ty) = expectation {
            if !self.db.is_implicitly_castable(ty, expected_ty) {
                throw!(
                    format!("expect `{:?}` but get `{:?}` instead", expected_ty, ty),
                    self.arena[idx].range
                )
            }
        }
        Ok(ty)
    }

    fn infer_entity_ty(
        &self,
        entity_route: EntityRoutePtr,
        husky_entity_kind: EntityKind,
    ) -> InferResult<EntityRoutePtr> {
        Ok(match husky_entity_kind {
            EntityKind::Module => EntityRoutePtr::Root(RootBuiltinIdentifier::Module),
            EntityKind::EnumVariant => match entity_route {
                EntityRoutePtr::Root(ident) => match ident {
                    RootBuiltinIdentifier::True | RootBuiltinIdentifier::False => {
                        RootBuiltinIdentifier::Bool.into()
                    }
                    _ => panic!(),
                },
                EntityRoutePtr::Custom(route) => route.parent(),
            },
            EntityKind::Type(_) => RootBuiltinIdentifier::TypeType.into(),
            EntityKind::Trait => RootBuiltinIdentifier::Trait.into(),
            EntityKind::Function { .. } | EntityKind::Member(_) => {
                let decl = self.db.entity_call_form_decl(entity_route)?;
                let base_route: EntityRoutePtr = if decl.is_lazy {
                    RootBuiltinIdentifier::Mor
                } else {
                    RootBuiltinIdentifier::ThickFp
                }
                .into();
                msg_once!("handle temporal/spatial parameters");
                msg_once!("make this into salsa db");
                let mut spatial_arguments: ThinVec<SpatialArgument> = decl
                    .primary_parameters
                    .iter()
                    .map(|parameter| parameter.ty().into())
                    .collect();
                if decl.opt_this_liason.is_some() {
                    spatial_arguments
                        .insert(0, SpatialArgument::EntityRoute(entity_route.parent()));
                }
                spatial_arguments.push(SpatialArgument::EntityRoute(decl.output.ty()));
                self.db.route_call(base_route, spatial_arguments)
            }
            EntityKind::Feature => self.db.feature_decl(entity_route)?.ty,
            EntityKind::Main => panic!(),
        })
    }

    fn infer_primitive_literal(
        &mut self,
        expectation: Option<EntityRoutePtr>,
        value: RawLiteralData,
    ) -> EntityRoutePtr {
        match value {
            RawLiteralData::Void => todo!(),
            RawLiteralData::Integer(_) => {
                if let Some(expectation) = expectation {
                    let intrinsic = expectation.intrinsic();
                    match intrinsic {
                        EntityRoutePtr::Root(RootBuiltinIdentifier::I64) => intrinsic,
                        _ => RootBuiltinIdentifier::I32.into(),
                    }
                } else {
                    // the default integer type is i32
                    RootBuiltinIdentifier::I32.into()
                }
            }
            RawLiteralData::I32(_) => todo!(),
            RawLiteralData::I64(_) => todo!(),
            RawLiteralData::Float(_) => {
                if let Some(expectation) = expectation {
                    let intrinsic = expectation.intrinsic();
                    match intrinsic {
                        EntityRoutePtr::Root(RootBuiltinIdentifier::F64) => intrinsic,
                        _ => RootBuiltinIdentifier::F32.into(),
                    }
                } else {
                    // the default float type is f32
                    RootBuiltinIdentifier::F32.into()
                }
            }
            RawLiteralData::F32(_) => todo!(),
            RawLiteralData::F64(_) => todo!(),
            RawLiteralData::Bits(_) => todo!(),
            RawLiteralData::B32(_) => {
                if let Some(expectation) = expectation {
                    match expectation.intrinsic() {
                        EntityRoutePtr::Root(RootBuiltinIdentifier::B32) => (),
                        _ => todo!(),
                    }
                }
                // the default float type is f32
                RootBuiltinIdentifier::B32.into()
            }
            RawLiteralData::B64(_) => todo!(),
            RawLiteralData::Bool(_) => todo!(),
        }
    }

    fn infer_opn(
        &mut self,
        idx: RawExprIdx,
        expectation: Option<EntityRoutePtr>,
        opr: &RawOpnVariant,
        opds: &RawExprRange,
    ) -> InferResult<EntityRoutePtr> {
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
    ) -> InferResult<EntityRoutePtr> {
        let lopd_ty = derived_not_none!(self.infer_expr(lopd, None))?;
        let ropd_ty = derived_not_none!(self.infer_expr(ropd, None))?;
        match opr {
            BinaryOpr::Pure(pure_binary_opr) => match lopd_ty {
                EntityRoutePtr::Root(lopd_root_ty) => match ropd_ty {
                    EntityRoutePtr::Root(ropd_root_ty) => self.root_pure_binary_opn(
                        pure_binary_opr,
                        lopd_root_ty,
                        ropd_root_ty,
                        range,
                    ),
                    EntityRoutePtr::Custom(_) => {
                        throw!(
                            format!("expect ropd to be of root type, but got `{}`", ropd_ty),
                            range
                        )
                    }
                },
                EntityRoutePtr::Custom(_) => match pure_binary_opr {
                    PureBinaryOpr::Eq | PureBinaryOpr::Neq => {
                        if lopd_ty.intrinsic() == ropd_ty.intrinsic() {
                            Ok(EntityRoutePtr::Root(RootBuiltinIdentifier::Bool))
                        } else {
                            todo!()
                        }
                    }
                    _ => {
                        p!(pure_binary_opr);
                        todo!()
                    }
                },
            },
            BinaryOpr::Assign(_) => {
                if lopd_ty != ropd_ty {
                    throw!(format!("expect same type for assignment"), range)
                }
                Ok(RootBuiltinIdentifier::Void.into())
            }
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::Curry => todo!(),
            BinaryOpr::As => todo!(),
        }
    }

    fn root_pure_binary_opn(
        &self,
        pure_binary_opr: PureBinaryOpr,
        lopd_builtin_ty: RootBuiltinIdentifier,
        ropd_builtin_ty: RootBuiltinIdentifier,
        range: TextRange,
    ) -> InferResult<EntityRoutePtr> {
        Ok(match pure_binary_opr {
            PureBinaryOpr::Less
            | PureBinaryOpr::Leq
            | PureBinaryOpr::Greater
            | PureBinaryOpr::Geq => {
                if lopd_builtin_ty != ropd_builtin_ty {
                    throw!("expect use of \"<, <=, >, >=\" on same types", range)
                }
                match lopd_builtin_ty {
                    RootBuiltinIdentifier::I32 | RootBuiltinIdentifier::F32 => (),
                    _ => throw!("expect use of \"<, <=, >, >=\" on i32 or f32", range),
                }
                RootBuiltinIdentifier::Bool
            }
            PureBinaryOpr::Eq | PureBinaryOpr::Neq => {
                if lopd_builtin_ty != ropd_builtin_ty {
                    throw!("expect use of \"!=\" on same types", range)
                }
                RootBuiltinIdentifier::Bool
            }
            PureBinaryOpr::Shl => {
                match lopd_builtin_ty {
                    RootBuiltinIdentifier::B32 | RootBuiltinIdentifier::B64 => (),
                    _ => throw!("expect b32 or b64 for lopd of shift left `<<`", range),
                }
                match ropd_builtin_ty {
                    RootBuiltinIdentifier::I32 => (),
                    _ => throw!("expect i32 for ropd of shift left `>>`", range),
                }
                lopd_builtin_ty
            }
            PureBinaryOpr::Shr => {
                match lopd_builtin_ty {
                    RootBuiltinIdentifier::B32 | RootBuiltinIdentifier::B64 => (),
                    _ => throw!("expect b32 or b64 for lopd of shift right `>>`", range),
                }
                match ropd_builtin_ty {
                    RootBuiltinIdentifier::I32 => (),
                    _ => throw!("expect i32 for ropd of shift right `>>`", range),
                }
                lopd_builtin_ty
            }
            PureBinaryOpr::Add
            | PureBinaryOpr::Sub
            | PureBinaryOpr::Mul
            | PureBinaryOpr::Div
            | PureBinaryOpr::Power => {
                if lopd_builtin_ty != ropd_builtin_ty {
                    throw!("expect use of \"+, -, *, /, **\" on same types", range)
                }
                match lopd_builtin_ty {
                    RootBuiltinIdentifier::I32 | RootBuiltinIdentifier::F32 => (),
                    _ => throw!("expect use of \"+, -, *, /, **\" on i32 or f32", range),
                }
                lopd_builtin_ty
            }
            PureBinaryOpr::And | PureBinaryOpr::Or => {
                if lopd_builtin_ty != RootBuiltinIdentifier::Bool {
                    throw!("expect lopd to be of type `bool`", range)
                }
                if ropd_builtin_ty != RootBuiltinIdentifier::Bool {
                    throw!("expect ropd to be of type `bool`", range)
                }
                RootBuiltinIdentifier::Bool
            }
            PureBinaryOpr::BitXor | PureBinaryOpr::BitAnd | PureBinaryOpr::BitOr => {
                if lopd_builtin_ty != ropd_builtin_ty {
                    throw!(
                        format!(
                            "expect use of bitor `|` on same types, but got `{}` and `{}`",
                            lopd_builtin_ty.as_str(),
                            ropd_builtin_ty.as_str()
                        ),
                        range
                    )
                }
                if lopd_builtin_ty != RootBuiltinIdentifier::B32
                    && lopd_builtin_ty != RootBuiltinIdentifier::B64
                {
                    throw!("expect use of \"|\" on b32 or b64", range)
                }
                lopd_builtin_ty
            }
            PureBinaryOpr::RemEuclid => match (lopd_builtin_ty, ropd_builtin_ty) {
                (RootBuiltinIdentifier::I32, RootBuiltinIdentifier::I32) => {
                    RootBuiltinIdentifier::I32
                }
                (RootBuiltinIdentifier::I64, RootBuiltinIdentifier::I64) => {
                    RootBuiltinIdentifier::I64
                }
                (RootBuiltinIdentifier::F32, RootBuiltinIdentifier::F32) => {
                    RootBuiltinIdentifier::F32
                }
                _ => {
                    throw!("expect use of rem euclid \"%\" on i32 or f32", range)
                }
            },
        }
        .into())
    }

    fn infer_prefix(&mut self, opr: PrefixOpr, opd: RawExprIdx) -> InferResult<EntityRoutePtr> {
        let opd_ty = derived_not_none!(self.infer_expr(opd, None,))?;
        match opr {
            PrefixOpr::Minus => match opd_ty {
                EntityRoutePtr::Root(root_ident) => match root_ident {
                    RootBuiltinIdentifier::I32
                    | RootBuiltinIdentifier::I64
                    | RootBuiltinIdentifier::F32
                    | RootBuiltinIdentifier::F64 => Ok(opd_ty),
                    _ => Err(error!(
                        "minus can only be applied for integer or float",
                        self.arena[opd].range
                    )),
                },
                EntityRoutePtr::Custom(_) => todo!(),
            },
            PrefixOpr::Not => {
                if self.db.is_implicitly_castable(
                    opd_ty,
                    EntityRoutePtr::Root(RootBuiltinIdentifier::Bool),
                ) {
                    Ok(EntityRoutePtr::Root(RootBuiltinIdentifier::Bool))
                } else {
                    p!(opd_ty);
                    todo!()
                }
            }
            PrefixOpr::BitNot => match opd_ty {
                EntityRoutePtr::Root(root_ident) => match root_ident {
                    RootBuiltinIdentifier::B32 | RootBuiltinIdentifier::B64 => Ok(opd_ty),
                    _ => todo!(),
                },
                EntityRoutePtr::Custom(_) => todo!(),
            },
            PrefixOpr::Shared => todo!(),
            PrefixOpr::Move => todo!(),
        }
    }

    fn infer_suffix(&mut self, opr: &RawSuffixOpr, opd: RawExprIdx) -> InferResult<EntityRoutePtr> {
        let opd_ty = derived_not_none!(self.infer_expr(opd, None,))?;
        match opr {
            RawSuffixOpr::Incr | RawSuffixOpr::Decr => {
                match opd_ty {
                    EntityRoutePtr::Root(opd_ty_ident) => match opd_ty_ident {
                        RootBuiltinIdentifier::I32 => (),
                        _ => todo!(),
                    },
                    EntityRoutePtr::Custom(_) => todo!(),
                }
                Ok(EntityRoutePtr::Root(RootBuiltinIdentifier::Void))
            }
            RawSuffixOpr::AsTy(ranged_ty) => Ok(ranged_ty.route),
            RawSuffixOpr::BePattern(_) => {
                msg_once!("check be pattern ty");
                Ok(RootBuiltinIdentifier::Bool.into())
            }
            RawSuffixOpr::Unveil => {
                if !opd_ty.is_option() {
                    todo!()
                }
                Ok(opd_ty.entity_route_argument(0))
            }
        }
    }

    fn infer_field_access(
        &mut self,
        opt_field_ident: Option<RangedCustomIdentifier>,
        opd: RawExprIdx,
    ) -> InferResult<EntityRoutePtr> {
        let opd_ty = derived_not_none!(self.infer_expr(opd, None))?;
        let field_ident = opt_field_ident.ok_or({
            let start = self.arena[opd].range.text_end();
            InferError {
                variant: InferErrorVariant::Original {
                    message: format!("expect field ident"),
                    range: (start..(start.to_right(1))).into(),
                },
                dev_src: dev_src!(),
            }
        })?;
        derived_unwrap!(self.db.ty_decl(opd_ty.intrinsic())).field_ty_result(field_ident)
    }

    fn list_opn_ty_result(
        &mut self,
        idx: RawExprIdx,
        expectation: Option<EntityRoutePtr>,
        opr: &ListOpr,
        opds: &RawExprRange,
        range: TextRange,
    ) -> InferResult<EntityRoutePtr> {
        msg_once!("expectation");
        match opr {
            ListOpr::NewTuple => self.infer_new_tuple(idx, expectation, opds),
            ListOpr::NewVec => self.infer_new_vec(idx, expectation, opds),
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

    fn infer_new_tuple(
        &mut self,
        idx: RawExprIdx,
        expectation: Option<EntityRoutePtr>,
        opds: &RawExprRange,
    ) -> InferResult<EntityRoutePtr> {
        msg_once!("expectation");
        if opds.start == opds.end {
            return Ok(RootBuiltinIdentifier::Void.into());
        }
        todo!()
    }

    fn infer_new_vec(
        &mut self,
        idx: RawExprIdx,
        expectation: Option<EntityRoutePtr>,
        opds: &RawExprRange,
    ) -> InferResult<EntityRoutePtr> {
        msg_once!("expectation");
        if opds.start == opds.end {
            // empty vec
            if let Some(expectation) = expectation {
                if expectation.variant
                    != (EntityRouteVariant::Root {
                        ident: RootBuiltinIdentifier::Vec,
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
    ) -> InferResult<EntityRoutePtr> {
        let caller = &self.arena[opds.start];
        // ad hoc
        let caller_ty_result = self.infer_expr(opds.start, None);
        let call_decl = match caller.variant {
            RawExprVariant::Entity { route, .. } => {
                self.entity_route_sheet
                    .function_call_routes
                    .insert_new(opds.start, Ok(route));
                self.db.entity_call_form_decl(route)
            }
            _ => self
                .db
                .value_call_form_decl(derived_not_none!(caller_ty_result)?),
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
    ) -> InferResult<EntityRoutePtr> {
        let intrinsic_ty = derived_not_none!(self.infer_expr(this, None))?.intrinsic();
        let this_intrinsic_ty_decl = derived_unwrap!(self.db.ty_decl(intrinsic_ty));
        let call_form_decl = this_intrinsic_ty_decl.method(method_ident, &self.trait_uses)?;
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
    ) -> InferResult<EntityRoutePtr> {
        if total_opds.end - total_opds.start < 2 {
            panic!()
            // throw!(format!("expect indices inside `[]`"), total_range);
        }
        if total_opds.end - total_opds.start > 2 {}
        let this_ty = derived_not_none!(self.infer_expr(total_opds.start, None))?;
        let index_ty = derived_not_none!(self.infer_expr(total_opds.start + 1, None))?;
        let this_ty_decl = derived_unwrap!(self.db.ty_decl(this_ty));
        let index_trai = self.db.intern_entity_route(EntityRoute {
            variant: self
                .db
                .entity_route_menu()
                .std_ops_index_trai
                .variant
                .clone(),
            temporal_arguments: thin_vec![],
            spatial_arguments: thin_vec![SpatialArgument::EntityRoute(index_ty)],
        });
        let trai_impl = ok_or!(
            this_ty_decl.trait_impl(index_trai),
            format!(
                "can't index by `{:?}` into type `{:?}`,\nit doesn't satisfy trait `{:?}`",
                index_ty, this_ty, index_trai
            ),
            total_range
        )?;
        Ok(match trai_impl.member_impls()[0] {
            TraitMemberImplDecl::AssociatedType { ty, .. } => ty,
            _ => panic!(),
        })
    }
}
