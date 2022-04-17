use ast::*;

use entity_route::{EntityRouteKind, EntityRoutePtr};
use infer_error::*;
use syntax_types::{ListOpr, Opr, PrefixOpr, SuffixOpr};
use text::TextRange;
use vm::{BinaryOpr, FieldContract};
use word::{CustomIdentifier, RangedCustomIdentifier};

use super::*;
use crate::*;

impl<'a> ContractSheetBuilder<'a> {
    pub(crate) fn infer_morphism(
        &mut self,
        output_ty: EntityRoutePtr,
        ast_iter: AstIter,
        arena: &RawExprArena,
    ) {
        self.infer_lazy_stmts(ast_iter.clone(), output_ty, arena);
    }
    pub(super) fn infer_lazy_stmts(
        &mut self,
        ast_iter: AstIter,
        output_ty: EntityRoutePtr,
        arena: &RawExprArena,
    ) {
        self.enter_block();
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.kind {
                    AstKind::Stmt(ref stmt) => self.infer_lazy_stmt(stmt, output_ty, arena),
                    _ => (),
                }
            }
            if let Some(children) = item.children {
                self.infer_lazy_stmts(children, output_ty, arena)
            }
        }
        self.exit_block()
    }

    fn infer_lazy_stmt(&mut self, stmt: &RawStmt, output_ty: EntityRoutePtr, arena: &RawExprArena) {
        match stmt.kind {
            RawStmtKind::Loop(raw_loop_kind) => panic!(),
            RawStmtKind::Branch(_) => todo!(),
            RawStmtKind::Exec(expr) => panic!(),
            RawStmtKind::Init {
                varname,
                initial_value,
                ..
            } => {
                self.infer_lazy_expr(initial_value, LazyContract::Take, arena);
            }
            RawStmtKind::Return(result) => self.infer_lazy_expr(result, LazyContract::Take, arena),
            RawStmtKind::Assert(condition) => self.infer_lazy_condition(condition, arena),
        }
    }

    fn infer_lazy_condition(&mut self, condition: RawExprIdx, arena: &RawExprArena) {
        self.infer_lazy_expr(condition, LazyContract::Pure, arena)
    }

    pub(super) fn infer_lazy_expr(
        &mut self,
        expr_idx: RawExprIdx,
        contract: LazyContract,
        arena: &RawExprArena,
    ) {
        let infer_result = match arena[expr_idx].kind {
            RawExprVariant::Variable { .. }
            | RawExprVariant::Unrecognized(_)
            | RawExprVariant::Scope { .. }
            | RawExprVariant::PrimitiveLiteral(_)
            | RawExprVariant::This { .. } => Ok(()),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn { opr, ref opds } => {
                self.infer_lazy_opn(opr, opds, contract, arena, arena[expr_idx].range)
            }
            RawExprVariant::Lambda(_, _) => todo!(),
        };
        should!(self
            .contract_sheet
            .lazy_expr_contract_results
            .insert(expr_idx, infer_result.map(|_| contract))
            .is_none());
    }

    fn infer_lazy_opn(
        &mut self,
        opr: Opr,
        opds: &RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
        range: TextRange,
    ) -> InferResult<()> {
        match opr {
            Opr::Binary(opr) => self.infer_lazy_binary_opn(opr, opds, contract, arena),
            Opr::Prefix(opr) => self.infer_lazy_prefix_opn(opr, opds.start, contract, arena),
            Opr::Suffix(opr) => self.infer_lazy_suffix(opr, opds.start, contract, arena),
            Opr::List(opr) => self.infer_lazy_list_opn(opr, opds, contract, arena, range),
        }
    }

    fn infer_lazy_binary_opn(
        &mut self,
        opr: BinaryOpr,
        opds: &RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        let lopd = opds.start;
        let ropd = opds.start + 1;
        match opr {
            BinaryOpr::Pure(pure_binary_opr) => {
                match contract {
                    LazyContract::Take => (),
                    LazyContract::Ref => todo!(),
                    LazyContract::Pure => (),
                }
                self.infer_lazy_expr(lopd, LazyContract::Pure, arena);
                self.infer_lazy_expr(ropd, LazyContract::Pure, arena);
            }
            BinaryOpr::Assign(_) => panic!(),
        }
        Ok(())
    }

    fn infer_lazy_prefix_opn(
        &mut self,
        opr: PrefixOpr,
        opd: RawExprIdx,
        contract: LazyContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        todo!()
    }

    fn infer_lazy_suffix(
        &mut self,
        opr: SuffixOpr,
        opd: RawExprIdx,
        contract: LazyContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        match opr {
            SuffixOpr::Incr => todo!(),
            SuffixOpr::Decr => todo!(),
            SuffixOpr::MayReturn => panic!("should handle this case in parse return statement"),
            SuffixOpr::MembAccess(ranged_ident) => {
                let this_ty_decl = self.expr_ty_decl(opd)?;
                let this_contract = match this_ty_decl.field_decl(ranged_ident)?.contract {
                    FieldContract::Own => match contract {
                        LazyContract::Take => LazyContract::Take,
                        LazyContract::Ref => todo!(),
                        LazyContract::Pure => LazyContract::Pure,
                    },
                    FieldContract::Ref => todo!(),
                    FieldContract::LazyOwn => match contract {
                        LazyContract::Take => todo!(),
                        LazyContract::Ref => todo!(),
                        LazyContract::Pure => LazyContract::Pure,
                    },
                };
                self.infer_lazy_expr(opd, this_contract, arena);
                Ok(())
            }
            SuffixOpr::WithType(_) => todo!(),
        }
    }

    fn infer_lazy_list_opn(
        &mut self,
        opr: ListOpr,
        opds: &RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
        range: TextRange,
    ) -> InferResult<()> {
        match opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => todo!(),
            ListOpr::NewDict => todo!(),
            ListOpr::Call => self.infer_lazy_list_call(opds, contract, arena, range),
            ListOpr::Index => todo!(),
            ListOpr::ModuloIndex => todo!(),
            ListOpr::StructInit => todo!(),
        }
    }

    fn infer_lazy_list_call(
        &mut self,
        all_opds: &RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
        range: TextRange,
    ) -> InferResult<()> {
        let call_expr = &arena[all_opds.start];
        match call_expr.kind {
            RawExprVariant::Scope { scope, .. } => {
                let call_decl = self.db.call_decl(scope)?;
                for i in 0..call_decl.inputs.len() {
                    self.infer_lazy_expr(
                        all_opds.start + 1 + i,
                        call_decl.inputs[i].contract.lazy()?,
                        arena,
                    )
                }
                Ok(())
            }
            RawExprVariant::Variable { .. } => todo!(),
            RawExprVariant::Unrecognized(_) => todo!(),
            RawExprVariant::PrimitiveLiteral(_) => todo!(),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn { opr, ref opds } => match opr {
                Opr::Binary(_) => todo!(),
                Opr::Prefix(_) => todo!(),
                Opr::Suffix(suffix_opr) => match suffix_opr {
                    SuffixOpr::Incr => todo!(),
                    SuffixOpr::Decr => todo!(),
                    SuffixOpr::MayReturn => todo!(),
                    SuffixOpr::MembAccess(ranged_ident) => self.infer_lazy_method(
                        opds.start,
                        ranged_ident,
                        (all_opds.start + 1)..all_opds.end,
                        contract,
                        arena,
                    ),
                    SuffixOpr::WithType(_) => todo!(),
                },
                Opr::List(_) => todo!(),
            },
            RawExprVariant::Lambda(_, _) => todo!(),
            RawExprVariant::This { .. } => todo!(),
        }
    }

    fn infer_lazy_method(
        &mut self,
        this: RawExprIdx,
        ranged_ident: RangedCustomIdentifier,
        inputs: RawExprRange,
        contract: LazyContract,
        arena: &RawExprArena,
    ) -> InferResult<()> {
        let this_ty_decl = derived_ok!(self.expr_ty_decl(this));
        let (_, method_call_decl) =
            derived_ok!(this_ty_decl.method(ranged_ident, &self.trait_uses));
        match contract {
            LazyContract::Take => (),
            LazyContract::Ref => todo!(),
            LazyContract::Pure => (),
        }
        self.infer_lazy_expr(this, method_call_decl.this_contract.lazy()?, arena);
        if inputs.end - inputs.start != method_call_decl.inputs.len() {
            todo!()
        }
        for i in 0..method_call_decl.inputs.len() {
            self.infer_lazy_expr(
                inputs.start + 1,
                method_call_decl.inputs[i].contract.lazy()?,
                arena,
            )
        }
        Ok(())
    }
}
