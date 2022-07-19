use husky_ast::{RawExprArena, RawExprIdx, RawExprRange, RawExprVariant};
use husky_entity_route::{EntityKind, EntityRouteKind, EntityRoutePtr};
use husky_file::FilePtr;
use husky_infer_entity_route::InferEntityRoute;
use husky_infer_qualified_ty::{EagerExprQualifier, EagerVariableQualifier, InferQualifiedTy};
use husky_text::{BindTextRangeInto, RangedCustomIdentifier};
use infer_contract::{EagerContract, InferContract};
use vm::*;
use word::{ContextualIdentifier, Identifier, RootIdentifier};

use crate::*;
use semantics_error::{derived_unwrap, err};

use super::EagerOpnVariant;

pub trait EagerExprParser<'a>: InferEntityRoute + InferContract + InferQualifiedTy {
    fn arena(&self) -> &'a RawExprArena;
    fn file(&self) -> FilePtr;

    fn parse_eager_expr(&mut self, raw_expr_idx: RawExprIdx) -> SemanticResult<Arc<EagerExpr>> {
        let raw_expr = &self.arena()[raw_expr_idx];
        let variant = match raw_expr.variant {
            RawExprVariant::Variable {
                varname,
                init_range,
            } => {
                let variable_qt = self
                    .eager_variable_qualified_ty(varname.into(), init_range)
                    .unwrap();

                let contract = match self.eager_expr_contract(raw_expr_idx) {
                    Ok(contract) => contract,
                    Err(_) => {
                        p!(self.file(), raw_expr_idx, raw_expr);
                        panic!("what's the contract?");
                    }
                };
                EagerExprVariant::Variable {
                    varname,
                    binding: variable_qt.qual.binding(contract),
                }
            }
            RawExprVariant::FrameVariable { varname, .. } => EagerExprVariant::Variable {
                varname,
                binding: Binding::Copy,
            },
            RawExprVariant::Unrecognized(ident) => {
                err!(format!(
                    "unrecognized identifier {} at {}:{:?}",
                    ident,
                    self.file().to_str().unwrap(),
                    raw_expr.range()
                ))
            }
            RawExprVariant::Entity { route, kind } => match kind {
                EntityKind::Module => todo!(),
                EntityKind::EnumLiteral => match route {
                    EntityRoutePtr::Root(RootIdentifier::True) => {
                        EagerExprVariant::PrimitiveLiteral(CopyableValue::Bool(true))
                    }
                    EntityRoutePtr::Root(RootIdentifier::False) => {
                        EagerExprVariant::PrimitiveLiteral(CopyableValue::Bool(false))
                    }
                    EntityRoutePtr::Custom(_) => EagerExprVariant::EnumKindLiteral(route),
                    _ => todo!(),
                },
                EntityKind::Type(_) => todo!(),
                EntityKind::Trait => todo!(),
                EntityKind::Function { .. } => todo!(),
                EntityKind::Feature => EagerExprVariant::EntityFeature { route },
                EntityKind::Member(_) => todo!(),
                EntityKind::Main => panic!(),
            },
            RawExprVariant::CopyableLiteral(value) => EagerExprVariant::PrimitiveLiteral(value),
            RawExprVariant::Bracketed(expr) => {
                EagerExprVariant::Bracketed(self.parse_eager_expr(expr)?)
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
            } => EagerExprVariant::ThisValue {
                binding: {
                    let this_contract = self.eager_expr_contract(raw_expr_idx).unwrap();
                    let this_qual = EagerExprQualifier::parameter_use_eager_qualifier(
                        self.decl_db(),
                        opt_this_ty.unwrap(),
                        opt_this_liason.unwrap(),
                        this_contract,
                        raw_expr.range,
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
                let field_contract = self.eager_expr_contract(raw_expr_idx).unwrap();
                let field_qt = self.eager_expr_qualified_ty(raw_expr_idx).unwrap();
                let is_field_copyable = self
                    .decl_db()
                    .is_copyable(opt_field_ty.unwrap().route)
                    .unwrap();
                let this_contract = EagerContract::field_access_this_eager_contract(
                    field_liason,
                    field_contract,
                    is_field_copyable,
                    self.arena()[raw_expr_idx].range,
                )?;
                let this_qual = EagerExprQualifier::parameter_use_eager_qualifier(
                    self.decl_db(),
                    opt_this_ty.unwrap(),
                    opt_this_liason.unwrap(),
                    this_contract,
                    raw_expr.range,
                )
                .unwrap();
                let ty_decl = self.decl_db().ty_decl(opt_this_ty.unwrap()).unwrap();
                EagerExprVariant::ThisField {
                    field_ident,
                    field_idx: ty_decl.field_idx(field_ident.ident),
                    this_ty: opt_this_ty.unwrap(),
                    this_binding: this_qual.binding(this_contract),
                    field_binding: { field_qt.qual.binding(field_contract) },
                }
            }
        };
        if let Err(e) = self.raw_expr_intrinsic_ty(raw_expr_idx) {
            p!(self.contract_sheet());
            p!(self.file(), raw_expr, raw_expr_idx);
            panic!()
        }
        let qualified_ty = match self.eager_expr_qualified_ty(raw_expr_idx) {
            Ok(qualified_ty) => qualified_ty,
            Err(e) => {
                p!(raw_expr, e);
                todo!()
            }
        };
        Ok(Arc::new(EagerExpr {
            range: raw_expr.range().clone(),
            variant,
            file: self.file(),
            instruction_id: Default::default(),
            qualified_ty,
            contract: self.eager_expr_contract(raw_expr_idx).unwrap(),
        }))
    }

    fn parse_opn(
        &mut self,
        opr: &RawOpnVariant,
        opds: &RawExprRange,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<EagerExprVariant> {
        match opr {
            RawOpnVariant::Binary(opr) => self.parse_binary_opr(*opr, opds),
            RawOpnVariant::Prefix(opr) => self.parse_prefix_opr(*opr, opds),
            RawOpnVariant::Suffix(opr) => self.parse_suffix_opr(*opr, opds),
            RawOpnVariant::List(opr) => match opr {
                ListOpr::TupleInit => todo!(),
                ListOpr::NewVec => todo!(),
                ListOpr::NewDict => todo!(),
                ListOpr::Call => self.parse_function_call(opds.clone(), raw_expr_idx),
                ListOpr::MethodCall { ranged_ident, .. } => self.parse_method_call(
                    opds.start,
                    (opds.start + 1)..opds.end,
                    *ranged_ident,
                    raw_expr_idx,
                ),
                ListOpr::Index => self.parse_element_access(opds.clone(), raw_expr_idx),
                ListOpr::ModuloIndex => todo!(),
                ListOpr::StructInit => todo!(),
            },
            RawOpnVariant::Field(field_ident) => {
                self.parse_field_access(*field_ident, opds, raw_expr_idx)
            }
        }
    }

    fn parse_binary_opr(
        &mut self,
        opr: BinaryOpr,
        raw_opd_idx_range: &RawExprRange,
    ) -> SemanticResult<EagerExprVariant> {
        let raw_opds = &self.arena()[raw_opd_idx_range];
        let lopd = self.parse_eager_expr(raw_opd_idx_range.start)?;
        let ropd = self.parse_eager_expr(raw_opd_idx_range.start + 1)?;
        Ok(EagerExprVariant::Opn {
            opn_variant: EagerOpnVariant::Binary {
                opr,
                this_ty: lopd.ty(),
            },
            opds: vec![lopd, ropd],
        })
    }

    fn parse_prefix_opr(
        &mut self,
        opr: PrefixOpr,
        raw_opds: &RawExprRange,
    ) -> SemanticResult<EagerExprVariant> {
        let opd_idx = raw_opds.start;
        let opd = self.parse_eager_expr(opd_idx)?;
        Ok(EagerExprVariant::Opn {
            opn_variant: EagerOpnVariant::Prefix {
                opr,
                this_ty: opd.ty(),
            },
            opds: vec![opd],
        })
    }

    fn parse_suffix_opr(
        &mut self,
        opr: SuffixOpr,
        raw_opds: &RawExprRange,
    ) -> SemanticResult<EagerExprVariant> {
        let opd_idx = raw_opds.start;
        let opd = self.parse_eager_expr(opd_idx)?;
        Ok(EagerExprVariant::Opn {
            opn_variant: EagerOpnVariant::Suffix {
                opr,
                this_ty: opd.ty(),
            },
            opds: vec![opd],
        })
    }

    fn parse_field_access(
        &mut self,
        field_ident: RangedCustomIdentifier,
        raw_opds: &RawExprRange,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<EagerExprVariant> {
        let this_idx = raw_opds.start;
        let this = self.parse_eager_expr(this_idx)?;
        let this_ty_decl = self.decl_db().ty_decl(this.ty()).unwrap();
        let field_decl = this_ty_decl.field_decl(field_ident).unwrap();
        let field_liason = field_decl.liason;
        let field_contract = self.eager_expr_contract(raw_expr_idx).unwrap();
        let field_qt = self.eager_expr_qualified_ty(raw_expr_idx).unwrap();
        Ok(EagerExprVariant::Opn {
            opn_variant: EagerOpnVariant::FieldAccess {
                field_ident,
                this_ty: this.ty(),
                field_liason,
                field_binding: field_qt.qual.binding(field_contract),
                field_kind: field_decl.field_kind,
            },
            opds: vec![this],
        })
    }

    fn parse_function_call(
        &mut self,
        opds: RawExprRange,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<EagerExprVariant> {
        let call = &self.arena()[opds.start];
        let input_opd_idx_range = (opds.start + 1)..opds.end;
        match call.variant {
            RawExprVariant::Entity {
                route: scope,
                kind:
                    EntityKind::Function {
                        requires_lazy: is_lazy,
                    },
                ..
            } => {
                if is_lazy {
                    todo!()
                }
                let arguments: Vec<_> = input_opd_idx_range
                    .clone()
                    .map(|raw| self.parse_eager_expr(raw))
                    .collect::<SemanticResult<_>>()?;
                Ok(EagerExprVariant::Opn {
                    opn_variant: EagerOpnVariant::RoutineCall(RangedEntityRoute {
                        route: scope,
                        range: call.range(),
                    }),
                    opds: arguments,
                })
            }
            RawExprVariant::Entity {
                route: scope,
                kind: EntityKind::Type(_),
                ..
            } => {
                let signature = derived_unwrap!(self.decl_db().call_form_decl(scope));
                let arguments: Vec<_> = input_opd_idx_range
                    .enumerate()
                    .map(|(i, raw)| self.parse_eager_expr(raw))
                    .collect::<SemanticResult<_>>()?;
                Ok(EagerExprVariant::Opn {
                    opn_variant: EagerOpnVariant::TypeCall {
                        ranged_ty: RangedEntityRoute {
                            route: scope,
                            range: call.range(),
                        },
                        ty_decl: derived_unwrap!(self.decl_db().ty_decl(scope)),
                    },
                    opds: arguments,
                })
            }
            _ => todo!(),
        }
    }

    fn parse_method_call(
        &mut self,
        this: RawExprIdx,
        arguments: RawExprRange,
        method_ident: RangedCustomIdentifier,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<EagerExprVariant> {
        let this = self.parse_eager_expr(this)?;
        let this_ty_decl = self.decl_db().ty_decl(this.ty()).unwrap();
        let output_binding = {
            let output_contract = self.eager_expr_contract(raw_expr_idx).unwrap();
            let output_qt = self.eager_expr_qualified_ty(raw_expr_idx).unwrap();
            output_qt.qual.binding(output_contract)
        };
        let opds = {
            let mut opds = vec![this];
            let arguments = arguments
                .map(|idx| self.parse_eager_expr(idx))
                .collect::<SemanticResult<Vec<_>>>()?;
            opds.extend(arguments);
            opds
        };
        Ok(EagerExprVariant::Opn {
            opn_variant: EagerOpnVariant::MethodCall {
                method_ident,
                this_ty_decl,
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
    ) -> SemanticResult<EagerExprVariant> {
        let element_ty = self.raw_expr_intrinsic_ty(raw_expr_idx).unwrap();
        Ok(EagerExprVariant::Opn {
            opn_variant: EagerOpnVariant::Index {
                element_binding: {
                    let element_qt = self.eager_expr_qualified_ty(raw_expr_idx).unwrap();
                    let contract = self.eager_expr_contract(raw_expr_idx).unwrap();
                    element_qt.qual.binding(contract)
                },
            },
            opds: opds
                .map(|raw| self.parse_eager_expr(raw))
                .collect::<SemanticResult<_>>()?,
        })
    }
}
