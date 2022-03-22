use std::sync::Arc;

use crate::*;
use ast::{RawExpr, RawExprArena, RawExprIdx, RawExprKind, RawExprRange};
use common::*;
use file::FilePtr;
use scope::{RangedScope, ScopeKind, ScopePtr, ScopeRoute};
use syntax_types::{ListOpr, Opr};
use vm::{BinaryOpr, EagerContract, PrimitiveValue, PureBinaryOpr};
use word::{BuiltinIdentifier, CustomIdentifier};

use super::*;
use semantics_error::*;

pub trait LazyExprParser<'a> {
    fn arena(&self) -> &'a RawExprArena;
    fn vartype(&self, varname: CustomIdentifier) -> ScopePtr;
    fn db(&self) -> &'a dyn InferQueryGroup;
    fn file(&self) -> FilePtr;

    fn parse_lazy_expr(&mut self, raw_expr_idx: RawExprIdx) -> SemanticResult<Arc<LazyExpr>> {
        let raw_expr = &self.arena()[raw_expr_idx];
        let kind: LazyExprKind = match raw_expr.kind {
            RawExprKind::Variable { varname, .. } => LazyExprKind::Variable(varname),
            RawExprKind::Unrecognized(ident) => {
                err!(format!(
                    "unrecognized identifier {} at {:?}",
                    ident,
                    raw_expr.range()
                ))
            }
            RawExprKind::Scope { scope, kind, .. } => match kind {
                ScopeKind::Module => todo!(),
                ScopeKind::Literal => match scope {
                    ScopePtr::Builtin(BuiltinIdentifier::True) => {
                        LazyExprKind::PrimitiveLiteral(PrimitiveValue::Bool(true))
                    }
                    ScopePtr::Builtin(BuiltinIdentifier::False) => {
                        LazyExprKind::PrimitiveLiteral(PrimitiveValue::Bool(false))
                    }
                    ScopePtr::Custom(scope_ref) => LazyExprKind::EnumLiteral {
                        scope,
                        value: self.db().enum_literal_value(scope),
                    },
                    _ => todo!(),
                },
                ScopeKind::Type => todo!(),
                ScopeKind::Trait => todo!(),
                ScopeKind::Routine => todo!(),
                ScopeKind::Feature => {
                    (
                        todo!()
                        // try_syntax!(self.db().scope_ty(scope)?,
                        // LazyExprKind::Scope {
                        //     scope,
                        //     compiled: None,
                        // },
                    )
                }
                ScopeKind::Pattern => todo!(),
            },
            RawExprKind::PrimitiveLiteral(value) => LazyExprKind::PrimitiveLiteral(value),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, ref opds } => self.parse_opn(opr, opds)?,
            RawExprKind::Lambda(_, _) => todo!(),
            RawExprKind::This { .. } => todo!(),
        };
        Ok(Arc::new(LazyExpr {
            range: raw_expr.range().clone(),
            ty: self.db().expr_ty_result(self.file(), raw_expr_idx).unwrap(),
            kind,
            file: self.file(),
            instruction_id: Default::default(),
            contract: self
                .db()
                .lazy_expr_contract_result(self.file(), raw_expr_idx)
                .unwrap(),
        }))
    }

    fn parse_opn(&mut self, opr: Opr, opds: &RawExprRange) -> SemanticResult<LazyExprKind> {
        match opr {
            Opr::Binary(opr) => self.parse_binary_opr(opr, opds),
            Opr::Prefix(_) => todo!(),
            Opr::Suffix(opr) => self.parse_suffix_opr(opr, opds),
            Opr::List(opr) => match opr {
                ListOpr::TupleInit => todo!(),
                ListOpr::NewVec => todo!(),
                ListOpr::NewDict => todo!(),
                ListOpr::Call => self.parse_call(opds),
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
                BuiltinIdentifier::Void.into()
            }
        };
        let opr = match opr {
            BinaryOpr::Pure(opr) => opr,
            BinaryOpr::Assign(_) => todo!(),
        };
        Ok(LazyExprKind::Opn {
            opn_kind: LazyOpnKind::Binary { opr, this: lopd.ty },
            opds: vec![lopd, ropd],
            compiled: (),
        })
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

    fn parse_suffix_opr(
        &mut self,
        opr: SuffixOpr,
        opds: &RawExprRange,
    ) -> SemanticResult<LazyExprKind> {
        let opd = self.parse_lazy_expr(opds.start)?;
        Ok(match opr {
            SuffixOpr::Incr => todo!(),
            SuffixOpr::Decr => todo!(),
            SuffixOpr::MayReturn => panic!("should handle this case in parse return statement"),
            SuffixOpr::MembVarAccess(ident) => LazyExprKind::Opn {
                opn_kind: LazyOpnKind::MembVarAccess(ident),
                compiled: (),
                opds: vec![opd],
            },
            SuffixOpr::WithType(_) => todo!(),
        })
    }

    fn parse_call(&mut self, opds: &RawExprRange) -> SemanticResult<LazyExprKind> {
        let call = &self.arena()[opds.start];
        match call.kind {
            RawExprKind::Scope {
                scope,
                kind: ScopeKind::Routine,
                ..
            } => {
                let arguments: Vec<_> = ((opds.start + 1)..opds.end)
                    .map(|raw| self.parse_lazy_expr(raw))
                    .collect::<SemanticResult<_>>()?;
                Ok(LazyExprKind::Opn {
                    opn_kind: LazyOpnKind::RoutineCall(RangedScope {
                        scope,
                        range: call.range(),
                    }),
                    compiled: (),
                    opds: arguments,
                })
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
            RawExprKind::Variable { .. } => todo!(),
            RawExprKind::Unrecognized(_) => todo!(),
            RawExprKind::PrimitiveLiteral(_) => todo!(),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, ref opds } => todo!(),
            RawExprKind::Lambda(_, _) => todo!(),
            RawExprKind::This { .. } => todo!(),
        }
    }
}
