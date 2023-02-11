mod application;
mod binary;
mod box_list;
mod literal;
mod prefix;
mod ritchie_call;

use super::*;
use husky_opn_syntax::*;

impl<'a> ExprTypeEngine<'a> {
    pub(in super::super) fn infer_new_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        expectation: ExprTypeExpectation,
    ) -> Option<LocalTerm> {
        let ty_result = self.calc_expr_ty(expr_idx, expectation);
        let (ty, opt_expectation) = match ty_result {
            Ok(ty) => (
                Some(ty),
                self.add_expectation_rule(expr_idx, ty, expectation),
            ),
            Err(_) => (None, Default::default()),
        };
        self.save_new_expr_ty(expr_idx, ExprTypeInfo::new(ty_result, opt_expectation));
        ty
    }

    fn infer_new_expr_ty_resolved(
        &mut self,
        expr_idx: ExprIdx,
        expectation: ExprTypeExpectation,
    ) -> Option<ReducedTerm> {
        match self.infer_new_expr_ty(expr_idx, expectation)? {
            LocalTerm::Resolved(lopd_ty) => Some(lopd_ty),
            LocalTerm::Unresolved(lopd_ty) => self.resolve_term(lopd_ty),
        }
    }

    fn save_new_expr_ty(&mut self, expr_idx: ExprIdx, info: ExprTypeInfo) {
        self.expr_ty_infos.insert_new(expr_idx, info)
    }

    fn calc_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        expectation: ExprTypeExpectation,
    ) -> ExprTypeResult<LocalTerm> {
        match self.expr_region_data[expr_idx] {
            Expr::Literal(literal_token_idx) => {
                self.calc_literal(expr_idx, literal_token_idx, expectation)
            }
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => match entity_path {
                Some(entity_path) => match self.db.entity_ty(entity_path) {
                    Ok(ty) => Ok(ty.into()),
                    Err(_) => Err(DerivedExprTypeError::EntityTypeError.into()),
                },
                None => todo!(),
            },
            Expr::InheritedSymbol {
                ident,
                inherited_symbol_idx,
                ..
            } => match self.inherited_symbol_tys.get(inherited_symbol_idx) {
                Some(ty) => Ok((*ty).into()),
                None => Err(DerivedExprTypeError::InheritedSymbolTypeError.into()),
            },
            Expr::CurrentSymbol {
                ident,
                current_symbol_idx,
                current_symbol_kind,
                ..
            } => self
                .current_symbol_tys
                .get(current_symbol_idx)
                .copied()
                .ok_or(DerivedExprTypeError::CurrentSymbolTypeError.into()),
            Expr::FrameVarDecl {
                ident,
                current_symbol_idx,
                current_symbol_kind,
                ..
            } => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::SelfValue(_) => todo!(),
            Expr::BinaryOpn {
                lopd,
                opr,
                ropd,
                opr_token_idx,
                ..
            } => {
                p!(opr_token_idx, self.path());
                self.calc_binary_expr_ty(lopd, opr, ropd)
            }
            Expr::Be {
                src, ref target, ..
            } => todo!(),
            Expr::PrefixOpn { opr, opd, .. } => self.calc_prefix_ty(opd, opr),
            Expr::SuffixOpn {
                opd, punctuation, ..
            } => todo!(),
            Expr::ApplicationOrFunctionCall {
                function, argument, ..
            } => {
                let function_ty = self.infer_new_expr_ty(function, ExprTypeExpectation::None);
                match function_ty {
                    Some(function_ty) => todo!(),
                    None => {
                        self.infer_new_expr_ty(argument, ExprTypeExpectation::None);
                        Err(DerivedExprTypeError::FunctionTypeNotInferredInApplicationOrFunctionCall.into())
                    }
                }
            }
            Expr::FunctionCall {
                function,
                ref implicit_arguments,
                arguments,
                ..
            } => {
                let function_ty = self.infer_new_expr_ty(function, ExprTypeExpectation::None);
                self.calc_ritchie_call_ty(function_ty, implicit_arguments.as_ref(), arguments)
            }
            Expr::Field {
                owner, ident_token, ..
            } => {
                if let Some(owner_ty) =
                    self.infer_new_expr_ty_resolved(owner, ExprTypeExpectation::None)
                {
                    let field_ty = self.db.field_ty(owner_ty, ident_token.ident());
                    match field_ty {
                        Ok(_) => todo!(),
                        Err(e) => Err(e.into()),
                    }
                } else {
                    Err(DerivedExprTypeError::FieldOwnerTypeNotInferred.into())
                }
            }
            Expr::MethodCall {
                self_argument,
                ident_token,
                ref implicit_arguments,
                nonself_arguments,
                ..
            } => {
                let Some(self_expr_ty) =
                    self.infer_new_expr_ty_resolved( self_argument, ExprTypeExpectation::None)
                    else {
                        if let Some(implicit_arguments) = implicit_arguments {
                            todo!()
                        }
                        for argument in nonself_arguments {
                            todo!()
                        }
                        return Err(DerivedExprTypeError::MethodOwnerTypeNotInferred.into())
                    };
                let method_ty = match self.db.ty_method_ty(self_expr_ty, ident_token.ident()) {
                    Ok(_) => todo!(),
                    Err(e) => return Err(e.into()),
                };
                self.calc_ritchie_call_ty(method_ty, implicit_arguments.as_ref(), nonself_arguments)
            }
            Expr::TemplateInstantiation {
                template,
                ref implicit_arguments,
            } => todo!(),
            Expr::Application { function, argument } => self.calc_application(function, argument),
            Expr::Bracketed { item, .. } => self
                .infer_new_expr_ty(item, expectation)
                .ok_or(DerivedExprTypeError::BracketedItemTypeError.into()),
            Expr::NewTuple { items, .. } => todo!(),
            Expr::NewBoxList { caller, items, .. } => self.calc_new_box_list(expr_idx, items),
            Expr::BoxColon { caller, .. } => todo!(),
            Expr::Block { stmts } => self
                .infer_new_block(stmts, expectation)
                .ok_or(DerivedExprTypeError::BlockTypeError.into()),
            Expr::Err(_) => Err(DerivedExprTypeError::ExprError.into()),
        }
    }
}
