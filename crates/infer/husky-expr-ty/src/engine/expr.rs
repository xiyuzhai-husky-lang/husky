use husky_token::FloatLiteral;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_expr(
        &mut self,
        expr_idx: ExprIdx,
        expectation: Expectation,
    ) -> Option<LocalTerm> {
        let ty_result = self.calc_expr(expr_idx, expectation);
        let (ty, opt_expectation) = match ty_result {
            Ok(ty) => (
                Some(ty),
                self.unresolved_term_table
                    .add_expectation_rule(ty, expectation, self.term_menu),
            ),
            Err(_) => (None, Default::default()),
        };
        self.save_expr(expr_idx, TypeInfo::new(ty_result, opt_expectation));
        ty
    }

    fn infer_new_expr_resolved(
        &mut self,
        expr_idx: ExprIdx,
        expectation: Expectation,
    ) -> Option<Term> {
        match self.infer_new_expr(expr_idx, expectation)? {
            LocalTerm::Resolved(lopd_ty) => Some(lopd_ty),
            LocalTerm::Unresolved(lopd_ty) => self.unresolved_term_table.resolve_term(lopd_ty),
        }
    }

    fn save_expr(&mut self, expr_idx: ExprIdx, info: TypeInfo) {
        self.expr_ty_infos.insert_new(expr_idx, info)
    }

    fn calc_expr(
        &mut self,
        expr_idx: ExprIdx,
        expectation: Expectation,
    ) -> ExprTypeResult<LocalTerm> {
        match self.expr_region_data[expr_idx] {
            Expr::Literal(literal_token_idx) => self.calc_literal(literal_token_idx, expectation),
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
                token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => match self.current_symbol_ty_infos.get(current_symbol_idx) {
                Some(ty_info) => ty_info
                    .ty()
                    .map_err(|_| DerivedExprTypeError::CurrentSymbolTypeError.into()),
                None => Err(DerivedExprTypeError::CurrentSymbolTypeError.into()),
            },
            Expr::FrameVarDecl {
                token_idx,
                ident,
                current_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::SelfValue(_) => todo!(),
            Expr::BinaryOpn {
                lopd,
                opr,
                opr_token_idx,
                ropd,
            } => self.calc_binary(lopd, ropd),
            Expr::Be {
                src,
                be_token_idx,
                ref target,
            } => todo!(),
            Expr::PrefixOpn {
                opr,
                opr_token_idx,
                opd,
            } => self.calc_prefix(opd, opr),
            Expr::SuffixOpn {
                opd,
                punctuation,
                punctuation_token_idx,
            } => todo!(),
            Expr::ApplicationOrFunctionCall {
                function,
                lpar_token_idx,
                argument,
                rpar_token_idx,
            } => todo!(),
            Expr::FunctionCall {
                function,
                ref implicit_arguments,
                lpar_token_idx,
                arguments,
                rpar_token_idx,
            } => {
                let function_ty = self.infer_new_expr(function, Expectation::None);
                self.calc_call_expr(None, function_ty, implicit_arguments.as_ref(), arguments)
            }
            Expr::Field {
                self_expr,
                dot_token_idx,
                ident_token,
            } => todo!(),
            Expr::MethodCall {
                self_expr,
                ident_token,
                ref implicit_arguments,
                nonself_arguments,
                ..
            } => {
                let Some(self_expr_ty) =
                    self.infer_new_expr_resolved( self_expr, Expectation::None)
                    else {
                        todo!()
                    };
                let method_ty = match self.db.ty_method_ty(self_expr_ty, ident_token.ident()) {
                    Ok(_) => todo!(),
                    Err(e) => return Err(e.into()),
                };
                self.calc_call_expr(
                    Some(self_expr_ty),
                    method_ty,
                    implicit_arguments.as_ref(),
                    nonself_arguments,
                )
            }
            Expr::TemplateInstantiation {
                template,
                ref implicit_arguments,
            } => todo!(),
            Expr::Application { function, argument } => self.calc_application(function, argument),
            Expr::Bracketed {
                lpar_token_idx,
                item,
                rpar_token_idx,
            } => self
                .infer_new_expr(item, expectation)
                .ok_or(DerivedExprTypeError::BracketedItemTypeError.into()),
            Expr::NewTuple { items, .. } => todo!(),
            Expr::NewBoxList { caller, items, .. } => todo!(),
            Expr::BoxColon { caller, .. } => todo!(),
            Expr::Block { stmts } => self
                .infer_new_block(stmts, expectation)
                .ok_or(DerivedExprTypeError::BlockTypeError.into()),
            Expr::Err(_) => Err(DerivedExprTypeError::ExprError.into()),
        }
    }

    fn calc_call_expr(
        &mut self,
        self_ty: Option<Term>,
        callable_ty: Option<LocalTerm>,
        implicit_arguments: Option<&ImplicitArgumentList>,
        arguments: ExprIdxRange,
    ) -> ExprTypeResult<LocalTerm> {
        let Some(callable_ty) = callable_ty
            else {
                if let Some(implicit_arguments) = implicit_arguments{
                    for argument in implicit_arguments.arguments() {
                        self.infer_new_expr(argument, Expectation::None);
                    }
                }
                for argument in arguments {
                    self.infer_new_expr(argument, Expectation::None);
                }
                return Err(DerivedExprTypeError::CallableTypeError.into())
            };
        todo!()
    }

    fn calc_binary(&mut self, lopd: ExprIdx, ropd: ExprIdx) -> ExprTypeResult<LocalTerm> {
        let Some(lopd_ty) = self.infer_new_expr_resolved(lopd, Expectation::None)
            else {
                return Err(DerivedExprTypeError::BinaryOpnFirstArgumentTypeNotInferred.into())
            };
        let Some(ropd_ty) = self.infer_new_expr_resolved(ropd, Expectation::None)
            else {
                return Err(DerivedExprTypeError::BinaryOpnSecondArgumentTypeNotInferred.into())
            };
        todo!()
    }

    fn calc_prefix(&mut self, opd: ExprIdx, opr: PrefixOpr) -> ExprTypeResult<LocalTerm> {
        let opd_ty = self.infer_new_expr(opd, Expectation::None);
        match opr {
            PrefixOpr::Minus => todo!(),
            PrefixOpr::Not => todo!(),
            PrefixOpr::BitNot => todo!(),
            PrefixOpr::Ref => {
                // Should consider more cases, could also be taking references
                opd_ty.ok_or(DerivedExprTypeError::PrefixOperandTypeNotInferred.into())
            }
            PrefixOpr::Vector => todo!(),
            PrefixOpr::Slice => todo!(),
            PrefixOpr::CyclicSlice => todo!(),
            PrefixOpr::Array(_) => todo!(),
            PrefixOpr::Option => {
                // Should check this is type.
                opd_ty.ok_or(DerivedExprTypeError::PrefixOperandTypeNotInferred.into())
            }
        }
    }

    fn calc_application(
        &mut self,
        function: ExprIdx,
        argument: ExprIdx,
    ) -> Result<LocalTerm, ExprTypeError> {
        let function_expr = &self[function];
        match function_expr {
            Expr::NewBoxList {
                caller: None,
                lbox_token_idx,
                items,
                rbox_token_idx,
            } => {
                match items.len() {
                    0 => {
                        let argument_ty = self.infer_new_expr(argument, Expectation::None);
                        // check this is type
                        argument_ty
                            .ok_or(DerivedExprTypeError::ApplicationArgumentTypeNotInferred.into())
                    }
                    1 => {
                        let arg0_ty = self.infer_new_expr(items.start(), Expectation::None);
                        match arg0_ty {
                            Some(_) => todo!(),
                            None => {
                                Err(DerivedExprTypeError::BoxListApplicationFirstArgumentError
                                    .into())
                            }
                        }
                    }
                    n => {
                        todo!()
                    }
                }
            }
            Expr::BoxColon {
                caller,
                lbox_token_idx,
                colon_token_idx,
                rbox_token,
            } => todo!(),
            _ => {
                let function_ty = self.infer_new_expr(function, Expectation::None);
                todo!()
            }
        }
    }

    fn calc_literal(
        &mut self,
        literal_token_idx: TokenIdx,
        expectation: Expectation,
    ) -> Result<LocalTerm, ExprTypeError> {
        let literal_token = self.token_sheet_data[literal_token_idx];
        match literal_token {
            Token::Literal(literal) => match literal {
                Literal::Unit => todo!(),
                Literal::Char(_) => todo!(),
                Literal::String(_) => todo!(),
                Literal::Integer(integer_literal) => match integer_literal {
                    IntegerLiteral::Unspecified => match expectation {
                        Expectation::None => {
                            let ty = self
                                .implicit_symbol_registry
                                .new_unspecified_integer_ty_symbol(self.term_menu);
                            let ty = self
                                .unresolved_term_table
                                .intern_unresolved_term(UnresolvedTerm::ImplicitSymbol(ty));
                            Ok(ty.into())
                        }
                        Expectation::Type => todo!(),
                        Expectation::UnitOrNever => todo!(),
                        Expectation::Condition => todo!(),
                        Expectation::Return { ty } => todo!(),
                    },
                    IntegerLiteral::I8(_) => todo!(),
                    IntegerLiteral::I16(_) => todo!(),
                    IntegerLiteral::I32(_) => todo!(),
                    IntegerLiteral::I64(_) => todo!(),
                    IntegerLiteral::I128(_) => todo!(),
                    IntegerLiteral::ISize(_) => todo!(),
                    IntegerLiteral::R8(_) => todo!(),
                    IntegerLiteral::R16(_) => todo!(),
                    IntegerLiteral::R32(_) => todo!(),
                    IntegerLiteral::R64(_) => todo!(),
                    IntegerLiteral::R128(_) => todo!(),
                    IntegerLiteral::RSize(_) => todo!(),
                    IntegerLiteral::U8(_) => todo!(),
                    IntegerLiteral::U16(_) => todo!(),
                    IntegerLiteral::U32(_) => todo!(),
                    IntegerLiteral::U64(_) => todo!(),
                    IntegerLiteral::U128(_) => todo!(),
                    IntegerLiteral::USize(_) => todo!(),
                },
                Literal::Float(float_literal) => match float_literal {
                    FloatLiteral::Unspecified => match expectation {
                        Expectation::None => {
                            let ty = self
                                .implicit_symbol_registry
                                .new_unspecified_float_ty_symbol(self.term_menu);
                            let ty = self
                                .unresolved_term_table
                                .intern_unresolved_term(UnresolvedTerm::ImplicitSymbol(ty));
                            Ok(ty.into())
                        }
                        Expectation::Type => todo!(),
                        Expectation::UnitOrNever => todo!(),
                        Expectation::Condition => todo!(),
                        Expectation::Return { ty } => todo!(),
                    },
                    FloatLiteral::F32(_) => todo!(),
                    FloatLiteral::F64(_) => todo!(),
                },
                Literal::TupleIndex(_) => todo!(),
                Literal::Bool(_) => Ok(self.term_menu.bool().into()),
            },
            _ => unreachable!(),
        }
    }
}
