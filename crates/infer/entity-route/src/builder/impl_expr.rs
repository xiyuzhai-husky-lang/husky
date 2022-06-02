use std::iter::zip;

use ast::RawExprRange;
use dev_utils::dev_src;
use infer_decl::{MethodKind, TraitMemberImplDecl};
use text::*;
use vm::*;
use word::CustomIdentifier;

use super::*;

impl<'a> EntityRouteSheetBuilder<'a> {
    pub(super) fn infer_expr(
        &mut self,
        raw_expr_idx: RawExprIdx,
        expectation: Option<EntityRoutePtr>,
        arena: &RawExprArena,
    ) -> Option<EntityRoutePtr> {
        let ty_result: InferResult<EntityRoutePtr> =
            self.expr_ty_result(raw_expr_idx, expectation, arena);
        let opt_ty = match ty_result {
            Ok(opd_ty) => Some(opd_ty),
            Err(_) => None,
        };
        self.entity_route_sheet
            .expr_tys
            .insert_new(raw_expr_idx, ty_result);
        opt_ty
    }

    pub(super) fn expr_ty_result(
        &mut self,
        raw_expr_idx: RawExprIdx,
        expectation: Option<EntityRoutePtr>,
        arena: &RawExprArena,
    ) -> InferResult<EntityRoutePtr> {
        let expr = &arena[raw_expr_idx];
        let ty = match expr.variant {
            RawExprVariant::Variable {
                varname,
                init_range,
            } => {
                derived_not_none!(self
                    .entity_route_sheet
                    .variable_tys
                    .get(&(varname, init_range))
                    .map(|route| *route))
            }
            RawExprVariant::Unrecognized(ident) => Err(InferError {
                variant: InferErrorVariant::Original {
                    message: format!("Unrecognized identifier `{}`", &ident),
                    range: arena[raw_expr_idx].range,
                },
                dev_src: dev_src!(),
            }),
            RawExprVariant::Entity { route, kind } => self.infer_entity(route, kind),
            RawExprVariant::CopyableLiteral(value) => Ok(value.ty().into()),
            RawExprVariant::Bracketed(expr) => {
                derived_not_none!(self.infer_expr(expr, expectation, arena))
            }
            RawExprVariant::Opn {
                opn_variant: ref opr,
                ref opds,
            } => self.infer_opn(opr, opds, raw_expr_idx, arena),
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::ThisValue {
                opt_this_ty: opt_ty,
                ..
            } => derived_not_none!(opt_ty),
            RawExprVariant::FrameVariable { .. } => Ok(self.db.entity_route_menu().i32_ty),
            RawExprVariant::ThisField { opt_field_ty, .. } => {
                Ok(derived_not_none!(opt_field_ty)?.route)
            }
        }?
        .deref_route();
        if let Some(expected_ty) = expectation {
            if !self.db.is_implicitly_castable(ty, expected_ty) {
                throw!(
                    format!("expect {:?} but get {:?} instead", expected_ty, ty),
                    arena[raw_expr_idx].range
                )
            }
        }
        Ok(ty)
    }

    fn infer_entity(
        &mut self,
        scope: EntityRoutePtr,
        entity_kind: EntityKind,
    ) -> InferResult<EntityRoutePtr> {
        Ok(match entity_kind {
            EntityKind::Module => EntityRoutePtr::Root(RootIdentifier::ModuleType),
            EntityKind::EnumLiteral => match scope {
                EntityRoutePtr::Root(RootIdentifier::True)
                | EntityRoutePtr::Root(RootIdentifier::False) => RootIdentifier::Bool.into(),
                EntityRoutePtr::Custom(scope) => match scope.kind {
                    EntityRouteKind::Root { ident } => todo!(),
                    EntityRouteKind::Package { main, ident } => todo!(),
                    EntityRouteKind::Child { parent, ident } => parent,
                    EntityRouteKind::Input { main } => todo!(),
                    EntityRouteKind::Generic { ident, .. } => todo!(),
                    EntityRouteKind::ThisType => todo!(),
                    EntityRouteKind::TypeAsTraitMember {
                        ty: parent,
                        trai,
                        ident,
                    } => todo!(),
                },
                _ => todo!(),
            },
            EntityKind::Type(_) => RootIdentifier::TypeType.into(),
            EntityKind::Trait => todo!(),
            EntityKind::Function { is_lazy } => {
                if is_lazy {
                    todo!()
                } else {
                    emsg_once!("todo: generics in fp");
                    RootIdentifier::Fp.into()
                }
            }
            EntityKind::Feature => self.db.feature_decl(scope)?.ty,
            EntityKind::Member(_) => todo!(),
            EntityKind::Main => panic!(),
        })
    }

