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
                    .add_expection_rule(ty, expectation),
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
        expection: Expectation,
    ) -> ExprTypeResult<LocalTerm> {
        let expr = &self.expr_region_data[expr_idx];
        match expr {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => match entity_path {
                Some(entity_path) => match self.db.entity_ty(*entity_path) {
                    Ok(ty) => Ok(ty.into()),
                    Err(_) => Err(DerivedExprTypeError::EntityTypeError.into()),
                },
                None => todo!(),
            },
            Expr::InheritedSymbol {
                ident,
                inherited_symbol_idx,
                ..
            } => {
                // match inherited_symbol_kind {
                //     InheritedSymbolKind::ImplicitParameter => todo!(),
                //     InheritedSymbolKind::RegularParameter => todo!(),
                // }
                p!(
                    ident.debug(self.db),
                    inherited_symbol_idx,
                    self.inherited_symbol_tys.debug(self.db)
                );
                match self.inherited_symbol_tys.get(*inherited_symbol_idx) {
                    Some(ty) => Ok((*ty).into()),
                    None => todo!(),
                }
            }
            Expr::CurrentSymbol {
                ident,
                token_idx,
                current_symbol_idx,
                current_symbol_kind,
            } => todo!(),
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
            } => self.calc_binary(*lopd, *ropd),
            Expr::Be {
                src,
                be_token_idx,
                target,
            } => todo!(),
            Expr::PrefixOpn {
                opr,
                opr_token_idx,
                opd,
            } => self.calc_prefix(*opd, *opr),
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
                implicit_arguments,
                lpar_token_idx,
                arguments,
                rpar_token_idx,
            } => todo!(),
            Expr::Field {
                this_expr,
                dot_token_idx,
                ident_token,
            } => todo!(),
            Expr::MethodCall {
                this_expr,
                ident_token,
                implicit_arguments,
                arguments,
                ..
            } => {
                let Some(this_expr_ty) =
                    self.infer_new_expr_resolved(*this_expr, Expectation::None)
                    else {
                        todo!()
                    };
                let method_ty = match self.db.ty_method_ty(this_expr_ty, ident_token.ident()) {
                    Ok(_) => todo!(),
                    Err(e) => return Err(e.into()),
                };
                todo!()
            }
            Expr::TemplateInstantiation {
                template,
                implicit_arguments,
            } => todo!(),
            Expr::Application { function, argument } => self.calc_application(*function, *argument),
            Expr::Bracketed {
                lpar_token_idx,
                item,
                rpar_token_idx,
            } => todo!(),
            Expr::NewTuple {
                lpar_token_idx,
                items,
                commas,
                rpar_token_idx,
            } => todo!(),
            Expr::NewBoxList {
                caller,
                lbox_token_idx,
                items,
                rbox_token_idx,
            } => todo!(),
            Expr::BoxColon {
                caller,
                lbox_token_idx,
                colon_token_idx,
                rbox_token,
            } => todo!(),
            Expr::Block { stmts } => self
                .infer_new_stmts(*stmts)
                .ok_or(DerivedExprTypeError::BlockTypeError.into()),
            Expr::Err(_) => Err(DerivedExprTypeError::ExprError.into()),
        }
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
}
