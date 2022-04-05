use ast::RawExprRange;
use map_utils::insert_new;
use syntax_types::{PrefixOpr, SuffixOpr};
use vm::{BinaryOpr, PureBinaryOpr};
use word::CustomIdentifier;

use super::*;

impl<'a> TySheetBuilder<'a> {
    pub(super) fn infer_expr(
        &mut self,
        expr_idx: RawExprIdx,
        expectation: Option<ScopePtr>,
        arena: &RawExprArena,
    ) -> Option<ScopePtr> {
        let ty_result: InferResult<ScopePtr> = self.expr_ty_result(expr_idx, expectation, arena);
        let opt_ty = ty_result.as_ref().ok().map(|ty| *ty);
        insert_new!(self.ty_sheet.exprs, expr_idx, ty_result);
        opt_ty
    }

    pub(super) fn expr_ty_result(
        &mut self,
        expr_idx: RawExprIdx,
        expectation: Option<ScopePtr>,
        arena: &RawExprArena,
    ) -> InferResult<ScopePtr> {
        let ty = match arena[expr_idx].kind {
            RawExprKind::Variable { varname, init_row } => Ok(derived_not_none!(self
                .ty_sheet
                .variables
                .get(&(varname, init_row))
                .unwrap()
                .clone())?),
            RawExprKind::Unrecognized(ident) => {
                p!(ident);
                todo!()
            }
            RawExprKind::Scope { scope, kind } => self.scope_ty_result(scope, kind),
            RawExprKind::PrimitiveLiteral(value) => Ok(value.ty().into()),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, ref opds } => self.opn_opt_ty(opr, opds, expr_idx, arena),
            RawExprKind::Lambda(_, _) => todo!(),
            RawExprKind::This { ty } => derived_not_none!(ty),
        }?;
        if let Some(expected_ty) = expectation {
            if !self.db.is_implicit_convertible(ty, expected_ty) {
                todo!()
            }
        }
        Ok(ty)
    }

    fn scope_ty_result(
        &mut self,
        scope: ScopePtr,
        entity_kind: RawEntityKind,
    ) -> InferResult<ScopePtr> {
        Ok(match entity_kind {
            RawEntityKind::Module => todo!(),
            RawEntityKind::Literal => match scope {
                ScopePtr::Builtin(BuiltinIdentifier::True)
                | ScopePtr::Builtin(BuiltinIdentifier::False) => BuiltinIdentifier::Bool.into(),
                ScopePtr::Custom(scope) => match scope.kind {
                    ScopeKind::Builtin { ident } => todo!(),
                    ScopeKind::Package { main, ident } => todo!(),
                    ScopeKind::ChildScope { parent, ident } => parent,
                    ScopeKind::Contextual { main, ident } => todo!(),
                    ScopeKind::Generic { ident, .. } => todo!(),
                },
                _ => todo!(),
            },
            RawEntityKind::Type(_) => BuiltinIdentifier::Type.into(),
            RawEntityKind::Trait => todo!(),
            RawEntityKind::Routine => {
                msg_once!("todo: generics in fp");
                BuiltinIdentifier::Fp.into()
            }
            RawEntityKind::Feature => self.db.feature_signature(scope)?.ty,
            RawEntityKind::Pattern => todo!(),
        })
    }

    fn opn_opt_ty(
        &mut self,
        opr: Opr,
        opds: &RawExprRange,
        expr_idx: RawExprIdx,
        arena: &RawExprArena,
    ) -> InferResult<ScopePtr> {
        match opr {
            Opr::Binary(opr) => self.binary_opn_ty_result(opr, opds.start, opds.start + 1, arena),
            Opr::Prefix(opr) => self.prefix_opn_ty_result(opr, opds.start, arena),
            Opr::Suffix(opr) => self.suffix_opn_ty_result(opr, opds.start, arena),
            Opr::List(opr) => self.list_opn_ty_result(opr, opds, arena),
        }
    }

    fn binary_opn_ty_result(
        &mut self,
        opr: BinaryOpr,
        lopd: RawExprIdx,
        ropd: RawExprIdx,
        arena: &RawExprArena,
    ) -> InferResult<ScopePtr> {
        let lopd_ty = derived_not_none!(self.infer_expr(lopd, None, arena))?;
        let ropd_ty = derived_not_none!(self.infer_expr(ropd, None, arena))?;
        match opr {
            BinaryOpr::Pure(pure_binary_opr) => match lopd_ty {
                ScopePtr::Builtin(lopd_builtin_ty) => match ropd_ty {
                    ScopePtr::Builtin(ropd_builtin_ty) => self.builtin_pure_binary_opn_ty_result(
                        pure_binary_opr,
                        lopd_builtin_ty,
                        ropd_builtin_ty,
                    ),
                    ScopePtr::Custom(_) => todo!(),
                },
                ScopePtr::Custom(lopd_custom_ty) => todo!(),
            },
            BinaryOpr::Assign(_) => {
                if lopd_ty != ropd_ty {
                    todo!()
                }
                Ok(BuiltinIdentifier::Void.into())
            }
        }
    }

    fn builtin_pure_binary_opn_ty_result(
        &self,
        pure_binary_opr: PureBinaryOpr,
        lopd_builtin_ty: BuiltinIdentifier,
        ropd_builtin_ty: BuiltinIdentifier,
    ) -> InferResult<ScopePtr> {
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

    fn prefix_opn_ty_result(
        &mut self,
        opr: PrefixOpr,
        opd: RawExprIdx,
        arena: &RawExprArena,
    ) -> InferResult<ScopePtr> {
        todo!()
    }

    fn suffix_opn_ty_result(
        &mut self,
        opr: SuffixOpr,
        opd: RawExprIdx,
        arena: &RawExprArena,
    ) -> InferResult<ScopePtr> {
        let opd_ty = derived_not_none!(self.infer_expr(opd, None, arena))?;
        match opr {
            SuffixOpr::Incr => todo!(),
            SuffixOpr::Decr => todo!(),
            SuffixOpr::MayReturn => panic!("should handle this case in parse return statement"),
            SuffixOpr::MembAccess(ident) => {
                self.db.ty_signature(opd_ty)?.memb_access_ty_result(ident)
            }
            SuffixOpr::WithType(_) => todo!(),
        }
    }

    fn list_opn_ty_result(
        &mut self,
        opr: ListOpr,
        opds: &RawExprRange,
        arena: &RawExprArena,
    ) -> InferResult<ScopePtr> {
        match opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.list_call_ty_result(opds, arena),
            ListOpr::Index => todo!(),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
        }
    }

    fn list_call_ty_result(
        &mut self,
        all_opds: &RawExprRange,
        arena: &RawExprArena,
    ) -> InferResult<ScopePtr> {
        let call_expr = &arena[all_opds.start];
        match call_expr.kind {
            RawExprKind::Scope { scope, .. } => {
                let call_signature = self.db.call_signature(scope)?;
                for i in 0..call_signature.inputs.len() {
                    let input_expr_idx = all_opds.start + 1 + i;
                    self.infer_expr(input_expr_idx, Some(call_signature.inputs[i].ty), arena);
                }
                Ok(call_signature.output)
            }
            RawExprKind::Variable { .. } => todo!(),
            RawExprKind::Unrecognized(_) => todo!(),
            RawExprKind::PrimitiveLiteral(_) => todo!(),
            RawExprKind::Bracketed(_) => todo!(),
            RawExprKind::Opn { opr, ref opds } => match opr {
                Opr::Binary(_) => todo!(),
                Opr::Prefix(_) => todo!(),
                Opr::Suffix(suffix) => match suffix {
                    SuffixOpr::MembAccess(ident) => self.memb_call_ty_result(
                        opds.start,
                        ident,
                        (all_opds.start + 1)..all_opds.end,
                        arena,
                    ),
                    SuffixOpr::Incr => todo!(),
                    SuffixOpr::Decr => todo!(),
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::WithType(_) => todo!(),
                },
                Opr::List(_) => todo!(),
            },
            RawExprKind::Lambda(_, _) => todo!(),
            RawExprKind::This { .. } => todo!(),
        }
    }

    fn memb_call_ty_result(
        &mut self,
        this: RawExprIdx,
        memb_ident: CustomIdentifier,
        inputs: RawExprRange,
        arena: &RawExprArena,
    ) -> InferResult<ScopePtr> {
        let this_ty = derived_not_none!(self.infer_expr(this, None, arena))?;
        let this_ty_signature = derived_ok!(self.db.ty_signature(this_ty));
        let memb_call_signature = this_ty_signature.memb_call_signature(memb_ident)?;
        if inputs.end - inputs.start != memb_call_signature.inputs.len() {
            todo!()
        }
        for i in 0..memb_call_signature.inputs.len() {
            self.infer_expr(
                inputs.start + i,
                Some(memb_call_signature.inputs[i].ty),
                arena,
            );
        }
        Ok(memb_call_signature.output)
    }
}
