use ast::{RawExpr, RawExprArena, RawExprKind, RawExprRange};
use common::p;
use file::FilePtr;
use scope::{ScopeKind, ScopePtr};
use syntax_types::{ListOpr, Opr};
use vm::{BinaryOpr, Contract, PrimitiveValue, PureBinaryOpr};
use word::BuiltinIdentifier;

use crate::{error::err, *};

use super::{BinaryOpnKind, Opn};

pub trait ExprParser<'a> {
    fn arena(&self) -> &'a RawExprArena;
    fn vartype(&self, varname: CustomIdentifier) -> ScopePtr;
    fn db(&self) -> &'a dyn InferQueryGroup;
    fn file(&self) -> FilePtr;

    fn parse_expr(&mut self, raw_expr: &RawExpr, contract: Contract) -> SemanticResult<Arc<Expr>> {
        let (ty, kind): (ScopePtr, _) = match raw_expr.kind {
            RawExprKind::Variable(ident) => (self.vartype(ident), StrictExprKind::Variable(ident)),
            RawExprKind::Unrecognized(_) => todo!(),
            RawExprKind::Scope { scope, kind, .. } => match kind {
                ScopeKind::Module => todo!(),
                ScopeKind::Literal => {
                    match scope {
                        ScopePtr::Builtin(BuiltinIdentifier::True) => (
                            ScopePtr::Builtin(BuiltinIdentifier::Bool),
                            StrictExprKind::Literal(PrimitiveValue::Bool(true)),
                        ),
                        ScopePtr::Builtin(BuiltinIdentifier::False) => (
                            ScopePtr::Builtin(BuiltinIdentifier::Bool),
                            StrictExprKind::Literal(PrimitiveValue::Bool(false)),
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
                ScopeKind::Func => todo!(),
                ScopeKind::Feature => (
                    self.db().scope_ty(scope)?,
                    StrictExprKind::Scope {
                        scope,
                        compiled: None,
                    },
                ),
            },
            RawExprKind::Literal(value) => (value.ty().into(), StrictExprKind::Literal(value)),
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

    fn parse_opn(
        &mut self,
        opr: Opr,
        opds: &RawExprRange,
    ) -> SemanticResult<(ScopePtr, StrictExprKind)> {
        match opr {
            Opr::Binary(opr) => self.parse_binary_opr(opr, opds),
            Opr::Prefix(_) => todo!(),
            Opr::Suffix(_) => todo!(),
            Opr::List(opr) => match opr {
                ListOpr::TupleInit => todo!(),
                ListOpr::NewVec => todo!(),
                ListOpr::NewDict => todo!(),
                ListOpr::Call => {
                    let call = &self.arena()[opds][0];
                    match call.kind {
                        RawExprKind::Scope {
                            scope,
                            kind: ScopeKind::Func,
                            ..
                        } => {
                            let signature = self.db().func_signature(scope)?;
                            let arguments: Vec<_> = self.arena()[opds][1..]
                                .iter()
                                .enumerate()
                                .map(|(i, raw)| self.parse_expr(raw, signature.inputs[i].contract))
                                .collect::<SemanticResult<_>>()?;
                            let output = signature.output;
                            Ok((
                                output,
                                StrictExprKind::Opn {
                                    opn: Opn::RoutineCall(RangedScope {
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
                        } => todo!(),
                        RawExprKind::Scope { .. } => todo!(),
                        RawExprKind::Variable(_) => todo!(),
                        RawExprKind::Unrecognized(_) => todo!(),
                        RawExprKind::Literal(_) => todo!(),
                        RawExprKind::Bracketed(_) => todo!(),
                        RawExprKind::Opn { opr, ref opds } => todo!(),
                        RawExprKind::Lambda(_, _) => todo!(),
                    }
                }
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
    ) -> SemanticResult<(ScopePtr, StrictExprKind)> {
        Ok(match opr {
            BinaryOpr::Pure(pure_binary) => {
                let opds = self.arena()[raw_opds]
                    .iter()
                    .map(|raw| self.parse_expr(raw, Contract::Pure))
                    .collect::<SemanticResult<Vec<_>>>()?;
                match pure_binary {
                    PureBinaryOpr::Less => todo!(),
                    PureBinaryOpr::Leq => todo!(),
                    PureBinaryOpr::Greater => todo!(),
                    PureBinaryOpr::Geq => todo!(),
                    PureBinaryOpr::Neq => todo!(),
                    PureBinaryOpr::Eq => {
                        if opds[0].ty != opds[1].ty {
                            err!("expect use of \"==\" on same types")
                        }
                        let opn = match opds[0].ty {
                            ScopePtr::Builtin(ident) => {
                                let kind = match ident {
                                    BuiltinIdentifier::Void => todo!(),
                                    BuiltinIdentifier::I32 => BinaryOpnKind::EqI32,
                                    BuiltinIdentifier::F32 => BinaryOpnKind::EqF32,
                                    BuiltinIdentifier::Bool => BinaryOpnKind::EqBool,
                                    _ => panic!(),
                                };
                                Opn::Binary {
                                    opr,
                                    this: opds[0].ty,
                                    kind,
                                }
                            }
                            ScopePtr::Custom(_) => todo!(),
                        };
                        (
                            BuiltinIdentifier::Bool.into(),
                            StrictExprKind::Opn {
                                opds,
                                compiled: None,
                                opn,
                            },
                        )
                    }
                    PureBinaryOpr::Shl => todo!(),
                    PureBinaryOpr::Shr => todo!(),
                    PureBinaryOpr::Add => {
                        if opds[0].ty != opds[1].ty {
                            err!("expect use of \"+\" on same types")
                        }
                        let opn = match opds[0].ty {
                            ScopePtr::Builtin(ident) => {
                                let kind = match ident {
                                    BuiltinIdentifier::I32 => BinaryOpnKind::AddI32,
                                    BuiltinIdentifier::F32 => BinaryOpnKind::AddF32,
                                    _ => panic!(),
                                };
                                Opn::Binary {
                                    opr,
                                    this: opds[0].ty,
                                    kind,
                                }
                            }
                            ScopePtr::Custom(_) => todo!(),
                        };
                        (
                            opds[0].ty,
                            StrictExprKind::Opn {
                                opds,
                                compiled: None,
                                opn,
                            },
                        )
                    }
                    PureBinaryOpr::Sub => todo!(),
                    PureBinaryOpr::Mul => todo!(),
                    PureBinaryOpr::Div => todo!(),
                    PureBinaryOpr::Power => todo!(),
                    PureBinaryOpr::And => todo!(),
                    PureBinaryOpr::BitAnd => todo!(),
                    PureBinaryOpr::Or => todo!(),
                    PureBinaryOpr::BitXor => todo!(),
                    PureBinaryOpr::BitOr => todo!(),
                    PureBinaryOpr::RemEuclid => todo!(),
                }
            }
            BinaryOpr::Assign(opt_binary) => {
                let opds = vec![
                    self.parse_expr(&self.arena()[raw_opds][0], Contract::BorrowMut)?,
                    self.parse_expr(&self.arena()[raw_opds][1], Contract::Pure)?,
                ];
                let opn = match opt_binary {
                    Some(binary) => match binary {
                        PureBinaryOpr::Add => {
                            if opds[0].ty != opds[1].ty {
                                err!("expect use of \"+=\" on same types")
                            }
                            match opds[0].ty {
                                ScopePtr::Builtin(ident) => {
                                    let kind = match ident {
                                        BuiltinIdentifier::Void => todo!(),
                                        BuiltinIdentifier::I32 => BinaryOpnKind::EqI32,
                                        BuiltinIdentifier::F32 => BinaryOpnKind::EqF32,
                                        BuiltinIdentifier::Bool => BinaryOpnKind::EqBool,
                                        _ => panic!(),
                                    };
                                    Opn::Binary {
                                        opr,
                                        this: opds[0].ty,
                                        kind,
                                    }
                                }
                                ScopePtr::Custom(_) => todo!(),
                            }
                        }
                        PureBinaryOpr::And => todo!(),
                        PureBinaryOpr::BitAnd => todo!(),
                        PureBinaryOpr::BitOr => todo!(),
                        PureBinaryOpr::BitXor => todo!(),
                        PureBinaryOpr::Div => todo!(),
                        PureBinaryOpr::Eq => todo!(),
                        PureBinaryOpr::Geq => todo!(),
                        PureBinaryOpr::Greater => todo!(),
                        PureBinaryOpr::Leq => todo!(),
                        PureBinaryOpr::Less => todo!(),
                        PureBinaryOpr::Mul => todo!(),
                        PureBinaryOpr::Neq => todo!(),
                        PureBinaryOpr::RemEuclid => todo!(),
                        PureBinaryOpr::Or => todo!(),
                        PureBinaryOpr::Power => todo!(),
                        PureBinaryOpr::Shl => todo!(),
                        PureBinaryOpr::Shr => todo!(),
                        PureBinaryOpr::Sub => todo!(),
                    },
                    None => {
                        if opds[0].ty != opds[1].ty {
                            err!("expect use of \"=\" on same types")
                        }
                        todo!()
                    }
                };
                (
                    BuiltinIdentifier::Void.into(),
                    StrictExprKind::Opn {
                        opds,
                        compiled: None,
                        opn,
                    },
                )
            }
        })
    }
}
