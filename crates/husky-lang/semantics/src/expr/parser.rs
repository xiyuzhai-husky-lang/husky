use ast::{RawExpr, RawExprArena, RawExprKind, RawExprRange};
use scope::{ScopeKind, ScopePtr};
use syntax_types::Opr;
use vm::BinaryOpr;
use word::BuiltinIdentifier;

use crate::{error::err, *};

use super::{BinaryOpnKind, Opn};

pub trait ExprParser<'a> {
    fn arena(&self) -> &'a RawExprArena;
    fn vartype(&self, varname: CustomIdentifier) -> ScopePtr;
    fn db(&self) -> &'a dyn InferQueryGroup;

    fn parse_expr(&mut self, raw_expr: &RawExpr) -> SemanticResult<Expr> {
        let (ty, kind): (ScopePtr, _) = match raw_expr.kind {
            RawExprKind::Variable(ident) => (self.vartype(ident), ExprKind::Variable(ident)),
            RawExprKind::Scope(id, kind) => match kind {
                ScopeKind::Module => todo!(),
                ScopeKind::Value => todo!(),
                ScopeKind::Type => todo!(),
                ScopeKind::Trait => todo!(),
                ScopeKind::Func => todo!(),
                ScopeKind::Feature => (
                    self.db().scope_ty(id)?,
                    ExprKind::Scope {
                        scope: id,
                        compiled: None,
                    },
                ),
            },
            RawExprKind::Literal(value) => (value.ty().into(), ExprKind::Literal(value)),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, ref opds } => self.parse_opn(opr, opds)?,
            RawExprKind::Lambda(_, _) => todo!(),
        };
        Ok(Expr {
            range: raw_expr.range.clone(),
            ty,
            kind,
        })
    }

    fn parse_opn(&mut self, opr: Opr, opds: &RawExprRange) -> SemanticResult<(ScopePtr, ExprKind)> {
        match opr {
            Opr::Binary(opr) => self.parse_binary_opr(opr, opds),
            Opr::Prefix(_) => todo!(),
            Opr::Suffix(_) => todo!(),
            Opr::List(opr) => match opr {
                syntax_types::ListOpr::TupleInit => todo!(),
                syntax_types::ListOpr::NewVec => todo!(),
                syntax_types::ListOpr::NewDict => todo!(),
                syntax_types::ListOpr::Call => {
                    let call = &self.arena()[opds][0];
                    match call.kind {
                        RawExprKind::Scope(scope, ScopeKind::Func) => {
                            let signature = self.db().func_signature(scope)?;
                            let arguments: Vec<Expr> = self.arena()[opds][1..]
                                .iter()
                                .map(|raw| self.parse_expr(raw))
                                .collect::<SemanticResult<_>>()?;
                            let output = signature.output;
                            Ok((
                                output,
                                ExprKind::Opn {
                                    opn: Opn::FuncCall { func: scope },
                                    compiled: signature.compiled,
                                    opds: arguments,
                                },
                            ))
                        }
                        RawExprKind::Scope(scope, ScopeKind::Type) => todo!(),
                        RawExprKind::Scope(_, _) => todo!(),
                        RawExprKind::Variable(_) => todo!(),
                        RawExprKind::Literal(_) => todo!(),
                        RawExprKind::Bracketed(_) => todo!(),
                        RawExprKind::Opn { opr, ref opds } => todo!(),
                        RawExprKind::Lambda(_, _) => todo!(),
                    }
                }
                syntax_types::ListOpr::Index => todo!(),
                syntax_types::ListOpr::ModuloIndex => todo!(),
                syntax_types::ListOpr::StructInit => todo!(),
            },
        }
    }

    fn parse_binary_opr(
        &mut self,
        opr: BinaryOpr,
        opds: &RawExprRange,
    ) -> SemanticResult<(ScopePtr, ExprKind)> {
        let opds = self.arena()[opds]
            .iter()
            .map(|raw| self.parse_expr(raw))
            .collect::<SemanticResult<Vec<_>>>()?;
        Ok(match opr {
            BinaryOpr::Less => todo!(),
            BinaryOpr::Leq => todo!(),
            BinaryOpr::Greater => todo!(),
            BinaryOpr::Geq => todo!(),
            BinaryOpr::Neq => todo!(),
            BinaryOpr::Eq => {
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
                            opr: BinaryOpr::Eq,
                            this: opds[0].ty,
                            kind,
                        }
                    }
                    ScopePtr::Custom(_) => todo!(),
                };
                (
                    BuiltinIdentifier::Bool.into(),
                    ExprKind::Opn {
                        opds,
                        compiled: None,
                        opn,
                    },
                )
            }
            BinaryOpr::Shl => todo!(),
            BinaryOpr::Shr => todo!(),
            BinaryOpr::Add => todo!(),
            BinaryOpr::Sub => todo!(),
            BinaryOpr::Mul => todo!(),
            BinaryOpr::Div => todo!(),
            BinaryOpr::Power => todo!(),
            BinaryOpr::And => todo!(),
            BinaryOpr::BitAnd => todo!(),
            BinaryOpr::Or => todo!(),
            BinaryOpr::BitXor => todo!(),
            BinaryOpr::BitOr => todo!(),
            BinaryOpr::RemEuclid => todo!(),
        })
    }
}
