use ast::RawExprRange;
use map_utils::insert_new;
use syntax_types::{PrefixOpr, SuffixOpr};
use text::TextRange;
use vm::{BinaryOpr, PureBinaryOpr};
use word::{CustomIdentifier, RangedCustomIdentifier};

use super::*;

impl<'a> TySheetBuilder<'a> {
    pub(super) fn infer_expr(
        &mut self,
        expr_idx: RawExprIdx,
        expectation: Option<EntityRoutePtr>,
        arena: &RawExprArena,
    ) -> Option<EntityRoutePtr> {
        let ty_result: InferResult<EntityRoutePtr> =
            self.expr_ty_result(expr_idx, expectation, arena);
        let opt_ty = ty_result.as_ref().ok().map(|ty| *ty);
        insert_new!(self.ty_sheet.exprs, expr_idx, ty_result);
        opt_ty
    }

    pub(super) fn expr_ty_result(
        &mut self,
        expr_idx: RawExprIdx,
        expectation: Option<EntityRoutePtr>,
        arena: &RawExprArena,
    ) -> InferResult<EntityRoutePtr> {
        let ty = match arena[expr_idx].kind {
            RawExprVariant::Variable { varname, init_row } => Ok(derived_not_none!(self
                .ty_sheet
                .variables
                .get(&(varname, init_row))
                .unwrap()
                .clone())?),
            RawExprVariant::Unrecognized(ident) => {
                p!(ident);
                todo!()
            }
            RawExprVariant::Scope { scope, kind } => self.scope_ty_result(scope, kind),
            RawExprVariant::PrimitiveLiteral(value) => Ok(value.ty().into()),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn { opr, ref opds } => self.opn_opt_ty(opr, opds, expr_idx, arena),
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::This { ty } => derived_not_none!(ty),
        }?;
        if let Some(expected_ty) = expectation {
            if !self.db.is_implicit_convertible(ty, expected_ty) {
                err!(
                    format!("expect {:?} but get {:?} instead", expected_ty, ty),
                    arena[expr_idx].range
                )
            }
        }
        Ok(ty)
    }

    fn scope_ty_result(
        &mut self,
        scope: EntityRoutePtr,
        entity_kind: EntityKind,
    ) -> InferResult<EntityRoutePtr> {
        Ok(match entity_kind {
            EntityKind::Module => todo!(),
            EntityKind::Literal => match scope {
                EntityRoutePtr::Root(RootIdentifier::True)
                | EntityRoutePtr::Root(RootIdentifier::False) => RootIdentifier::Bool.into(),
                EntityRoutePtr::Custom(scope) => match scope.kind {
                    EntityRouteKind::Root { ident } => todo!(),
                    EntityRouteKind::Package { main, ident } => todo!(),
                    EntityRouteKind::ChildScope { parent, ident } => parent,
                    EntityRouteKind::Contextual { main, ident } => todo!(),
                    EntityRouteKind::Generic { ident, .. } => todo!(),
                    EntityRouteKind::ThisType => todo!(),
                },
                _ => todo!(),
            },
            EntityKind::Type(_) => RootIdentifier::Type.into(),
            EntityKind::Trait => todo!(),
            EntityKind::Routine => {
                msg_once!("todo: generics in fp");
                RootIdentifier::Fp.into()
            }
            EntityKind::Feature => self.db.feature_decl(scope)?.ty,
            EntityKind::Pattern => todo!(),
        })
    }

    fn opn_opt_ty(
        &mut self,
        opr: Opr,
        opds: &RawExprRange,
        expr_idx: RawExprIdx,
        arena: &RawExprArena,
    ) -> InferResult<EntityRoutePtr> {
        let range = arena[expr_idx].range;
        match opr {
            Opr::Binary(opr) => {
                self.binary_opn_ty_result(opr, opds.start, opds.start + 1, arena, range)
            }
            Opr::Prefix(opr) => self.prefix_opn_ty_result(opr, opds.start, arena),
            Opr::Suffix(opr) => self.suffix_opn_ty_result(opr, opds.start, arena, range),
            Opr::List(opr) => self.list_opn_ty_result(opr, opds, arena, range),
        }
    }

    fn binary_opn_ty_result(
        &mut self,
        opr: BinaryOpr,
        lopd: RawExprIdx,
        ropd: RawExprIdx,
        arena: &RawExprArena,
        range: TextRange,
    ) -> InferResult<EntityRoutePtr> {
        let lopd_ty = derived_not_none!(self.infer_expr(lopd, None, arena))?;
        let ropd_ty = derived_not_none!(self.infer_expr(ropd, None, arena))?;
        match opr {
            BinaryOpr::Pure(pure_binary_opr) => match lopd_ty {
                EntityRoutePtr::Root(lopd_builtin_ty) => match ropd_ty {
                    EntityRoutePtr::Root(ropd_builtin_ty) => self
                        .builtin_pure_binary_opn_ty_result(
                            pure_binary_opr,
                            lopd_builtin_ty,
                            ropd_builtin_ty,
                            range,
                        ),
                    EntityRoutePtr::Custom(_) => todo!(),
                    EntityRoutePtr::ThisType => todo!(),
                },
                EntityRoutePtr::Custom(lopd_custom_ty) => todo!(),
                EntityRoutePtr::ThisType => todo!(),
            },
            BinaryOpr::Assign(_) => {
                if lopd_ty != ropd_ty {
                    todo!()
                }
                Ok(RootIdentifier::Void.into())
            }
        }
    }