    fn infer_opn(
        &mut self,
        opr: &RawOpnVariant,
        opds: &RawExprRange,
        raw_expr_idx: RawExprIdx,
        arena: &RawExprArena,
    ) -> InferResult<EntityRoutePtr> {
        let range = arena[raw_expr_idx].range;
        match opr {
            RawOpnVariant::Binary(opr) => {
                self.binary_opn(*opr, opds.start, opds.start + 1, arena, range)
            }
            RawOpnVariant::Prefix(opr) => self.infer_prefix(*opr, opds.start, arena),
            RawOpnVariant::Suffix(opr) => self.infer_suffix(*opr, opds.start, arena, range),
            RawOpnVariant::List(opr) => {
                self.list_opn_ty_result(opr, opds, arena, range, raw_expr_idx)
            }
            RawOpnVariant::FieldAccess(field_ident) => {
                self.infer_field_access(*field_ident, opds.start, arena, range)
            }
        }
    }

    fn binary_opn(
        &mut self,
        opr: BinaryOpr,
        lopd: RawExprIdx,
        ropd: RawExprIdx,
        arena: &RawExprArena,
        range: TextRange,
    ) -> InferResult<EntityRoutePtr> {
        let lopd_ty = derived_not_none!(self.infer_expr(lopd, None, arena))?;
        let ropd_ty = derived_not_none!(self.infer_expr(ropd, None, arena))?;
        match opr {
            BinaryOpr::Pure(pure_binary_opr) => match lopd_ty {
                EntityRoutePtr::Root(lopd_builtin_ty) => match ropd_ty {
                    EntityRoutePtr::Root(ropd_builtin_ty) => self.builtin_pure_binary_opn(
                        pure_binary_opr,
                        lopd_builtin_ty,
                        ropd_builtin_ty,
                        range,
                    ),
                    EntityRoutePtr::Custom(_) => {
                        throw!(
                            format!("expect ropd to be of root type, but got `{}`", ropd_ty),
                            range
                        )
                    }
                    EntityRoutePtr::ThisType => todo!(),
                },
                EntityRoutePtr::Custom(lopd_custom_ty) => match pure_binary_opr {
                    PureBinaryOpr::Eq | PureBinaryOpr::Neq => {
                        if lopd_ty == ropd_ty {
                            Ok(EntityRoutePtr::Root(RootIdentifier::Bool))
                        } else {
                            todo!()
                        }
                    }
                    PureBinaryOpr::Add => todo!(),
                    PureBinaryOpr::And => todo!(),
                    PureBinaryOpr::BitAnd => todo!(),
                    PureBinaryOpr::BitOr => todo!(),
                    PureBinaryOpr::BitXor => todo!(),
                    PureBinaryOpr::Div => todo!(),
                    PureBinaryOpr::Geq => todo!(),
                    PureBinaryOpr::Greater => todo!(),
                    PureBinaryOpr::Leq => todo!(),
                    PureBinaryOpr::Less => todo!(),
                    PureBinaryOpr::Mul => todo!(),
                    PureBinaryOpr::RemEuclid => todo!(),
                    PureBinaryOpr::Or => todo!(),
                    PureBinaryOpr::Power => todo!(),
                    PureBinaryOpr::Shl => todo!(),
                    PureBinaryOpr::Shr => todo!(),
                    PureBinaryOpr::Sub => todo!(),
                },
                EntityRoutePtr::ThisType => todo!(),
            },
            BinaryOpr::Assign(_) => {
                if lopd_ty != ropd_ty {
                    throw!(format!("expect same type for assignment"), range)
                }
                Ok(RootIdentifier::Void.into())
            }
        }
    }

