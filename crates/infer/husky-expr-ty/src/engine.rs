use husky_opn_syntax::PrefixOpr;
use husky_print_utils::p;

use crate::*;

pub(crate) struct ExprTypeEngine<'a> {
    db: &'a dyn ExprTypeDb,
    term_menu: &'a TermMenu,
    expr_region_data: &'a ExprRegionData,
    expr_ty_infos: ExprMap<ExprTypeInfo>,
    unresolved_term_table: UnresolvedTermTable,
}

impl<'a> std::ops::Index<ExprIdx> for ExprTypeEngine<'a> {
    type Output = Expr;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_region_data[index]
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn new(db: &'a dyn ExprTypeDb, expr_region: ExprRegion) -> Self {
        let expr_region_data = expr_region.data(db);
        Self {
            db,
            term_menu: db.term_menu(expr_region.toolchain(db)).as_ref().unwrap(),
            expr_region_data,
            expr_ty_infos: ExprMap::new(expr_region_data.expr_arena()),
            unresolved_term_table: Default::default(),
        }
    }

    pub(crate) fn infer_all(&mut self) {
        for root in self.expr_region_data.roots() {
            let ty = self.infer_new(root.expr(), None);
            // todo: check coherence
        }
    }

    fn infer_new(
        &mut self,
        expr_idx: ExprIdx,
        expectation: Option<Expectation>,
    ) -> Option<LocalTerm> {
        let ty_result = self.calc(expr_idx, expectation.as_ref());
        let ty = match ty_result {
            Ok(ty) => Some(ty),
            Err(_) => None,
        };
        let opt_expectation = self.unresolved_term_table.intern_expection(expectation);
        self.save(expr_idx, ExprTypeInfo::new(ty_result, opt_expectation));
        ty
    }

    fn infer_new_resolved(
        &mut self,
        expr_idx: ExprIdx,
        expectation: Option<Expectation>,
    ) -> Option<Term> {
        match self.infer_new(expr_idx, expectation)? {
            LocalTerm::Resolved(lopd_ty) => Some(lopd_ty),
            LocalTerm::Unresolved(lopd_ty) => self.unresolved_term_table.resolve_term(lopd_ty),
        }
    }

    fn save(&mut self, expr_idx: ExprIdx, info: ExprTypeInfo) {
        self.expr_ty_infos.insert_new(expr_idx, info)
    }

    pub(crate) fn finish(self) -> ExprTypeRegion {
        ExprTypeRegion::new(self.expr_region_data.path(), self.expr_ty_infos)
    }

    fn calc(
        &mut self,
        expr_idx: ExprIdx,
        expection: Option<&Expectation>,
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
            } => {
                let Some(lopd_ty) = self.infer_new_resolved(*lopd, None)
                    else {
                        return Err(DerivedExprTypeError::BinaryOpnFirstArgumentTypeNotInferred.into())
                    };
                let Some(ropd_ty) = self.infer_new_resolved(*ropd, None)
                    else {
                        return Err(DerivedExprTypeError::BinaryOpnsecondArgumentTypeNotInferred.into())
                    };
                todo!()
            }
            Expr::Be {
                src,
                be_token_idx,
                target,
            } => todo!(),
            Expr::PrefixOpn {
                opr,
                opr_token_idx,
                opd,
            } => {
                let opd_ty = self.infer_new(*opd, None);
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
                dot_token_idx,
                ident_token,
                implicit_arguments,
                lpar_token_idx,
                arguments,
                rpar_token_idx,
            } => todo!(),
            Expr::TemplateInstantiation {
                template,
                implicit_arguments,
            } => todo!(),
            Expr::Application { function, argument } => {
                let function_expr = &self[*function];
                match function_expr {
                    Expr::NewBoxList {
                        caller: None,
                        lbox_token_idx,
                        items,
                        rbox_token_idx,
                    } => {
                        match items.len() {
                            0 => {
                                let argument_ty = self.infer_new(*argument, None);
                                // check this is type
                                argument_ty.ok_or(
                                    DerivedExprTypeError::ApplicationArgumentTypeNotInferred.into(),
                                )
                            }
                            1 => {
                                let arg0_ty = self.infer_new(items.start(), None);
                                match arg0_ty {
                                    Some(_) => todo!(),
                                    None => Err(
                                        DerivedExprTypeError::BoxListApplicationFirstArgumentError
                                            .into(),
                                    ),
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
                        let function_ty = self.infer_new(*function, None);
                        todo!()
                    }
                }
            }
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
            Expr::Block { stmts } => todo!(),
            Expr::Err(_) => Err(DerivedExprTypeError::ExprError.into()),
        }
    }
}
