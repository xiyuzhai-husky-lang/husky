use ast::{RawExprArena, RawExprRange, RawExprVariant, RawLoopKind, RawStmt, RawStmtVariant};
use check_utils::should;
use defn_head::InputPlaceholder;
use entity_route::EntityRoutePtr;
use infer_error::derived_not_none;
use print_utils::{msg_once, p};
use text::TextRanged;
use word::RangedCustomIdentifier;

use super::*;

impl<'a> QualifiedTySheetBuilder<'a> {
    pub(super) fn build_routine(
        &mut self,
        inputs: &[InputPlaceholder],
        ast_iter: AstIter,
        arena: &RawExprArena,
        opt_output_ty: Option<EntityRoutePtr>,
        output_contract: OutputContract,
    ) {
        self.add_inputs(inputs);
        self.build_eager_stmts(arena, ast_iter, opt_output_ty, output_contract)
    }

    pub(super) fn build_eager_stmts(
        &mut self,
        arena: &RawExprArena,
        ast_iter: AstIter,
        opt_output_ty: Option<EntityRoutePtr>,
        output_contract: OutputContract,
    ) {
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.kind {
                    AstKind::Stmt(ref stmt) => {
                        self.build_eager_stmt(arena, stmt, opt_output_ty, output_contract)
                    }
                    _ => (),
                }
            }
            if let Some(children) = item.children {
                self.build_eager_stmts(arena, children, opt_output_ty, output_contract)
            }
        }
    }

    fn build_eager_stmt(
        &mut self,
        arena: &RawExprArena,
        stmt: &RawStmt,
        opt_output_ty: Option<EntityRoutePtr>,
        output_contract: OutputContract,
    ) {
        match stmt.variant {
            RawStmtVariant::Loop(loop_kind) => match loop_kind {
                RawLoopKind::For {
                    frame_var,
                    initial_boundary,
                    final_boundary,
                    step,
                } => todo!(),
                RawLoopKind::ForExt {
                    frame_var,
                    final_boundary,
                    step,
                } => {
                    if let Some(bound) = final_boundary.opt_bound {
                        self.build_eager_expr(arena, bound);
                    }
                }
                RawLoopKind::While { condition } => {
                    self.build_eager_expr(arena, condition);
                }
                RawLoopKind::DoWhile { condition } => {
                    self.build_eager_expr(arena, condition);
                }
            },
            RawStmtVariant::Branch(_) => todo!(),
            RawStmtVariant::Exec(expr) => {
                self.build_eager_expr(arena, expr);
            }
            RawStmtVariant::Init {
                init_kind,
                varname,
                initial_value,
            } => {
                if let Some(initial_value_qualified_ty) =
                    self.build_eager_expr(arena, initial_value)
                {
                    should!(self
                        .qualified_ty_sheet
                        .variable_qualified_tys
                        .insert(
                            (varname.ident, stmt.row()),
                            initial_value_qualified_ty.use_for_init(init_kind)
                        )
                        .is_none())
                }
            }
            RawStmtVariant::Return(expr) => {
                match (opt_output_ty, self.build_eager_expr(arena, expr)) {
                    (Some(output_ty), Some(qualified_ty)) => {
                        if !self
                            .db
                            .is_qualified_ty_implicitly_convertible_to_contracted_ty(
                                qualified_ty,
                                output_contract,
                                output_ty,
                            )
                        {
                            todo!()
                        }
                    }
                    _ => (),
                }
            }
            RawStmtVariant::Assert(_) => todo!(),
            RawStmtVariant::Break => todo!(),
        }
    }

    fn build_eager_expr(&mut self, arena: &RawExprArena, expr: RawExprIdx) -> Option<QualifiedTy> {
        let qualified_qualified_ty_result =
            self.infer_eager_expr_qualified_qualified_ty_result(expr, arena);
        let opt_qualified_ty = qualified_qualified_ty_result
            .as_ref()
            .map(|qualified_ty| *qualified_ty)
            .ok();
        self.qualified_ty_sheet
            .qualified_tys
            .insert(expr, qualified_qualified_ty_result);
        opt_qualified_ty
    }

    fn infer_eager_expr_qualified_qualified_ty_result(
        &mut self,
        expr_idx: RawExprIdx,
        arena: &RawExprArena,
    ) -> InferResult<QualifiedTy> {
        let expr = &arena[expr_idx];
        match expr.variant {
            RawExprVariant::Variable { varname, init_row } => match derived_not_none!(self
                .qualified_ty_sheet
                .variable_qualified_tys
                .get(&(varname, init_row)))?
            {
                Ok(qt) => Ok(*qt),
                Err(e) => Err(e.derived()),
            },
            RawExprVariant::FrameVariable { varname, init_row } => todo!(),
            RawExprVariant::This { ty } => todo!(),
            RawExprVariant::Unrecognized(_) => todo!(),
            RawExprVariant::Entity { route, kind } => todo!(),
            RawExprVariant::PrimitiveLiteral(_) => Ok(QualifiedTy {
                qual: Qualifier::Copyable,
                ty: self.expr_ty_result(expr_idx).unwrap(),
            }),
            RawExprVariant::Bracketed(expr) => {
                derived_not_none!(self.build_eager_expr(arena, expr))
            }
            RawExprVariant::Opn { opr, ref opds } => self
                .infer_eager_opn_qualified_qualified_ty_result(arena, expr_idx, opr, opds.clone()),
            RawExprVariant::Lambda(_, _) => todo!(),
        }
    }

    fn infer_eager_opn_qualified_qualified_ty_result(
        &mut self,
        arena: &RawExprArena,
        expr_idx: RawExprIdx,
        opr: Opr,
        opds: RawExprRange,
    ) -> InferResult<QualifiedTy> {
        match opr {
            Opr::Binary(binary_opr) => self.infer_binary_qualified_ty_result(arena, expr_idx, opds),
            Opr::Prefix(_) => todo!(),
            Opr::Suffix(_) => todo!(),
            Opr::List(list_opr) => {
                self.infer_list_qualified_ty_result(arena, expr_idx, list_opr, opds)
            }
        }
    }

    fn infer_binary_qualified_ty_result(
        &mut self,
        arena: &RawExprArena,
        expr_idx: RawExprIdx,
        opds: RawExprRange,
    ) -> InferResult<QualifiedTy> {
        self.build_eager_expr(arena, opds.start);
        self.build_eager_expr(arena, opds.start + 1);
        Ok(QualifiedTy {
            qual: Qualifier::Transitive,
            ty: self.expr_ty_result(expr_idx)?,
        })
    }

    fn infer_list_qualified_ty_result(
        &mut self,
        arena: &RawExprArena,
        expr_idx: RawExprIdx,
        list_opr: ListOpr,
        opds: RawExprRange,
    ) -> InferResult<QualifiedTy> {
        match list_opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.infer_call_qualified_ty_result(arena, expr_idx, opds),
            ListOpr::Index => todo!(),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
        }
    }

    fn infer_call_qualified_ty_result(
        &mut self,
        arena: &RawExprArena,
        expr_idx: RawExprIdx,
        total_opds: RawExprRange,
    ) -> InferResult<QualifiedTy> {
        match arena[total_opds.start].variant {
            RawExprVariant::Entity { route, .. } => {
                let call_decl = self.db.call_decl(route)?;
                let opt_opd_qualified_tys: Vec<_> = ((total_opds.start + 1)..total_opds.end)
                    .into_iter()
                    .map(|opd_idx| self.build_eager_expr(arena, opd_idx))
                    .collect();
                match call_decl.output.contract {
                    OutputContract::Transitive => {
                        msg_once!("handle ref");
                        Ok(QualifiedTy {
                            qual: if self.db.is_copyable(call_decl.output.ty) {
                                Qualifier::Copyable
                            } else {
                                Qualifier::Transitive
                            },
                            ty: call_decl.output.ty,
                        })
                    }
                    OutputContract::MemberAccess => todo!(),
                }
            }
            RawExprVariant::Opn { opr, ref opds } => match opr {
                Opr::Binary(_) => todo!(),
                Opr::Prefix(_) => todo!(),
                Opr::Suffix(suffix) => match suffix {
                    SuffixOpr::MembAccess(ident) => self.infer_method(
                        opds.start,
                        ident,
                        (total_opds.start + 1)..total_opds.end,
                        arena,
                        expr_idx,
                    ),
                    SuffixOpr::Incr => todo!(),
                    SuffixOpr::Decr => todo!(),
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::WithType(_) => todo!(),
                },
                Opr::List(_) => todo!(),
            },
            _ => {
                p!(arena[total_opds.start].variant);
                todo!()
            }
        }
    }

    fn infer_method(
        &mut self,
        this: RawExprIdx,
        method_ident: RangedCustomIdentifier,
        inputs: RawExprRange,
        arena: &RawExprArena,
        expr_idx: RawExprIdx,
    ) -> InferResult<QualifiedTy> {
        todo!()
    }
}
