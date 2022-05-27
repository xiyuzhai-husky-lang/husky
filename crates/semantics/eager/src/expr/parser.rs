use ast::{RawExprArena, RawExprIdx, RawExprRange, RawExprVariant};
use entity_route::{EntityKind, EntityRouteKind, EntityRoutePtr};
use file::FilePtr;
use infer_contract::InferContract;
use infer_entity_route::InferEntityRoute;
use infer_qualifier::InferQualifiedTy;
use text::RangedCustomIdentifier;
use vm::*;
use word::RootIdentifier;

use crate::*;
use semantics_error::{derived_unwrap, err};

use super::EagerOpnVariant;

pub trait EagerExprParser<'a>: InferEntityRoute + InferContract + InferQualifiedTy {
    fn arena(&self) -> &'a RawExprArena;
    fn file(&self) -> FilePtr;
    // fn db(&self) -> &'a dyn InferQueryGroup;

    fn parse_eager_expr(&mut self, raw_expr_idx: RawExprIdx) -> SemanticResult<Arc<EagerExpr>> {
        let raw_expr = &self.arena()[raw_expr_idx];
        let kind = match raw_expr.variant {
            RawExprVariant::Variable { varname, .. } => EagerExprVariant::Variable(varname),
            RawExprVariant::FrameVariable {
                varname,
                init_range: init_row,
            } => EagerExprVariant::Variable(varname),
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
                EntityKind::Feature => {
                    panic!("what")
                }
                EntityKind::Member(_) => todo!(),
                EntityKind::Main => panic!(),
            },
            RawExprVariant::CopyableLiteral(value) => EagerExprVariant::PrimitiveLiteral(value),
            RawExprVariant::Bracketed(expr) => {
                EagerExprVariant::Bracketed(self.parse_eager_expr(expr)?)
            }
            RawExprVariant::Opn {
                ref opr, ref opds, ..
            } => self.parse_opn(opr, opds, raw_expr_idx)?,
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::This { .. } => EagerExprVariant::ThisData,
        };
        if let Err(e) = self.raw_expr_ty(raw_expr_idx) {
            p!(self.contract_sheet());
            p!(self.file(), raw_expr, raw_expr_idx);
            panic!()
        }
        Ok(Arc::new(EagerExpr {
            range: raw_expr.range().clone(),
            ty: self.raw_expr_ty(raw_expr_idx).unwrap(),
            variant: kind,
            file: self.file(),
            instruction_id: Default::default(),
            contract: derived_unwrap!(self.eager_expr_contract(raw_expr_idx)),
            qualified_ty: self.eager_expr_qualified_ty(raw_expr_idx)?,
        }))
    }

    fn parse_opn(
        &mut self,
        opr: &Opr,
        opds: &RawExprRange,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<EagerExprVariant> {
        match opr {
            Opr::Binary(opr) => self.parse_binary_opr(*opr, opds),
            Opr::Prefix(opr) => self.parse_prefix_opr(*opr, opds),
            Opr::Suffix(opr) => self.parse_suffix_opr(*opr, opds),
            Opr::List(opr) => match opr {
                ListOpr::TupleInit => todo!(),
                ListOpr::NewVec => todo!(),
                ListOpr::NewDict => todo!(),
                ListOpr::Call => self.parse_call(opds.clone(), raw_expr_idx),
                ListOpr::MethodCall { ranged_ident, .. } => self.parse_method_call(
                    opds.start,
                    (opds.start + 1)..opds.end,
                    *ranged_ident,
                    raw_expr_idx,
                ),
                ListOpr::Index => self.parse_element_access(opds.clone()),
                ListOpr::ModuloIndex => todo!(),
                ListOpr::StructInit => todo!(),
            },
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
                this_ty: lopd.ty,
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
                this_ty: opd.ty,
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
                this_ty: opd.ty,
            },
            opds: vec![opd],
        })
    }

    fn parse_call(
        &mut self,
        opds: RawExprRange,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<EagerExprVariant> {
        let call = &self.arena()[opds.start];
        let input_opd_idx_range = (opds.start + 1)..opds.end;
        match call.variant {
            RawExprVariant::Entity {
                route: scope,
                kind: EntityKind::Function { is_lazy },
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
                let signature = derived_unwrap!(self.decl_db().call_decl(scope));
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
            RawExprVariant::Entity { kind, .. } => {
                p!(kind);
                todo!()
            }
            RawExprVariant::Variable { .. } => todo!(),
            RawExprVariant::Unrecognized(_) => todo!(),
            RawExprVariant::CopyableLiteral(_) => todo!(),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn {
                ref opr,
                opds: ref field_opds,
            } => match opr {
                Opr::Binary(_) => todo!(),
                Opr::Prefix(_) => todo!(),
                Opr::Suffix(suffix_opr) => todo!(),
                Opr::List(_) => todo!(),
            },
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::This { .. } => todo!(),
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
    ) -> SemanticResult<EagerExprVariant> {
        let this = self.parse_eager_expr(this)?;
        let inputs = inputs
            .map(|idx| self.parse_eager_expr(idx))
            .collect::<SemanticResult<Vec<_>>>()?;
        let this_ty_decl = self.decl_db().ty_decl(this.ty).unwrap();
        let mut opds = vec![this];
        opds.extend(inputs);
        emsg_once!("todo: memb call compiled");
        Ok(EagerExprVariant::Opn {
            opn_variant: EagerOpnVariant::MethodCall {
                method_ident,
                this_ty_decl,
                method_route: self.entity_route_sheet().call_route(raw_expr_idx).unwrap(),
            },
            opds,
        })
    }

    fn parse_element_access(&mut self, opds: RawExprRange) -> SemanticResult<EagerExprVariant> {
        Ok(EagerExprVariant::Opn {
            opn_variant: EagerOpnVariant::ElementAccess,
            opds: opds
                .map(|raw| self.parse_eager_expr(raw))
                .collect::<SemanticResult<_>>()?,
        })
    }
}
