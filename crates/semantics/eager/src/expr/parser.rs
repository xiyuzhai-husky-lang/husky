use ast::{RawExprArena, RawExprIdx, RawExprRange, RawExprVariant};
use entity_route::{EntityKind, EntityRouteKind, EntityRoutePtr};
use file::FilePtr;
use infer_contract::InferContract;
use infer_entity_route::InferEntityRoute;
use syntax_types::{ListOpr, Opr, SuffixOpr};
use vm::{BinaryOpr, EagerContract, PrimitiveValue};
use word::RootIdentifier;

use crate::*;
use semantics_error::{err, try_infer};

use super::EagerOpnKind;

pub trait EagerExprParser<'a>: InferEntityRoute + InferContract {
    fn arena(&self) -> &'a RawExprArena;
    fn file(&self) -> FilePtr;
    // fn db(&self) -> &'a dyn InferQueryGroup;

    fn parse_eager_expr(&mut self, raw_expr_idx: RawExprIdx) -> SemanticResult<Arc<EagerExpr>> {
        let raw_expr = &self.arena()[raw_expr_idx];
        let kind = match raw_expr.variant {
            RawExprVariant::Variable { varname, .. } => EagerExprVariant::Variable(varname),
            RawExprVariant::Unrecognized(ident) => {
                err!(format!(
                    "unrecognized identifier {} at {}:{:?}",
                    ident,
                    self.file().to_str().unwrap(),
                    raw_expr.range()
                ))
            }
            RawExprVariant::Entity { route: scope, kind } => match kind {
                EntityKind::Module => todo!(),
                EntityKind::Literal => match scope {
                    EntityRoutePtr::Root(RootIdentifier::True) => {
                        EagerExprVariant::PrimitiveLiteral(PrimitiveValue::Bool(true))
                    }
                    EntityRoutePtr::Root(RootIdentifier::False) => {
                        EagerExprVariant::PrimitiveLiteral(PrimitiveValue::Bool(false))
                    }
                    EntityRoutePtr::Custom(_) => todo!(),
                    _ => todo!(),
                },
                EntityKind::Type(_) => todo!(),
                EntityKind::Trait => todo!(),
                EntityKind::Routine => todo!(),
                EntityKind::Feature => {
                    panic!("what")
                }
                EntityKind::Pattern => todo!(),
                EntityKind::TypeMember => todo!(),
                EntityKind::Member => todo!(),
            },
            RawExprVariant::PrimitiveLiteral(value) => EagerExprVariant::PrimitiveLiteral(value),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn { opr, ref opds } => self.parse_opn(opr, opds, raw_expr_idx)?,
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::This { .. } => EagerExprVariant::This,
            RawExprVariant::FrameVariable { varname, init_row } => todo!(),
        };
        Ok(Arc::new(EagerExpr {
            range: raw_expr.range().clone(),
            ty: try_infer!(self.expr_ty_result(raw_expr_idx)),
            variant: kind,
            file: self.file(),
            instruction_id: Default::default(),
            contract: try_infer!(self.eager_expr_contract_result(raw_expr_idx)),
        }))
    }

    fn parse_opn(
        &mut self,
        opr: Opr,
        opds: &RawExprRange,
        raw_expr_idx: RawExprIdx,
    ) -> SemanticResult<EagerExprVariant> {
        match opr {
            Opr::Binary(opr) => self.parse_binary_opr(opr, opds),
            Opr::Prefix(_) => todo!(),
            Opr::Suffix(opr) => self.parse_suffix_opr(opr, opds),
            Opr::List(opr) => match opr {
                ListOpr::TupleInit => todo!(),
                ListOpr::NewVec => todo!(),
                ListOpr::NewDict => todo!(),
                ListOpr::Call => self.parse_call(opds.clone(), raw_expr_idx),
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
            opn_kind: EagerOpnKind::Binary { opr, this: lopd.ty },
            opds: vec![lopd, ropd],
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
            opn_kind: EagerOpnKind::Suffix { opr, this: opd.ty },
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
                kind: EntityKind::Routine,
                ..
            } => {
                let arguments: Vec<_> = input_opd_idx_range
                    .clone()
                    .map(|raw| self.parse_eager_expr(raw))
                    .collect::<SemanticResult<_>>()?;
                Ok(EagerExprVariant::Opn {
                    opn_kind: EagerOpnKind::RoutineCall(RangedEntityRoute {
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
                let signature = try_infer!(self.decl_db().call_decl(scope));
                let arguments: Vec<_> = input_opd_idx_range
                    .enumerate()
                    .map(|(i, raw)| self.parse_eager_expr(raw))
                    .collect::<SemanticResult<_>>()?;
                Ok(EagerExprVariant::Opn {
                    opn_kind: EagerOpnKind::TypeCall {
                        ranged_ty: RangedEntityRoute {
                            route: scope,
                            range: call.range(),
                        },
                        ty_decl: try_infer!(self.decl_db().type_decl(scope)),
                    },
                    opds: arguments,
                })
            }
            RawExprVariant::Entity { .. } => todo!(),
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
                    SuffixOpr::MembAccess(field_ident) => {
                        let this = self.parse_eager_expr(field_opds.start)?;
                        let inputs = input_opd_idx_range
                            .map(|idx| self.parse_eager_expr(idx))
                            .collect::<SemanticResult<Vec<_>>>()?;
                        let this_ty_decl = self.decl_db().type_decl(this.ty).unwrap();
                        let mut opds = vec![this];
                        opds.extend(inputs);
                        msg_once!("todo: memb call compiled");
                        Ok(EagerExprVariant::Opn {
                            opn_kind: EagerOpnKind::MethodCall {
                                method_ident: field_ident,
                                this_ty_decl,
                                method_route: self
                                    .entity_route_sheet()
                                    .call_route_result(raw_expr_idx)
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

    fn parse_element_access(&mut self, opds: RawExprRange) -> SemanticResult<EagerExprVariant> {
        Ok(EagerExprVariant::Opn {
            opn_kind: EagerOpnKind::ElementAccess,
            opds: opds
                .map(|raw| self.parse_eager_expr(raw))
                .collect::<SemanticResult<_>>()?,
        })
    }
}
