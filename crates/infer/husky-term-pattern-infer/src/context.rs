use crate::*;
use husky_entity_path::EntityPath;
use husky_expr::{Expr, ExprIdx};
use husky_term::{Term, TermContext, TermMenu};
use husky_term_pattern::TermPatternItd;
use husky_word::WordDb;

pub(crate) struct TermPatternInferContext<'a> {
    db: &'a dyn TermPatternInferDb,
    expr_idx: ExprIdx,
    term_menu: &'a TermMenu,
}

impl<'a> TermPatternInferContext<'a> {
    pub(crate) fn new(
        db: &'a dyn TermPatternInferDb,
        expr_idx: ExprIdx,
        term_menu: &'a TermMenu,
    ) -> Self {
        Self {
            db,
            expr_idx,
            term_menu,
        }
    }

    pub(crate) fn write_inference(self, sheet: &mut TermPatternInferSheet) {
        if let Some(subexprs) = self.subexprs() {
            for subexpr in subexprs {
                self.subexpr_context(subexpr).write_inference(sheet)
            }
        }
        let result = self.infer(sheet);
        sheet.insert_result(self.expr_idx, result)
    }

    pub(crate) fn infer(
        &self,
        sheet: &mut TermPatternInferSheet,
    ) -> ExprTermPatternInferRawResults {
        todo!()
        // match self.expr() {
        //     Expr::Literal(literal) => self.infer_literal(todo!(), sheet),
        //     Expr::EntityPath(_) => todo!(),
        //     Expr::Variable {
        //         token_idx,
        //         variable_idx,
        //     } => todo!(),
        //     // Expr::Symbol(symbol) => match symbol {
        //     //     Symbol::Entity(_) => todo!(),
        //     //     Symbol::Variable(_) => todo!(),
        //     //     Symbol::Lifetime(_) => todo!(),
        //     //     Symbol::Label(_) => todo!(),
        //     // },
        //     Expr::Unrecognized(ident) => {
        //         let error = self.error_original(OriginalTermPatternInferError::IdentUnrecognized {
        //             ident: self.db.dt_ident(*ident).to_owned(),
        //         });
        //         ExprTermPatternInferRawResults {
        //             const_expr: Err(error.clone()),
        //             ty: self
        //                 .err_derived(DerivedTermPatternInferError::TermPatternInferError(error)),
        //         }
        //     }
        //     Expr::Uncertain(_) => todo!(),
        //     Expr::Opn {
        //         opn: ref opn_variant,
        //         ref opds,
        //     } => self.infer_opn_ty(opn_variant, opds, sheet),
        //     Expr::Bracketed(_) => todo!(),
        //     Expr::Err(_) => todo!(),
        //     Expr::MethodCall {
        //         this_expr,
        //         arguments,
        //         lpar_token_idx,
        //         rpar_token_idx,
        //     } => todo!(),
        // }
    }

    // fn infer_opn_ty(
    //     &self,
    //     opn_variant: &Opn,
    //     opds: &ExprIdxRange,
    //     sheet: &mut TermPatternInferSheet,
    // ) -> ExprTermPatternInferRawResults {
    //     todo!()
    //     // match opn_variant {
    //     //     Opn::List { .. } => todo!(),
    //     //     Opn::Field { .. } => todo!(),
    //     //     Opn::Abstraction => todo!(),
    //     //     Opn::Application => todo!(),
    //     //     Opn::Method { ident_token } => todo!(),
    //     // }
    // }

    fn infer_binary_opn(
        &self,
        opr: BinaryOpr,
        opds: &ExprIdxRange,
        sheet: &mut TermPatternInferSheet,
    ) -> ExprTermPatternInferRawResults {
        match opr {
            BinaryOpr::Assign(_) => todo!(),
            BinaryOpr::PureClosed(_) => ExprTermPatternInferRawResults {
                // todo: if both operands are constant, make this constexpr?
                const_expr: Ok(None),
                ty: self.expr_ty_result(sheet, opds.start()),
            },
            BinaryOpr::Comparison(_) => todo!(),
            BinaryOpr::ShortcuitLogic(_) => todo!(),
            BinaryOpr::ScopeResolution => todo!(),
            BinaryOpr::Curry => todo!(),
            BinaryOpr::As => todo!(),
            BinaryOpr::Is => todo!(),
        }
    }

