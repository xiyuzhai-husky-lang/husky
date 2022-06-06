use std::sync::Arc;

use crate::*;
use ast::{RawExprArena, RawExprIdx, RawExprRange, RawExprVariant};
use entity_kind::TyKind;
use entity_route::{EntityKind, EntityRoutePtr, RangedEntityRoute};
use file::FilePtr;
use infer_contract::{InferContract, LazyContract};
use infer_entity_route::InferEntityRoute;
use infer_qualifier::{InferQualifiedTy, LazyExprQualifier};
use text::RangedCustomIdentifier;
use vm::*;
use word::{CustomIdentifier, RootIdentifier};

use super::*;
use semantics_error::*;

pub trait LazyExprParser<'a>: InferEntityRoute + InferContract + InferQualifiedTy {
    fn arena(&self) -> &'a RawExprArena;
    fn file(&self) -> FilePtr;
    fn db(&self) -> &dyn InferQueryGroup;

    fn parse_lazy_expr(&mut self, raw_expr_idx: RawExprIdx) -> SemanticResult<Arc<LazyExpr>> {
        let raw_expr = &self.arena()[raw_expr_idx];
        let kind: LazyExprVariant = match raw_expr.variant {
            RawExprVariant::Variable {
                varname,
                init_range,
            } => {
                let variable_qt = self
                    .lazy_variable_qualified_ty(varname.into(), init_range)
                    .unwrap();
                let contract = self.lazy_expr_contract(raw_expr_idx).unwrap();
                LazyExprVariant::Variable {
                    varname,
                    binding: variable_qt.qual.binding(contract),
                }
            }
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
                EntityKind::EnumLiteral => match entity_route {
                    EntityRoutePtr::Root(RootIdentifier::True) => {
                        LazyExprVariant::PrimitiveLiteral(CopyableValue::Bool(true))
                    }
                    EntityRoutePtr::Root(RootIdentifier::False) => {
                        LazyExprVariant::PrimitiveLiteral(CopyableValue::Bool(false))
                    }
                    EntityRoutePtr::Custom(scope_ref) => {
                        LazyExprVariant::EnumLiteral { entity_route }
                    }
                    _ => todo!(),
                },
                EntityKind::Type(_) => todo!(),
                EntityKind::Trait => todo!(),
                EntityKind::Function { .. } => todo!(),
                EntityKind::Feature => LazyExprVariant::EntityFeature {
                    route: entity_route,
                },
                EntityKind::Member(_) => todo!(),
                EntityKind::Main => panic!(),
            },
            RawExprVariant::CopyableLiteral(value) => LazyExprVariant::PrimitiveLiteral(value),
            RawExprVariant::Bracketed(bracketed_expr) => {
                LazyExprVariant::Bracketed(self.parse_lazy_expr(bracketed_expr)?)
            }
            RawExprVariant::Opn {
                opn_variant: ref opr,
                ref opds,
                ..
            } => self.parse_opn(opr, opds, raw_expr_idx)?,
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::ThisValue {
                opt_this_ty,
                opt_this_liason,
            } => LazyExprVariant::ThisValue {
                binding: {
                    let this_contract = self.lazy_expr_contract(raw_expr_idx).unwrap();
                    let this_qual = LazyExprQualifier::parameter_use_lazy_qualifier(
                        opt_this_liason.unwrap(),
                        self.decl_db().is_copyable(opt_this_ty.unwrap()).unwrap(),
                        this_contract,
                    )
                    .unwrap();
                    this_qual.binding(this_contract)
                },
            },
            RawExprVariant::ThisField {
                field_ident,
                opt_this_ty,
                opt_this_liason,
                field_liason,
                opt_field_ty,
            } => {
                let field_contract = self.lazy_expr_contract(raw_expr_idx).unwrap();
                let field_qt = self.lazy_expr_qualified_ty(raw_expr_idx).unwrap();
                let this_contract = LazyContract::field_access_lazy_contract(
                    field_liason,
                    field_contract,
                    self.decl_db()
                        .is_copyable(opt_field_ty.unwrap().route)
                        .unwrap(),
                    self.arena()[raw_expr_idx].range,
                )?;
                let this_qual = LazyExprQualifier::parameter_use_lazy_qualifier(
                    opt_this_liason.unwrap(),
                    self.decl_db().is_copyable(opt_this_ty.unwrap())?,
                    this_contract,
                    // raw_expr.range,
                )
                .unwrap();
                let ty_decl = self.decl_db().ty_decl(opt_this_ty.unwrap()).unwrap();
                LazyExprVariant::ThisField {
                    field_ident,
                    field_idx: ty_decl.field_idx(field_ident.ident),
                    this_ty: opt_this_ty.unwrap(),
                    this_binding: this_qual.binding(this_contract),
                    field_binding: { field_qt.qual.binding(field_contract) },
                }
            }
            RawExprVariant::FrameVariable {
                varname,
                init_range: init_row,
            } => todo!(),
        };
        Ok(Arc::new(LazyExpr {
            range: raw_expr.range().clone(),
            qualified_ty: self.lazy_expr_qualified_ty(raw_expr_idx)?,
            variant: kind,
            file: self.file(),
            instruction_id: Default::default(),
        }))
    }

    fn parse_opn(
        &mut self,
        opr: &RawOpnVariant,
        opds: &RawExprRange,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<LazyExprVariant> {
        match opr {
            RawOpnVariant::Binary(opr) => self.parse_binary_opr(*opr, opds),
            RawOpnVariant::Prefix(_) => todo!(),
            RawOpnVariant::Suffix(opr) => self.parse_suffix_opr(*opr, opds),
            RawOpnVariant::List(opr) => match opr {
                ListOpr::TupleInit => todo!(),
                ListOpr::NewVec => todo!(),
                ListOpr::NewDict => todo!(),
                ListOpr::Call => self.parse_function_call(opds, raw_expr_idx),
                ListOpr::Index => self.parse_element_access(opds.clone(), raw_expr_idx),
                ListOpr::ModuloIndex => todo!(),
                ListOpr::StructInit => todo!(),
                ListOpr::MethodCall { ranged_ident, .. } => self.parse_method_call(
                    opds.start,
                    (opds.start + 1)..opds.end,
                    *ranged_ident,
                    raw_expr_idx,
                ),
            },
            RawOpnVariant::FieldAccess(field_ident) => {
                self.parse_field_access(*field_ident, opds.start, raw_expr_idx)
            }
        }
    }

    fn parse_binary_opr(
        &mut self,
        opr: BinaryOpr,
        raw_opds: &RawExprRange,
    ) -> SemanticResult<LazyExprVariant> {
        // let raw_opds = &self.arena()[raw_opds];
        let lopd = self.parse_lazy_expr(raw_opds.start)?;
        let ropd = self.parse_lazy_expr(raw_opds.start + 1)?;
        let output_type = match opr {
            BinaryOpr::Pure(pure_binary_opr) => {
                self.infer_pure_binary_opr_type(pure_binary_opr, lopd.ty(), ropd.ty())?
            }
            BinaryOpr::Assign(opt_binary) => {
                if let Some(pure_binary_opr) = opt_binary {
                    if lopd.ty()
                        != self.infer_pure_binary_opr_type(pure_binary_opr, lopd.ty(), ropd.ty())?
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
        Ok(LazyExprVariant::Opn {
            opn_kind: LazyOpnKind::Binary {
                opr,
                this: lopd.ty(),
            },
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
    ) -> SemanticResult<LazyExprVariant> {
        let this = self.parse_lazy_expr(opds.start)?;
        Ok(match opr {
            SuffixOpr::Incr => todo!(),
            SuffixOpr::Decr => todo!(),
            SuffixOpr::AsTy(_) => todo!(),
        })
    }

    fn parse_field_access(
        &mut self,
        field_ident: RangedCustomIdentifier,
        this_idx: RawExprIdx,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<LazyExprVariant> {
        let this = self.parse_lazy_expr(this_idx)?;
        let ty_decl = self.raw_expr_deref_ty_decl(this_idx).unwrap();
        let this_ty_decl = self.decl_db().ty_decl(this.ty()).unwrap();
        let field_decl = this_ty_decl.field_decl(field_ident).unwrap();
        let field_liason = field_decl.liason;
        let field_contract = self.lazy_expr_contract(raw_expr_idx).unwrap();
        let field_qt = self.lazy_expr_qualified_ty(raw_expr_idx).unwrap();
        Ok(LazyExprVariant::Opn {
            opn_kind: LazyOpnKind::FieldAccess {
                field_ident,
                field_kind: ty_decl.field_kind(field_ident.ident),
                field_binding: field_qt.qual.binding(field_contract),
            },
            opds: vec![this],
        })
    }

    fn parse_function_call(
        &mut self,
        opd_idx_range: &RawExprRange,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<LazyExprVariant> {
        let call = &self.arena()[opd_idx_range.start];
        let input_opd_idx_range = (opd_idx_range.start + 1)..opd_idx_range.end;
        match call.variant {
            RawExprVariant::Entity { route, kind, .. } => {
                let arguments: Vec<_> = ((opd_idx_range.start + 1)..opd_idx_range.end)
                    .map(|raw| self.parse_lazy_expr(raw))
                    .collect::<SemanticResult<_>>()?;
                let opn_kind = match kind {
                    EntityKind::Module => todo!(),
                    EntityKind::Type(ty_kind) => match ty_kind {
                        TyKind::Enum => todo!(),
                        TyKind::Record => LazyOpnKind::RecordCall(RangedEntityRoute {
                            route,
                            range: call.range(),
                        }),
                        TyKind::Struct => LazyOpnKind::StructCall(RangedEntityRoute {
                            route,
                            range: call.range(),
                        }),
                        TyKind::Primitive => todo!(),
                        TyKind::Other => todo!(),
                        TyKind::Vec => todo!(),
                        TyKind::Array => todo!(),
                    },
                    EntityKind::Trait => todo!(),
                    EntityKind::Function { is_lazy } => {
                        if is_lazy {
                            todo!()
                        } else {
                            LazyOpnKind::NormalRoutineCall(RangedEntityRoute {
                                route,
                                range: call.range(),
                            })
                        }
                    }
                    EntityKind::Feature => todo!(),
                    EntityKind::EnumLiteral => todo!(),
                    EntityKind::Member(_) => todo!(),
                    EntityKind::Main => panic!(),
                };
                Ok(LazyExprVariant::Opn {
                    opn_kind,
                    opds: arguments,
                })
            }
            RawExprVariant::Variable { .. } => todo!(),
            RawExprVariant::Unrecognized(_) => panic!(),
            RawExprVariant::CopyableLiteral(_) => todo!(),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn { .. } => todo!(),
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::ThisValue { .. } => todo!(),
            RawExprVariant::ThisField { .. } => todo!(),
            RawExprVariant::FrameVariable {
                varname,
                init_range: init_row,
            } => todo!(),
        }
    }

    fn parse_method_call(
        &mut self,
        this: RawExprIdx,
        inputs: RawExprRange,
        method_ident: RangedCustomIdentifier,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<LazyExprVariant> {
        let this = self.parse_lazy_expr(this)?;
        let output_binding = {
            let output_contract = self.lazy_expr_contract(raw_expr_idx).unwrap();
            let output_qt = self.lazy_expr_qualified_ty(raw_expr_idx).unwrap();
            output_qt.qual.binding(output_contract)
        };
        let inputs = inputs
            .map(|idx| self.parse_lazy_expr(idx))
            .collect::<SemanticResult<Vec<_>>>()?;
        let mut opds = vec![this];
        opds.extend(inputs);
        Ok(LazyExprVariant::Opn {
            opn_kind: LazyOpnKind::MethodCall {
                method_ident,
                method_route: self.entity_route_sheet().call_route(raw_expr_idx).unwrap(),
                output_binding,
            },
            opds,
        })
    }

    fn parse_element_access(
        &mut self,
        opds: RawExprRange,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<LazyExprVariant> {
        let element_ty = self.raw_expr_deref_ty(raw_expr_idx).unwrap();
        Ok(LazyExprVariant::Opn {
            opn_kind: LazyOpnKind::ElementAccess {
                element_binding: {
                    let element_contract = self.lazy_expr_contract(raw_expr_idx).unwrap();
                    let element_qt = self.lazy_expr_qualified_ty(raw_expr_idx).unwrap();
                    element_qt.qual.binding(element_contract)
                },
            },
            opds: opds
                .map(|raw| self.parse_lazy_expr(raw))
                .collect::<SemanticResult<_>>()?,
        })
    }
}
