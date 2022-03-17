use ast::{RawExpr, RawExprArena, RawExprKind, RawExprRange};
use common::p;
use file::FilePtr;
use scope::{ScopeKind, ScopePtr};
use syntax_types::{ListOpr, Opr};
use vm::{BinaryOpr, Contract, PrimitiveValue, PureBinaryOpr};
use word::BuiltinIdentifier;

use crate::{error::err, *};

use super::OpnKind;

pub trait StrictExprParser<'a> {
    fn arena(&self) -> &'a RawExprArena;
    fn vartype(&self, varname: CustomIdentifier) -> ScopePtr;
    fn db(&self) -> &'a dyn InferQueryGroup;
    fn file(&self) -> FilePtr;

    fn parse_eager_expr(
        &mut self,
        raw_expr: &RawExpr,
        contract: Contract,
    ) -> SemanticResult<Arc<Expr>> {
        let (ty, kind): (ScopePtr, _) = match raw_expr.kind {
            RawExprKind::Variable(ident) => (self.vartype(ident), ExprKind::Variable(ident)),
            RawExprKind::Unrecognized(ident) => {
                err!(format!(
                    "unrecognized identifier {} at {:?}",
                    ident,
                    raw_expr.range()
                ))
            }
            RawExprKind::Scope { scope, kind, .. } => match kind {
                ScopeKind::Module => todo!(),
                ScopeKind::Literal => {
                    match scope {
                        ScopePtr::Builtin(BuiltinIdentifier::True) => (
                            ScopePtr::Builtin(BuiltinIdentifier::Bool),
                            ExprKind::Literal(PrimitiveValue::Bool(true)),
                        ),
                        ScopePtr::Builtin(BuiltinIdentifier::False) => (
                            ScopePtr::Builtin(BuiltinIdentifier::Bool),
                            ExprKind::Literal(PrimitiveValue::Bool(false)),
                        ),
                        ScopePtr::Custom(_) => todo!(),
                        _ => todo!(),
                    }
                    // (
                    //     self.db().scope_ty(scope)?,
                    //     ExprKind::Scope {
                    //         scope,
                    //         compiled: None,
                    //     },
                    // )
                }
                ScopeKind::Type => todo!(),
                ScopeKind::Trait => todo!(),
                ScopeKind::Routine => todo!(),
                ScopeKind::Feature => (
                    self.db().scope_ty(scope)?,
                    ExprKind::Scope {
                        scope,
                        compiled: None,
                    },
                ),
            },
            RawExprKind::Literal(value) => (value.ty().into(), ExprKind::Literal(value)),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, ref opds } => self.parse_opn(opr, opds)?,
            RawExprKind::Lambda(_, _) => todo!(),
        };
        Ok(Arc::new(Expr {
            range: raw_expr.range().clone(),
            ty,
            kind,
            file: self.file(),
            instruction_id: Default::default(),
            contract,
        }))
    }

    fn parse_opn(&mut self, opr: Opr, opds: &RawExprRange) -> SemanticResult<(ScopePtr, ExprKind)> {
        match opr {
            Opr::Binary(opr) => self.parse_binary_opr(opr, opds),
            Opr::Prefix(_) => todo!(),
            Opr::Suffix(_) => todo!(),
            Opr::List(opr) => match opr {
                ListOpr::TupleInit => todo!(),
                ListOpr::NewVec => todo!(),
                ListOpr::NewDict => todo!(),
                ListOpr::Call => self.parse_call(&self.arena()[opds]),
                ListOpr::Index => todo!(),
                ListOpr::ModuloIndex => todo!(),
                ListOpr::StructInit => todo!(),
            },
        }
    }

    fn parse_binary_opr(
        &mut self,
        opr: BinaryOpr,
        raw_opds: &RawExprRange,
    ) -> SemanticResult<(ScopePtr, ExprKind)> {
        let raw_opds = &self.arena()[raw_opds];
        let lopd = self.parse_eager_expr(
            &raw_opds[0],
            match opr {
                BinaryOpr::Pure(_) => Contract::PureInput,
                BinaryOpr::Assign(_) => Contract::BorrowMut,
            },
        )?;
        let ropd = self.parse_eager_expr(&raw_opds[1], Contract::PureInput)?;
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
                BuiltinIdentifier::Void.into()
            }
        };
        Ok((
            output_type,
            ExprKind::Opn {
                opn_kind: OpnKind::Binary { opr, this: lopd.ty },
                opds: vec![lopd, ropd],
                compiled: None,
            },
        ))
    }

    fn infer_pure_binary_opr_type(
        &self,
        pure_binary_opr: PureBinaryOpr,
        lopd_ty: ScopePtr,
        ropd_ty: ScopePtr,
    ) -> SemanticResult<ScopePtr> {
        match lopd_ty {
            ScopePtr::Builtin(lopd_builtin_ty) => match ropd_ty {
                ScopePtr::Builtin(ropd_builtin_ty) => self.infer_builtin_pure_binary_opr_type(
                    pure_binary_opr,
                    lopd_builtin_ty,
                    ropd_builtin_ty,
                ),
                ScopePtr::Custom(_) => todo!(),
            },
            ScopePtr::Custom(lopd_custom_ty) => todo!(),
        }
    }

    fn infer_builtin_pure_binary_opr_type(
        &self,
        pure_binary_opr: PureBinaryOpr,
        lopd_builtin_ty: BuiltinIdentifier,
        ropd_builtin_ty: BuiltinIdentifier,
    ) -> SemanticResult<ScopePtr> {
        Ok(match pure_binary_opr {
            PureBinaryOpr::Less
            | PureBinaryOpr::Leq
            | PureBinaryOpr::Greater
            | PureBinaryOpr::Geq => {
                if lopd_builtin_ty != ropd_builtin_ty {
                    err!("expect use of \"<, <=, >, >=\" on same types")
                }
                match lopd_builtin_ty {
                    BuiltinIdentifier::I32 | BuiltinIdentifier::F32 => (),
                    _ => err!("expect use of \"<, <=, >, >=\" on i32 or f32"),
                }
                BuiltinIdentifier::Bool
            }
            PureBinaryOpr::Eq | PureBinaryOpr::Neq => {
                if lopd_builtin_ty != ropd_builtin_ty {
                    err!("expect use of \"!=\" on same types")
                }
                BuiltinIdentifier::Bool
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
                    BuiltinIdentifier::I32 | BuiltinIdentifier::F32 => (),
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

    fn parse_call(&mut self, opds: &[RawExpr]) -> SemanticResult<(ScopePtr, ExprKind)> {
        let call = &opds[0];
        match call.kind {
            RawExprKind::Scope {
                scope,
                kind: ScopeKind::Routine,
                ..
            } => {
                let signature = self.db().call_signature(scope)?;
                let arguments: Vec<_> = opds[1..]
                    .iter()
                    .enumerate()
                    .map(|(i, raw)| self.parse_eager_expr(raw, signature.inputs[i].contract))
                    .collect::<SemanticResult<_>>()?;
                let output = signature.output;
                Ok((
                    output,
                    ExprKind::Opn {
                        opn_kind: OpnKind::RoutineCall(RangedScope {
                            scope,
                            range: call.range(),
                        }),
                        compiled: signature.compiled,
                        opds: arguments,
                    },
                ))
            }
            RawExprKind::Scope {
                scope,
                kind: ScopeKind::Type,
                ..
            } => {
                p!(scope);
                todo!()
            }
            RawExprKind::Scope { .. } => todo!(),
            RawExprKind::Variable(_) => todo!(),
            RawExprKind::Unrecognized(_) => todo!(),
            RawExprKind::Literal(_) => todo!(),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, ref opds } => todo!(),
            RawExprKind::Lambda(_, _) => todo!(),
        }
    }
}