    fn infer_literal(
        &self,
        literal: &Literal,
        sheet: &mut TermPatternInferSheet,
    ) -> ExprTermPatternInferRawResults {
        let _term_menu = self.term_menu();
        match literal {
            Literal::Unit => todo!(),
            Literal::Integer(_) => {
                let term = sheet.it_unresolved(UnresolvedTerm::IntegerLiteral(self.expr_idx()));
                let ty = sheet.it_unresolved(UnresolvedTerm::IntegerType(term));
                ExprTermPatternInferRawResults {
                    const_expr: Ok(Some(ConstExprPattern {
                        term: term.into(),
                        opt_substitution_ctx: None,
                    })),
                    ty: Ok(ty.into()),
                }
            }
            // LiteralToken::I32(_i) => ExprTermPatternInferRawResults {
            //     const_expr: Ok(Some(ConstExprPattern {
            //         term: TermPatternItd::Resolved(todo!()),
            //         opt_substitution_ctx: None,
            //     })),
            //     ty: Ok(self.term_menu.i32().into()),
            // },
            // LiteralToken::I64(_i) => ExprTermPatternInferRawResults {
            //     const_expr: Ok(Some(ConstExprPattern {
            //         term: TermPatternItd::Resolved(todo!()),
            //         opt_substitution_ctx: None,
            //     })),
            //     ty: Ok(self.term_menu.i32().into()),
            // },
            Literal::Float(_) => {
                let term = sheet.it_unresolved(UnresolvedTerm::FloatLiteral(self.expr_idx()));
                let ty = sheet.it_unresolved(UnresolvedTerm::FloatType(term));
                ExprTermPatternInferRawResults {
                    const_expr: Ok(Some(ConstExprPattern {
                        term: term.into(),
                        opt_substitution_ctx: None,
                    })),
                    ty: Ok(ty.into()),
                }
            }
            Literal::Bool(_) => todo!(),
            Literal::String(_) => todo!(),
            Literal::Char(_) => todo!(),
            Literal::TupleIndex(_) => todo!(),
        }
    }

    fn subexpr_context(&self, subexpr: ExprIdx) -> Self {
        Self {
            db: self.db,
            expr_idx: subexpr,
            term_menu: self.term_menu,
        }
    }

    fn subexprs(&self) -> Option<ExprIdxRange> {
        todo!()
        // match self.expr() {
        //     Expr::Opn { ref opds, .. } => Some(opds.clone()),
        //     Expr::Bracketed(_) => todo!(),
        //     Expr::Err(_) => todo!(),
        //     Expr::Literal(_) => todo!(),
        //     Expr::EntityPath(_) => todo!(),
        //     Expr::Variable {
        //         token_idx,
        //         variable_idx,
        //     } => todo!(),
        //     Expr::Uncertain(_) => todo!(),
        //     Expr::Unrecognized(_) => todo!(),
        //     Expr::MethodCall {
        //         this_expr,
        //         arguments,
        //         lpar_token_idx,
        //         rpar_token_idx,
        //     } => todo!(),
        // }
    }

    pub(crate) fn expr_idx(&self) -> ExprIdx {
        self.expr_idx
    }

    pub(crate) fn expr(&self) -> &'a Expr {
        todo!()
        // &self.expr_sheet[self.expr_idx]
    }

    pub(crate) fn term_menu(&self) -> &'a TermMenu {
        self.term_menu
    }

    #[inline(always)]
    fn term_ctx(&self) -> TermContext<'a> {
        TermContext::new(self.db.upcast(), self.term_menu)
    }

    pub(crate) fn entity_path_term(&self, path: EntityPath) -> TermPatternInferResult<Term> {
        self.map_original(self.term_ctx().entity_path_term(path))
    }
}