    fn builtin_pure_binary_opn_ty_result(
        &self,
        pure_binary_opr: PureBinaryOpr,
        lopd_builtin_ty: RootIdentifier,
        ropd_builtin_ty: RootIdentifier,
        range: TextRange,
    ) -> InferResult<EntityRoutePtr> {
        Ok(match pure_binary_opr {
            PureBinaryOpr::Less
            | PureBinaryOpr::Leq
            | PureBinaryOpr::Greater
            | PureBinaryOpr::Geq => {
                if lopd_builtin_ty != ropd_builtin_ty {
                    err!("expect use of \"<, <=, >, >=\" on same types", range)
                }
                match lopd_builtin_ty {
                    RootIdentifier::I32 | RootIdentifier::F32 => (),
                    _ => err!("expect use of \"<, <=, >, >=\" on i32 or f32", range),
                }
                RootIdentifier::Bool
            }
            PureBinaryOpr::Eq | PureBinaryOpr::Neq => {
                if lopd_builtin_ty != ropd_builtin_ty {
                    err!("expect use of \"!=\" on same types", range)
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
                    err!("expect use of \"+, -, *, /, **\" on same types", range)
                }
                match lopd_builtin_ty {
                    RootIdentifier::I32 | RootIdentifier::F32 => (),
                    _ => err!("expect use of \"+, -, *, /, **\" on i32 or f32", range),
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
    ) -> InferResult<EntityRoutePtr> {
        todo!()
    }

    fn suffix_opn_ty_result(
        &mut self,
        opr: SuffixOpr,
        opd: RawExprIdx,
        arena: &RawExprArena,
        range: TextRange,
    ) -> InferResult<EntityRoutePtr> {
        let opd_ty = derived_not_none!(self.infer_expr(opd, None, arena))?;
        match opr {
            SuffixOpr::Incr => todo!(),
            SuffixOpr::Decr => todo!(),
            SuffixOpr::MayReturn => panic!("should handle this case in parse return statement"),
            SuffixOpr::MembAccess(ident) => self.db.ty_decl(opd_ty)?.field_access_ty_result(ident),
            SuffixOpr::WithType(_) => todo!(),
        }
    }

    fn list_opn_ty_result(
        &mut self,
        opr: ListOpr,
        opds: &RawExprRange,
        arena: &RawExprArena,
        range: TextRange,
    ) -> InferResult<EntityRoutePtr> {
        match opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.list_call_ty_result(opds, arena, range),
            ListOpr::Index => todo!(),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
        }
    }

    fn list_call_ty_result(
        &mut self,
        all_opds: &RawExprRange,
        arena: &RawExprArena,
        range: TextRange,
    ) -> InferResult<EntityRoutePtr> {
        let call_expr = &arena[all_opds.start];
        match call_expr.kind {
            RawExprVariant::Scope { scope, .. } => {
                let call_decl = self.db.call_decl(scope)?;
                for i in 0..call_decl.inputs.len() {
                    let input_expr_idx = all_opds.start + 1 + i;
                    self.infer_expr(input_expr_idx, Some(call_decl.inputs[i].ty), arena);
                }
                Ok(call_decl.output)
            }
            RawExprVariant::Variable { .. } => todo!(),
            RawExprVariant::Unrecognized(_) => todo!(),
            RawExprVariant::PrimitiveLiteral(_) => todo!(),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn { opr, ref opds } => match opr {
                Opr::Binary(_) => todo!(),
                Opr::Prefix(_) => todo!(),
                Opr::Suffix(suffix) => match suffix {
                    SuffixOpr::MembAccess(ident) => self.field_call_ty_result(
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
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::This { .. } => todo!(),
        }
    }

    fn field_call_ty_result(
        &mut self,
        this: RawExprIdx,
        field_ident: RangedCustomIdentifier,
        inputs: RawExprRange,
        arena: &RawExprArena,
    ) -> InferResult<EntityRoutePtr> {
        let this_ty = derived_not_none!(self.infer_expr(this, None, arena))?;
        let this_ty_decl = derived_ok!(self.db.ty_decl(this_ty));
        let field_call_decl = this_ty_decl.method_decl(field_ident)?;
        if inputs.end - inputs.start != field_call_decl.inputs.len() {
            todo!()
        }
        for i in 0..field_call_decl.inputs.len() {
            self.infer_expr(inputs.start + i, Some(field_call_decl.inputs[i].ty), arena);
        }
        Ok(field_call_decl.output)
    }
}
