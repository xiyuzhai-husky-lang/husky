use std::sync::Arc;

use crate::*;
use ast::{RawExprArena, RawExprIdx, RawExprRange, RawExprVariant};
use entity_kind::TyKind;
use entity_route::{EntityKind, EntityRoutePtr, RangedEntityRoute};
use file::FilePtr;
use infer_contract::InferContract;
use infer_entity_route::InferEntityRoute;
use vm::*;
use word::{CustomIdentifier, RootIdentifier};

use super::*;
use semantics_error::*;

pub trait LazyExprParser<'a>: InferEntityRoute + InferContract {
    fn arena(&self) -> &'a RawExprArena;
    fn vartype(&self, varname: CustomIdentifier) -> EntityRoutePtr;
    fn file(&self) -> FilePtr;
    fn db(&self) -> &dyn InferQueryGroup;

    fn parse_lazy_expr(&mut self, raw_expr_idx: RawExprIdx) -> SemanticResult<Arc<LazyExpr>> {
        let raw_expr = &self.arena()[raw_expr_idx];
        let kind: LazyExprKind = match raw_expr.variant {
            RawExprVariant::Variable { varname, .. } => LazyExprKind::Variable(varname),
            RawExprVariant::Unrecognized(ident) => {
                err!(format!(
                    "unrecognized identifier {} at {:?}",
                    ident,
                    raw_expr.range()
                ))
            }
            RawExprVariant::Entity {
                route: entity_route,
                kind,
                ..
            } => match kind {
                EntityKind::Module => todo!(),
                EntityKind::Literal => match entity_route {
                    EntityRoutePtr::Root(RootIdentifier::True) => {
                        LazyExprKind::PrimitiveLiteral(PrimitiveValue::Bool(true))
                    }
                    EntityRoutePtr::Root(RootIdentifier::False) => {
                        LazyExprKind::PrimitiveLiteral(PrimitiveValue::Bool(false))
                    }
                    EntityRoutePtr::Custom(scope_ref) => LazyExprKind::EnumLiteral { entity_route },
                    _ => todo!(),
                },
                EntityKind::Type(_) => todo!(),
                EntityKind::Trait => todo!(),
                EntityKind::Routine => todo!(),
                EntityKind::Feature => LazyExprKind::EntityFeature {
                    route: entity_route,
                },
                EntityKind::Pattern => todo!(),
                EntityKind::Member(_) => todo!(),
            },
            RawExprVariant::PrimitiveLiteral(value) => LazyExprKind::PrimitiveLiteral(value),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn { opr, ref opds } => self.parse_opn(opr, opds, raw_expr_idx)?,
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::This { .. } => LazyExprKind::This,
            RawExprVariant::FrameVariable { varname, init_row } => todo!(),
        };
        Ok(Arc::new(LazyExpr {
            range: raw_expr.range().clone(),
            ty: self.raw_expr_ty(raw_expr_idx)?,
            kind,
            file: self.file(),
            instruction_id: Default::default(),
            contract: self.lazy_expr_contract(raw_expr_idx)?,
        }))
    }

    fn parse_opn(
        &mut self,
        opr: Opr,
        opds: &RawExprRange,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<LazyExprKind> {
        match opr {
            Opr::Binary(opr) => self.parse_binary_opr(opr, opds),
            Opr::Prefix(_) => todo!(),
            Opr::Suffix(opr) => self.parse_suffix_opr(opr, opds),
            Opr::List(opr) => match opr {
                ListOpr::TupleInit => todo!(),
                ListOpr::NewVec => todo!(),
                ListOpr::NewDict => todo!(),
                ListOpr::Call => self.parse_call(opds, raw_expr_idx),
                ListOpr::Index => self.parse_element_access(opds.clone()),
                ListOpr::ModuloIndex => todo!(),
                ListOpr::StructInit => todo!(),
            },
        }
    }

    fn parse_binary_opr(
        &mut self,
        opr: BinaryOpr,
        raw_opds: &RawExprRange,
    ) -> SemanticResult<LazyExprKind> {
        // let raw_opds = &self.arena()[raw_opds];
        let lopd = self.parse_lazy_expr(raw_opds.start)?;
        let ropd = self.parse_lazy_expr(raw_opds.start + 1)?;
        let output_type = match opr {
            BinaryOpr::Pure(pure_binary_opr) => {
                self.infer_pure_binary_opr_type(pure_binary_opr, lopd.ty, ropd.ty)?
            }
            BinaryOpr::Assign(opt_binary) => {
                if let Some(pure_binary_opr) = opt_binary {
                    if lopd.ty
                        != self.infer_pure_binary_opr_type(pure_binary_opr, lopd.ty, ropd.ty)?
                    {
                        todo!()
                    }
                }
                RootIdentifier::Void.into()
            }
        };
        let opr = match opr {
            BinaryOpr::Pure(opr) => opr,
            BinaryOpr::Assign(_) => todo!(),
        };
        Ok(LazyExprKind::Opn {
            opn_kind: LazyOpnKind::Binary { opr, this: lopd.ty },
            opds: vec![lopd, ropd],
        })
    }

    fn infer_pure_binary_opr_type(
        &self,
        pure_binary_opr: PureBinaryOpr,
        lopd_ty: EntityRoutePtr,
        ropd_ty: EntityRoutePtr,
    ) -> SemanticResult<EntityRoutePtr> {
        match lopd_ty {
            EntityRoutePtr::Root(lopd_builtin_ty) => match ropd_ty {
                EntityRoutePtr::Root(ropd_builtin_ty) => self.infer_builtin_pure_binary_opr_type(
                    pure_binary_opr,
                    lopd_builtin_ty,
                    ropd_builtin_ty,
                ),
                EntityRoutePtr::Custom(_) => todo!(),
                EntityRoutePtr::ThisType => todo!(),
            },
            EntityRoutePtr::Custom(lopd_custom_ty) => todo!(),
            EntityRoutePtr::ThisType => todo!(),
        }
    }

    fn infer_builtin_pure_binary_opr_type(
        &self,
        pure_binary_opr: PureBinaryOpr,
        lopd_builtin_ty: RootIdentifier,
        ropd_builtin_ty: RootIdentifier,
    ) -> SemanticResult<EntityRoutePtr> {
        Ok(match pure_binary_opr {
            PureBinaryOpr::Less
            | PureBinaryOpr::Leq
            | PureBinaryOpr::Greater
            | PureBinaryOpr::Geq => {
                if lopd_builtin_ty != ropd_builtin_ty {
                    err!("expect use of \"<, <=, >, >=\" on same types")
                }
                match lopd_builtin_ty {
                    RootIdentifier::I32 | RootIdentifier::F32 => (),
                    _ => err!("expect use of \"<, <=, >, >=\" on i32 or f32"),
                }
                RootIdentifier::Bool
            }
            PureBinaryOpr::Eq | PureBinaryOpr::Neq => {
                if lopd_builtin_ty != ropd_builtin_ty {
                    err!("expect use of \"!=\" on same types")
                }
                RootIdentifier::Bool
            }
            PureBinaryOpr::Shl => todo!(),
            PureBinaryOpr::Shr => todo!(),
            PureBinaryOpr::Add
            | PureBinaryOpr::Sub
            | PureBinaryOpr::Mul
            | PureBinaryOpr::Div
            | PureBinaryOpr::Power => {
                if lopd_builtin_ty != ropd_builtin_ty {
                    err!("expect use of \"+, -, *, /, **\" on same types")
                }
                match lopd_builtin_ty {
                    RootIdentifier::I32 | RootIdentifier::F32 => (),
                    _ => err!("expect use of \"+, -, *, /, **\" on i32 or f32"),
                }
                lopd_builtin_ty
            }
            PureBinaryOpr::And => todo!(),
            PureBinaryOpr::BitAnd => todo!(),
            PureBinaryOpr::Or => todo!(),
            PureBinaryOpr::BitXor => todo!(),
            PureBinaryOpr::BitOr => todo!(),
            PureBinaryOpr::RemEuclid => todo!(),
        }
        .into())
    }

    fn parse_suffix_opr(
        &mut self,
        opr: SuffixOpr,
        opds: &RawExprRange,
    ) -> SemanticResult<LazyExprKind> {
        let this = self.parse_lazy_expr(opds.start)?;
        Ok(match opr {
            SuffixOpr::Incr => todo!(),
            SuffixOpr::Decr => todo!(),
            SuffixOpr::MayReturn => panic!("should handle this case in parse return statement"),
            SuffixOpr::FieldAccess(ranged_ident) => {
                let ty_decl = self.raw_expr_ty_decl(opds.start).unwrap();
                LazyExprKind::Opn {
                    opn_kind: LazyOpnKind::FieldAccess {
                        field_ident: ranged_ident,
                        field_kind: ty_decl.field_kind(ranged_ident.ident),
                    },
                    opds: vec![this],
                }
                // match *ty_decl {
                //     TySignature::Struct {
                //         ref field_vars,
                //         ref field_routines,
                //     } => {
                //         todo!()
                //     }
                //     TySignature::Enum { ref variants } => todo!(),
                //     TySignature::Record {
                //         ref field_vars,
                //         ref field_features,
                //     } => todo!(),
                // }
            }
            SuffixOpr::WithType(_) => todo!(),
        })
    }

    fn parse_call(
        &mut self,
        opd_idx_range: &RawExprRange,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<LazyExprKind> {
        let call = &self.arena()[opd_idx_range.start];
        let input_opd_idx_range = (opd_idx_range.start + 1)..opd_idx_range.end;
        match call.variant {
            RawExprVariant::Entity {
                route: scope, kind, ..
            } => {
                let arguments: Vec<_> = ((opd_idx_range.start + 1)..opd_idx_range.end)
                    .map(|raw| self.parse_lazy_expr(raw))
                    .collect::<SemanticResult<_>>()?;
                let opn_kind = match kind {
                    EntityKind::Module => todo!(),
                    EntityKind::Type(ty_kind) => match ty_kind {
                        TyKind::Enum => todo!(),
                        TyKind::Record => LazyOpnKind::RecordCall(RangedEntityRoute {
                            route: scope,
                            range: call.range(),
                        }),
                        TyKind::Struct => LazyOpnKind::StructCall(RangedEntityRoute {
                            route: scope,
                            range: call.range(),
                        }),
                        TyKind::Primitive => todo!(),
                        TyKind::Other => todo!(),
                        TyKind::Vec => todo!(),
                        TyKind::Array => todo!(),
                    },
                    EntityKind::Trait => todo!(),
                    EntityKind::Routine => LazyOpnKind::RoutineCall(RangedEntityRoute {
                        route: scope,
                        range: call.range(),
                    }),
                    EntityKind::Feature => todo!(),
                    EntityKind::Pattern => todo!(),
                    EntityKind::Literal => todo!(),
                    EntityKind::Member(_) => todo!(),
                };
                Ok(LazyExprKind::Opn {
                    opn_kind,
                    opds: arguments,
                })
            }
            RawExprVariant::Variable { .. } => todo!(),
            RawExprVariant::Unrecognized(_) => todo!(),
            RawExprVariant::PrimitiveLiteral(_) => todo!(),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn {
                opr,
                opds: ref field_opds,
            } => match opr {
                Opr::Binary(_) => todo!(),
                Opr::Prefix(_) => todo!(),
                Opr::Suffix(suffix_opr) => match suffix_opr {
                    SuffixOpr::Incr => todo!(),
                    SuffixOpr::Decr => todo!(),
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::FieldAccess(ranged_ident) => {
                        let this = self.parse_lazy_expr(field_opds.start)?;
                        let inputs = input_opd_idx_range
                            .map(|idx| self.parse_lazy_expr(idx))
                            .collect::<SemanticResult<Vec<_>>>()?;
                        let mut opds = vec![this];
                        opds.extend(inputs);
                        msg_once!("todo: memb call compiled");
                        Ok(LazyExprKind::Opn {
                            opn_kind: LazyOpnKind::MethodCall {
                                method_ident: ranged_ident,
                                method_route: self
                                    .entity_route_sheet()
                                    .call_route(raw_expr_idx)
                                    .unwrap(),
                            },
                            opds,
                        })
                    }
                    SuffixOpr::WithType(_) => todo!(),
                },
                Opr::List(_) => todo!(),
            },
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::This { .. } => todo!(),
            RawExprVariant::FrameVariable { varname, init_row } => todo!(),
        }
    }

    fn parse_element_access(&mut self, opds: RawExprRange) -> SemanticResult<LazyExprKind> {
        Ok(LazyExprKind::Opn {
            opn_kind: LazyOpnKind::ElementAccess,
            opds: opds
                .map(|raw| self.parse_lazy_expr(raw))
                .collect::<SemanticResult<_>>()?,
        })
    }
}
