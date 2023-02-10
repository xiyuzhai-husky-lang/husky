use husky_opn_syntax::{BinaryOpr, BinaryPureClosedOpr};
use husky_token::FloatLiteral;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_expr_ty(
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
                lopd, opr, ropd, ..
            } => self.calc_binary_ty(lopd, opr, ropd),
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
                self.calc_call_ty(None, function_ty, implicit_arguments.as_ref(), arguments)
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
                self_expr,
                ident_token,
                ref implicit_arguments,
                nonself_arguments,
                ..
            } => {
                let Some(self_expr_ty) =
                    self.infer_new_expr_ty_resolved( self_expr, ExprTypeExpectation::None)
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
                self.calc_call_ty(
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

    fn calc_new_box_list(
        &mut self,
        expr_idx: ExprIdx,
        items: ExprIdxRange,
    ) -> Result<LocalTerm, ExprTypeError> {
        let element_ty = self.new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ImplicitType);
        for item in items {
            self.infer_new_expr_ty(
                item,
                ExprTypeExpectation::ImplicitlyConvertibleTo { ty: element_ty },
            );
        }
        Ok(self
            .intern_unresolved_term(UnresolvedTerm::TypeApplication {
                ty: self.entity_path_menu.list_ty(),
                arguments: vec![element_ty],
            })
            .into())
    }

    fn calc_call_ty(
        &mut self,
        self_ty: Option<ReducedTerm>,
        callable_ty: Option<LocalTerm>,
        implicit_arguments: Option<&ImplicitArgumentList>,
        arguments: ExprIdxRange,
    ) -> ExprTypeResult<LocalTerm> {
        let Some(callable_ty) = callable_ty
            else {
                if let Some(implicit_arguments) = implicit_arguments{
                    for argument in implicit_arguments.arguments() {
                        self.infer_new_expr_ty(argument, ExprTypeExpectation::None);
                    }
                }
                for argument in arguments {
                    self.infer_new_expr_ty(argument, ExprTypeExpectation::None);
                }
                return Err(DerivedExprTypeError::CallableTypeError.into())
            };
        p!(callable_ty.debug(self.db));
        todo!()
    }

    fn calc_binary_ty(
        &mut self,
        lopd: ExprIdx,
        opr: BinaryOpr,
        ropd: ExprIdx,
    ) -> ExprTypeResult<LocalTerm> {
        let menu = self.reduced_term_menu;
        match opr {
            BinaryOpr::PureClosed(opr) => {
                let lopd_ty = self.infer_new_expr_ty_resolved(lopd, ExprTypeExpectation::None);
                let ropd_ty = self.infer_new_expr_ty_resolved(ropd, ExprTypeExpectation::None);
                let Some(lopd_ty) = lopd_ty
                    else {
                        return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
                    };
                let Some(ropd_ty) = ropd_ty
                    else {
                        return Err(DerivedExprTypeError::BinaryOperationRightOperandTypeNotInferred.into())
                    };
                let lopd_ty = self.db.intrinsic_ty(lopd_ty).reduced_term();
                let ropd_ty = self.db.intrinsic_ty(ropd_ty).reduced_term();
                match opr {
                    BinaryPureClosedOpr::Add => match lopd_ty {
                        lopd_ty if lopd_ty == menu.i32() => match ropd_ty {
                            ropd_ty if ropd_ty == menu.i32() => Ok(menu.i32().into()),
                            _ => Err(todo!()),
                        },
                        lopd_ty if lopd_ty == menu.i64() => match ropd_ty {
                            ropd_ty if ropd_ty == menu.i64() => Ok(menu.i64().into()),
                            _ => Err(todo!()),
                        },
                        _ => Err(todo!()),
                    },
                    BinaryPureClosedOpr::BitAnd => todo!(),
                    BinaryPureClosedOpr::BitOr => todo!(),
                    BinaryPureClosedOpr::BitXor => todo!(),
                    BinaryPureClosedOpr::Div => todo!(),
                    BinaryPureClosedOpr::Mul => todo!(),
                    BinaryPureClosedOpr::RemEuclid => todo!(),
                    BinaryPureClosedOpr::Power => todo!(),
                    BinaryPureClosedOpr::Shl => todo!(),
                    BinaryPureClosedOpr::Shr => todo!(),
                    BinaryPureClosedOpr::Sub => todo!(),
                }
            }
            BinaryOpr::Comparison(_) => {
                let lopd_ty = self.infer_new_expr_ty_resolved(lopd, ExprTypeExpectation::None);
                let ropd_ty_expectation = match lopd_ty {
                    Some(_) => todo!(),
                    None => ExprTypeExpectation::None,
                };
                let ropd_ty = self.infer_new_expr_ty_resolved(ropd, ropd_ty_expectation);
                Ok(self.reduced_term_menu.bool().into())
            }
            BinaryOpr::ShortcuitLogic(_) => {
                let expectation = ExprTypeExpectation::ImplicitlyConvertibleTo {
                    ty: self.reduced_term_menu.bool().into(),
                };
                self.infer_new_expr_ty_resolved(lopd, expectation);
                self.infer_new_expr_ty_resolved(ropd, expectation);
                Ok(self.reduced_term_menu.bool().into())
            }
            BinaryOpr::Assign(opr) => {
                let lopd_ty = self.infer_new_expr_ty(lopd, ExprTypeExpectation::None);
                let ropd_ty = self.infer_new_expr_ty(ropd, ExprTypeExpectation::None);
                let Some(lopd_ty) = lopd_ty
                    else {
                        return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
                    };
                let Some(ropd_ty) = ropd_ty
                    else {
                        return Err(DerivedExprTypeError::BinaryOperationRightOperandTypeNotInferred.into())
                    };
                let lopd_ty = match lopd_ty {
                    LocalTerm::Resolved(lopd_ty) => match lopd_ty.term() {
                        Term::Application(lopd_ty) => todo!(),
                        _ => todo!(),
                    },
                    LocalTerm::Unresolved(_) => todo!(),
                };
                let ropd_ty = match ropd_ty {
                    LocalTerm::Resolved(ropd_ty) => self.db.intrinsic_ty(ropd_ty).reduced_term(),
                    LocalTerm::Unresolved(_) => todo!(),
                };
                match opr {
                    Some(opr) => match opr {
                        BinaryPureClosedOpr::Add => todo!(),
                        BinaryPureClosedOpr::BitAnd => todo!(),
                        BinaryPureClosedOpr::BitOr => todo!(),
                        BinaryPureClosedOpr::BitXor => todo!(),
                        BinaryPureClosedOpr::Div => todo!(),
                        BinaryPureClosedOpr::Mul => todo!(),
                        BinaryPureClosedOpr::RemEuclid => todo!(),
                        BinaryPureClosedOpr::Power => todo!(),
                        BinaryPureClosedOpr::Shl => todo!(),
                        BinaryPureClosedOpr::Shr => todo!(),
                        BinaryPureClosedOpr::Sub => todo!(),
                    },
                    None => todo!(),
                }
                Ok(self.reduced_term_menu.unit().into())
            }
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::Curry => {
                let Some(lopd_ty) = self.infer_new_expr_ty_resolved(lopd, ExprTypeExpectation::TypeType)
                    else {
                        return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
                    };
                let Some(ropd_ty) = self.infer_new_expr_ty_resolved(ropd, ExprTypeExpectation::TypeType)
                    else {
                        return Err(DerivedExprTypeError::BinaryOperationRightOperandTypeNotInferred.into())
                    };
                let x = lopd_ty.term();
                match x {
                    Term::Category(_) => (),
                    _ => return Err(todo!()),
                }
                let y = ropd_ty.term();
                match y {
                    Term::Category(_) => (),
                    _ => return Err(todo!()),
                }
                Ok(self
                    .db
                    .reduced_term(
                        TermCurry::new(self.db, /* ad hoc */ Variance::Invariant, x, y).into(),
                    )
                    .into())
            }
            BinaryOpr::As => {
                self.infer_new_expr_ty_resolved(ropd, ExprTypeExpectation::TypeType);
                let Some(ropd_term) = self.infer_new_expr_term(ropd)
                    else {
                        return Err(DerivedExprTypeError::AsOperationRightOperandTermNotInferred.into())
                    };
                let Some(lopd_ty) = self.infer_new_expr_ty_resolved(lopd, ExprTypeExpectation::ImplicitlyConvertibleTo{
                    ty: todo!()
                })
                    else {
                        return Err(DerivedExprTypeError::BinaryOperationLeftOperandTypeNotInferred.into())
                    };
                todo!()
            }
            BinaryOpr::Is => {
                let Some(ropd_ty) = self.infer_new_expr_ty_resolved(ropd, ExprTypeExpectation::None)
                    else {
                        return Err(DerivedExprTypeError::BinaryOperationRightOperandTypeNotInferred.into())
                    };
                let mut ropd_expectation = ExprTypeExpectation::None;
                match ropd_ty.term() {
                    Term::Entity(path) if path == self.entity_path_menu.trai_ty().into() => {
                        todo!()
                    }
                    Term::Category(_) => {
                        if let Some(ropd_term) = self.infer_new_expr_term(ropd) {
                            ropd_expectation =
                                ExprTypeExpectation::ImplicitlyConvertibleTo { ty: ropd_term }
                        }
                    }
                    _ => todo!(),
                }
                Ok(self.reduced_term_menu.prop().into())
            }
            BinaryOpr::In => todo!(),
        }
    }

    fn calc_prefix_ty(&mut self, opd: ExprIdx, opr: PrefixOpr) -> ExprTypeResult<LocalTerm> {
        match opr {
            PrefixOpr::Minus => {
                let opd_ty = self.infer_new_expr_ty(opd, ExprTypeExpectation::None);
                match opd_ty {
                    Some(opd_ty) => match opd_ty {
                        LocalTerm::Resolved(_) => todo!(),
                        LocalTerm::Unresolved(unresolved_term) => {
                            match self.unresolved_term_table[unresolved_term].unresolved_term() {
                                UnresolvedTerm::ImplicitSymbol(implicit_symbol) => {
                                    match implicit_symbol.variant() {
                                        ImplicitSymbolVariant::Lifetime => todo!(),
                                        ImplicitSymbolVariant::UnspecifiedIntegerType
                                        | ImplicitSymbolVariant::UnspecifiedFloatType => Ok(opd_ty),
                                        ImplicitSymbolVariant::ImplicitType => todo!(),
                                    }
                                }
                                UnresolvedTerm::TypeApplication { ty, arguments } => todo!(),
                            }
                        }
                    },
                    None => Err(DerivedExprTypeError::PrefixOperandTypeNotInferred.into()),
                }
            }
            PrefixOpr::Not => {
                let _opd_ty = self.infer_new_expr_ty(opd, ExprTypeExpectation::CastibleAsBool);
                // here we differs from Rust, but agrees with C
                Ok(self.reduced_term_menu.bool().into())
            }
            PrefixOpr::BitNot => todo!(),
            PrefixOpr::Ref => {
                let opd_ty = self.infer_new_expr_ty(opd, ExprTypeExpectation::None);
                // Should consider more cases, could also be taking references
                opd_ty.ok_or(DerivedExprTypeError::PrefixOperandTypeNotInferred.into())
            }
            PrefixOpr::Vector => todo!(),
            PrefixOpr::Slice => todo!(),
            PrefixOpr::CyclicSlice => todo!(),
            PrefixOpr::Array(_) => todo!(),
            PrefixOpr::Option => {
                let opd_ty = self.infer_new_expr_ty(opd, ExprTypeExpectation::TypeType);
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
                        let argument_ty =
                            self.infer_new_expr_ty(argument, ExprTypeExpectation::None);
                        // check this is type
                        argument_ty
                            .ok_or(DerivedExprTypeError::ApplicationArgumentTypeNotInferred.into())
                    }
                    1 => {
                        let arg0_ty =
                            self.infer_new_expr_ty(items.start(), ExprTypeExpectation::None);
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
                let function_ty = self.infer_new_expr_ty(function, ExprTypeExpectation::None);
                todo!()
            }
        }
    }

    fn calc_literal(
        &mut self,
        expr_idx: ExprIdx,
        literal_token_idx: TokenIdx,
        expectation: ExprTypeExpectation,
    ) -> Result<LocalTerm, ExprTypeError> {
        let literal_token = self.token_sheet_data[literal_token_idx];
        match literal_token {
            Token::Literal(literal) => match literal {
                Literal::Unit => todo!(),
                Literal::Char(_) => todo!(),
                Literal::String(_) => Ok(self.reduced_term_menu.static_str_ref().into()),
                Literal::Integer(integer_literal) => match integer_literal {
                    IntegerLikeLiteral::Unspecified => match expectation.term() {
                        // MOM
                        Some(term) if term == self.reduced_term_menu.i32() => todo!(),
                        _ => Ok(self
                            .new_implicit_symbol(
                                expr_idx,
                                ImplicitSymbolVariant::UnspecifiedIntegerType,
                            )
                            .into()),
                    },
                    IntegerLikeLiteral::I8(_) => todo!(),
                    IntegerLikeLiteral::I16(_) => todo!(),
                    IntegerLikeLiteral::I32(_) => Ok(self.reduced_term_menu.i32().into()),
                    IntegerLikeLiteral::I64(_) => todo!(),
                    IntegerLikeLiteral::I128(_) => todo!(),
                    IntegerLikeLiteral::ISize(_) => todo!(),
                    IntegerLikeLiteral::R8(_) => todo!(),
                    IntegerLikeLiteral::R16(_) => todo!(),
                    IntegerLikeLiteral::R32(_) => todo!(),
                    IntegerLikeLiteral::R64(_) => todo!(),
                    IntegerLikeLiteral::R128(_) => todo!(),
                    IntegerLikeLiteral::RSize(_) => todo!(),
                    IntegerLikeLiteral::U8(_) => todo!(),
                    IntegerLikeLiteral::U16(_) => todo!(),
                    IntegerLikeLiteral::U32(_) => todo!(),
                    IntegerLikeLiteral::U64(_) => todo!(),
                    IntegerLikeLiteral::U128(_) => todo!(),
                    IntegerLikeLiteral::USize(_) => todo!(),
                },
                Literal::Float(float_literal) => match float_literal {
                    FloatLiteral::Unspecified => match expectation {
                        ExprTypeExpectation::None => {
                            let ty = self.new_implicit_symbol(
                                expr_idx,
                                ImplicitSymbolVariant::UnspecifiedFloatType,
                            );
                            Ok(ty.into())
                        }
                        ExprTypeExpectation::TypeType => todo!(),
                        ExprTypeExpectation::CastibleAsBool => todo!(),
                        ExprTypeExpectation::FrameVariableType => todo!(),
                        ExprTypeExpectation::Return { ty } => todo!(),
                        ExprTypeExpectation::ImplicitlyConvertibleTo { ty } => todo!(),
                    },
                    FloatLiteral::F32(_) => todo!(),
                    FloatLiteral::F64(_) => todo!(),
                },
                Literal::TupleIndex(_) => todo!(),
                Literal::Bool(_) => Ok(self.reduced_term_menu.bool().into()),
            },
            _ => unreachable!(),
        }
    }

    fn infer_new_expr_term(&mut self, expr_idx: ExprIdx) -> Option<LocalTerm> {
        let term_result = self.calc_expr_term(expr_idx);
        let term = term_result.as_ref().ok().copied();
        self.save_new_expr_term(expr_idx, term_result);
        term
    }

    fn save_new_expr_term(&mut self, expr_idx: ExprIdx, term_result: ExprTermResult<LocalTerm>) {
        self.expr_terms.insert_new(expr_idx, term_result)
    }

    fn calc_expr_term(&mut self, expr_idx: ExprIdx) -> ExprTermResult<LocalTerm> {
        match self.expr_region_data[expr_idx] {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath {
                entity_path_expr,
                entity_path,
            } => match entity_path {
                Some(entity_path) => Ok(self.db.reduced_term(entity_path.into()).into()),
                None => todo!(),
            },
            Expr::InheritedSymbol {
                ident,
                token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => todo!(),
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
            } => todo!(),
            Expr::Be { .. } => todo!(),
            Expr::PrefixOpn {
                opr,
                opr_token_idx,
                opd,
            } => todo!(),
            Expr::SuffixOpn {
                opd,
                punctuation,
                punctuation_token_idx,
            } => todo!(),
            Expr::ApplicationOrFunctionCall { .. } => todo!(),
            Expr::FunctionCall { .. } => todo!(),
            Expr::Field {
                owner,
                dot_token_idx,
                ident_token,
            } => todo!(),
            Expr::MethodCall { .. } => todo!(),
            Expr::TemplateInstantiation { .. } => todo!(),
            Expr::Application { function, argument } => todo!(),
            Expr::Bracketed {
                lpar_token_idx,
                item,
                rpar_token_idx,
            } => todo!(),
            Expr::NewTuple { .. } => todo!(),
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
            Expr::Block { stmts } => todo!(),
            Expr::Err(_) => Err(DerivedExprTermError::ExprError.into()),
        }
    }
}
