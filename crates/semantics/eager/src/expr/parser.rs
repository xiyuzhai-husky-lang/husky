use ast::{RawExprArena, RawExprIdx, RawExprKind, RawExprRange};
use entity_route::{EntityRouteKind, EntityRoutePtr, RawEntityKind};
use file::FilePtr;
use syntax_types::{ListOpr, Opr, SuffixOpr};
use vm::{BinaryOpr, EagerContract, PrimitiveValue};
use word::BuiltinIdentifier;

use crate::*;
use semantics_error::{err, try_infer};

use super::EagerOpnKind;

pub trait EagerExprParser<'a> {
    fn arena(&self) -> &'a RawExprArena;
    fn db(&self) -> &'a dyn InferQueryGroup;
    fn file(&self) -> FilePtr;

    fn parse_eager_expr(&mut self, raw_expr_idx: RawExprIdx) -> SemanticResult<Arc<EagerExpr>> {
        let raw_expr = &self.arena()[raw_expr_idx];
        let kind = match raw_expr.kind {
            RawExprKind::Variable { varname, .. } => EagerExprKind::Variable(varname),
            RawExprKind::Unrecognized(ident) => {
                err!(format!(
                    "unrecognized identifier {} at {}:{:?}",
                    ident,
                    self.file().to_str().unwrap(),
                    raw_expr.range()
                ))
            }
            RawExprKind::Scope { scope, kind } => match kind {
                RawEntityKind::Module => todo!(),
                RawEntityKind::Literal => match scope {
                    EntityRoutePtr::Builtin(BuiltinIdentifier::True) => {
                        EagerExprKind::PrimitiveLiteral(PrimitiveValue::Bool(true))
                    }
                    EntityRoutePtr::Builtin(BuiltinIdentifier::False) => {
                        EagerExprKind::PrimitiveLiteral(PrimitiveValue::Bool(false))
                    }
                    EntityRoutePtr::Custom(_) => todo!(),
                    _ => todo!(),
                },
                RawEntityKind::Type(_) => todo!(),
                RawEntityKind::Trait => todo!(),
                RawEntityKind::Routine => todo!(),
                RawEntityKind::Feature => {
                    panic!("what")
                }
                RawEntityKind::Pattern => todo!(),
            },
            RawExprKind::PrimitiveLiteral(value) => EagerExprKind::PrimitiveLiteral(value),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, ref opds } => self.parse_opn(opr, opds)?,
            RawExprKind::Lambda(_, _) => todo!(),
            RawExprKind::This { .. } => EagerExprKind::This,
        };
        Ok(Arc::new(EagerExpr {
            range: raw_expr.range().clone(),
            ty: try_infer!(self.db().expr_ty_result(self.file(), raw_expr_idx)),
            kind,
            file: self.file(),
            instruction_id: Default::default(),
            contract: try_infer!(self
                .db()
                .eager_expr_contract_result(self.file(), raw_expr_idx)),
        }))
    }

    fn parse_opn(&mut self, opr: Opr, opds: &RawExprRange) -> SemanticResult<EagerExprKind> {
        match opr {
            Opr::Binary(opr) => self.parse_binary_opr(opr, opds),
            Opr::Prefix(_) => todo!(),
            Opr::Suffix(opr) => self.parse_suffix_opr(opr, opds),
            Opr::List(opr) => match opr {
                ListOpr::TupleInit => todo!(),
                ListOpr::NewVec => todo!(),
                ListOpr::NewDict => todo!(),
                ListOpr::Call => self.parse_call(opds.clone()),
                ListOpr::Index => todo!(),
                ListOpr::ModuloIndex => todo!(),
                ListOpr::StructInit => todo!(),
            },
        }
    }

    fn parse_binary_opr(
        &mut self,
        opr: BinaryOpr,
        raw_opd_idx_range: &RawExprRange,
    ) -> SemanticResult<EagerExprKind> {
        let raw_opds = &self.arena()[raw_opd_idx_range];
        let lopd = self.parse_eager_expr(raw_opd_idx_range.start)?;
        let ropd = self.parse_eager_expr(raw_opd_idx_range.start + 1)?;
        Ok(EagerExprKind::Opn {
            opn_kind: EagerOpnKind::Binary { opr, this: lopd.ty },
            opds: vec![lopd, ropd],
        })
    }

    fn parse_suffix_opr(
        &mut self,
        opr: SuffixOpr,
        raw_opds: &RawExprRange,
    ) -> SemanticResult<EagerExprKind> {
        let opd_idx = raw_opds.start;
        let opd = self.parse_eager_expr(opd_idx)?;
        Ok(EagerExprKind::Opn {
            opn_kind: EagerOpnKind::Suffix { opr, this: opd.ty },
            opds: vec![opd],
        })
    }

    fn parse_call(&mut self, opd_idx_range: RawExprRange) -> SemanticResult<EagerExprKind> {
        let call = &self.arena()[opd_idx_range.start];
        let input_opd_idx_range = (opd_idx_range.start + 1)..opd_idx_range.end;
        match call.kind {
            RawExprKind::Scope {
                scope,
                kind: RawEntityKind::Routine,
                ..
            } => {
                let signature = try_infer!(self.db().call_decl(scope));
                let arguments: Vec<_> = input_opd_idx_range
                    .clone()
                    .enumerate()
                    .map(|(i, raw)| self.parse_eager_expr(raw))
                    .collect::<SemanticResult<_>>()?;
                let output = signature.output;
                Ok(EagerExprKind::Opn {
                    opn_kind: EagerOpnKind::RoutineCall(RangedEntityRoute {
                        route: scope,
                        range: call.range(),
                    }),
                    opds: arguments,
                })
            }
            RawExprKind::Scope {
                scope,
                kind: RawEntityKind::Type(_),
                ..
            } => {
                let signature = try_infer!(self.db().call_decl(scope));
                let arguments: Vec<_> = input_opd_idx_range
                    .enumerate()
                    .map(|(i, raw)| self.parse_eager_expr(raw))
                    .collect::<SemanticResult<_>>()?;
                Ok(EagerExprKind::Opn {
                    opn_kind: EagerOpnKind::TypeCall {
                        ranged_ty: RangedEntityRoute {
                            route: scope,
                            range: call.range(),
                        },
                        ty_decl: try_infer!(self.db().ty_decl(scope)),
                    },
                    opds: arguments,
                })
            }
            RawExprKind::Scope { .. } => todo!(),
            RawExprKind::Variable { .. } => todo!(),
            RawExprKind::Unrecognized(_) => todo!(),
            RawExprKind::PrimitiveLiteral(_) => todo!(),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn {
                opr,
                opds: ref memb_opds,
            } => match opr {
                Opr::Binary(_) => todo!(),
                Opr::Prefix(_) => todo!(),
                Opr::Suffix(suffix_opr) => match suffix_opr {
                    SuffixOpr::Incr => todo!(),
                    SuffixOpr::Decr => todo!(),
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::MembAccess(memb_ident) => {
                        let this = self.parse_eager_expr(memb_opds.start)?;
                        let inputs = input_opd_idx_range
                            .map(|idx| self.parse_eager_expr(idx))
                            .collect::<SemanticResult<Vec<_>>>()?;
                        let this_ty_decl = self.db().ty_decl(this.ty).unwrap();
                        let mut opds = vec![this];
                        opds.extend(inputs);
                        msg_once!("todo: memb call compiled");
                        Ok(EagerExprKind::Opn {
                            opn_kind: EagerOpnKind::MembRoutineCall {
                                memb_ident,
                                this_ty_decl,
                            },
                            opds,
                        })
                    }
                    SuffixOpr::WithType(_) => todo!(),
                },
                Opr::List(_) => todo!(),
            },
            RawExprKind::Lambda(_, _) => todo!(),
            RawExprKind::This { .. } => todo!(),
        }
    }
}