    fn builtin_pure_binary_opn(
        &self,
        pure_binary_opr: PureBinaryOpr,
        lopd_builtin_ty: RootIdentifier,
        ropd_builtin_ty: RootIdentifier,
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
                    RootIdentifier::I32 | RootIdentifier::F32 => (),
                    _ => throw!("expect use of \"<, <=, >, >=\" on i32 or f32", range),
                }
                RootIdentifier::Bool
            }
            PureBinaryOpr::Eq | PureBinaryOpr::Neq => {
                if lopd_builtin_ty != ropd_builtin_ty {
                    throw!("expect use of \"!=\" on same types", range)
                }
                RootIdentifier::Bool
            }
            PureBinaryOpr::Shl => {
                match lopd_builtin_ty {
                    RootIdentifier::B32 | RootIdentifier::B64 => (),
                    _ => throw!("expect b32 or b64 for lopd of shift left `<<`", range),
                }
                match ropd_builtin_ty {
                    RootIdentifier::I32 => (),
                    _ => throw!("expect i32 for ropd of shift left `>>`", range),
                }
                lopd_builtin_ty
            }
            PureBinaryOpr::Shr => {
                match lopd_builtin_ty {
                    RootIdentifier::B32 | RootIdentifier::B64 => (),
                    _ => throw!("expect b32 or b64 for lopd of shift right `>>`", range),
                }
                match ropd_builtin_ty {
                    RootIdentifier::I32 => (),
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
                    RootIdentifier::I32 | RootIdentifier::F32 => (),
                    _ => throw!("expect use of \"+, -, *, /, **\" on i32 or f32", range),
                }
                lopd_builtin_ty
            }
            PureBinaryOpr::And | PureBinaryOpr::Or => {
                if lopd_builtin_ty != RootIdentifier::Bool {
                    throw!("expect lopd to be of type `bool`", range)
                }
                if ropd_builtin_ty != RootIdentifier::Bool {
                    throw!("expect ropd to be of type `bool`", range)
                }
                RootIdentifier::Bool
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
                if lopd_builtin_ty != RootIdentifier::B32 && lopd_builtin_ty != RootIdentifier::B64
                {
                    throw!("expect use of \"|\" on b32 or b64", range)
                }
                lopd_builtin_ty
            }
            PureBinaryOpr::RemEuclid => match (lopd_builtin_ty, ropd_builtin_ty) {
                (RootIdentifier::I32, RootIdentifier::I32) => RootIdentifier::I32,
                _ => {
                    p!(lopd_builtin_ty, ropd_builtin_ty);
                    todo!()
                }
            },
        }
        .into())
    }

    fn infer_prefix(
        &mut self,
        opr: PrefixOpr,
        opd: RawExprIdx,
        arena: &RawExprArena,
    ) -> InferResult<EntityRoutePtr> {
        let opd_ty = derived_not_none!(self.infer_expr(opd, None, arena))?;
        match opr {
            PrefixOpr::Minus => match opd_ty {
                EntityRoutePtr::Root(root_ident) => match root_ident {
                    RootIdentifier::Void => todo!(),
                    RootIdentifier::I32 => Ok(EntityRoutePtr::Root(RootIdentifier::I32)),
                    RootIdentifier::F32 => Ok(EntityRoutePtr::Root(RootIdentifier::F32)),
                    RootIdentifier::B32 => todo!(),
                    RootIdentifier::B64 => todo!(),
                    RootIdentifier::Bool => todo!(),
                    RootIdentifier::True => todo!(),
                    RootIdentifier::False => todo!(),
                    RootIdentifier::Vec => todo!(),
                    RootIdentifier::Tuple => todo!(),
                    RootIdentifier::Debug => todo!(),
                    RootIdentifier::Std => todo!(),
                    RootIdentifier::Core => todo!(),
                    RootIdentifier::Fp => todo!(),
                    RootIdentifier::Fn => todo!(),
                    RootIdentifier::FnMut => todo!(),
                    RootIdentifier::FnOnce => todo!(),
                    RootIdentifier::Array => todo!(),
                    RootIdentifier::Datasets => todo!(),
                    RootIdentifier::DatasetType => todo!(),
                    RootIdentifier::TypeType => todo!(),
                    RootIdentifier::ModuleType => todo!(),
                    RootIdentifier::CloneTrait => todo!(),
                    RootIdentifier::CopyTrait => todo!(),
                    RootIdentifier::PartialEqTrait => todo!(),
                    RootIdentifier::EqTrait => todo!(),
                    RootIdentifier::Ref => todo!(),
                },
                EntityRoutePtr::Custom(_) => todo!(),
                EntityRoutePtr::ThisType => todo!(),
            },
            PrefixOpr::Not => {
                if self
                    .db
                    .is_implicitly_castable(opd_ty, EntityRoutePtr::Root(RootIdentifier::Bool))
                {
                    Ok(EntityRoutePtr::Root(RootIdentifier::Bool))
                } else {
                    p!(opd_ty);
                    todo!()
                }
            }
            PrefixOpr::BitNot => match opd_ty {
                EntityRoutePtr::Root(root_ident) => match root_ident {
                    RootIdentifier::B32 | RootIdentifier::B64 => Ok(opd_ty),
                    _ => todo!(),
                },
                EntityRoutePtr::Custom(_) => todo!(),
                EntityRoutePtr::ThisType => todo!(),
            },
            PrefixOpr::Shared => todo!(),
            PrefixOpr::Move => todo!(),
        }
    }

    fn infer_suffix(
        &mut self,
        opr: SuffixOpr,
        opd: RawExprIdx,
        arena: &RawExprArena,
        range: TextRange,
    ) -> InferResult<EntityRoutePtr> {
        let opd_ty = derived_not_none!(self.infer_expr(opd, None, arena))?;
        match opr {
            SuffixOpr::Incr | SuffixOpr::Decr => {
                match opd_ty {
                    EntityRoutePtr::Root(opd_ty_ident) => match opd_ty_ident {
                        RootIdentifier::I32 => (),
                        _ => todo!(),
                    },
                    EntityRoutePtr::Custom(_) => todo!(),
                    EntityRoutePtr::ThisType => todo!(),
                }
                Ok(EntityRoutePtr::Root(RootIdentifier::Void))
            }
            SuffixOpr::WithTy(_) => todo!(),
            SuffixOpr::AsTy(ranged_ty) => Ok(ranged_ty.route),
        }
    }

    fn infer_field_access(
        &mut self,
        field_ident: RangedCustomIdentifier,
        opd: RawExprIdx,
        arena: &RawExprArena,
        range: TextRange,
    ) -> InferResult<EntityRoutePtr> {
        let opd_ty = derived_not_none!(self.infer_expr(opd, None, arena))?;
        derived_unwrap!(self.db.ty_decl(opd_ty)).field_ty_result(field_ident)
    }

    fn list_opn_ty_result(
        &mut self,
        opr: &ListOpr,
        opds: &RawExprRange,
        arena: &RawExprArena,
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EntityRoutePtr> {
        match opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.infer_call(opds, arena, range, raw_expr_idx),
            ListOpr::Index => self.infer_index(arena, opds, range),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
            ListOpr::MethodCall { ranged_ident, .. } => self.infer_method_call(
                opds.start,
                *ranged_ident,
                (opds.start + 1)..opds.end,
                arena,
                raw_expr_idx,
            ),
        }
    }

    fn infer_call(
        &mut self,
        total_opds: &RawExprRange,
        arena: &RawExprArena,
        range: TextRange,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EntityRoutePtr> {
        let caller = &arena[total_opds.start];
        match caller.variant {
            RawExprVariant::Entity { route, kind, .. } => {
                let call_decl_result: InferResult<_> = self.db.call_decl(route).bind_into(caller);
                let call_decl = call_decl_result?;
                if call_decl.primary_parameters.len() != total_opds.end - total_opds.start - 1 {
                    self.entity_route_sheet.extra_errors.push(InferError {
                        variant: InferErrorVariant::Original {
                            message: format!(
                                "expect {} arguments, but get {} arguments",
                                call_decl.primary_parameters.len(),
                                total_opds.end - total_opds.start - 1
                            ),
                            range,
                        },
                        dev_src: dev_src!(),
                    })
                }
                for (argument, parameter) in zip(
                    ((total_opds.start + 1)..total_opds.end).into_iter(),
                    call_decl.primary_parameters.iter(),
                ) {
                    self.infer_expr(argument, Some(parameter.ty), arena);
                }
                Ok(call_decl.output.ty)
            }
            RawExprVariant::Unrecognized(_) => {
                throw!("unrecognized caller", caller.range)
            }
            RawExprVariant::CopyableLiteral(_) | RawExprVariant::FrameVariable { .. } => {
                throw!("a primitive literal can't be a caller", caller.range)
            }
            RawExprVariant::Bracketed(_)
            | RawExprVariant::Opn { .. }
            | RawExprVariant::ThisValue { .. }
            | RawExprVariant::Variable { .. } => todo!(),
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::ThisField { .. } => todo!(),
        }
    }

    fn infer_method_call(
        &mut self,
        this: RawExprIdx,
        method_ident: RangedCustomIdentifier,
        inputs: RawExprRange,
        arena: &RawExprArena,
        raw_expr_idx: RawExprIdx,
    ) -> InferResult<EntityRoutePtr> {
        let this_ty = derived_not_none!(self.infer_expr(this, None, arena))?;
        let this_ty_decl = derived_unwrap!(self.db.ty_decl(this_ty));
        let method_decl = this_ty_decl.method(method_ident, &self.trait_uses)?;
        for (argument, parameter) in zip(inputs.into_iter(), method_decl.parameters.iter()) {
            self.infer_expr(argument, Some(parameter.ty), arena);
        }
        let generic_arguments = if method_decl.generic_parameters.len() > 0 {
            todo!()
        } else {
            vec![]
        };
        self.entity_route_sheet.call_routes.insert_new(
            raw_expr_idx,
            Ok(self.db.intern_entity_route(EntityRoute {
                kind: match method_decl.kind {
                    MethodKind::Type => EntityRouteKind::Child {
                        parent: this_ty,
                        ident: method_decl.ident,
                    },
                    MethodKind::Trait { trai } => EntityRouteKind::TypeAsTraitMember {
                        ty: this_ty,
                        ident: method_decl.ident,
                        trai,
                    },
                },
                temporal_arguments: vec![],
                spatial_arguments: generic_arguments,
            })),
        );
        Ok(method_decl.output.ty)
    }

    fn infer_index(
        &mut self,
        arena: &RawExprArena,
        total_opds: &RawExprRange,
        total_range: TextRange,
    ) -> InferResult<EntityRoutePtr> {
        if total_opds.end - total_opds.start < 2 {
            panic!()
            // throw!(format!("expect indices inside `[]`"), total_range);
        }
        if total_opds.end - total_opds.start > 2 {}
        let this_ty = derived_not_none!(self.infer_expr(total_opds.start, None, arena))?;
        let index_ty = derived_not_none!(self.infer_expr(total_opds.start + 1, None, arena))?;
        let this_ty_decl = derived_unwrap!(self.db.ty_decl(this_ty));
        let index_trai = self.db.intern_entity_route(EntityRoute {
            kind: self.db.entity_route_menu().std_ops_index_trai.kind,
            temporal_arguments: vec![],
            spatial_arguments: vec![SpatialArgument::EntityRoute(index_ty)],
        });
        let trai_impl = ok_or!(
            this_ty_decl.trait_impl(index_trai),
            format!(
                "can't index by `{:?}` into type `{:?}`,\nit doesn't satisfy trait `{:?}`",
                index_ty, this_ty, index_trai
            ),
            total_range
        )?;
        Ok(match trai_impl.member_impls[0] {
            TraitMemberImplDecl::Method(_) => todo!(),
            TraitMemberImplDecl::AssociatedType { ty, .. } => ty,
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
        })
    }
}
