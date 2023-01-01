use husky_print_utils::p;

use super::*;

#[derive(Default, Debug)]
pub(crate) struct ExprParserStack {
    unfinished_exprs: Vec<(UnfinishedExpr, Precedence)>,
    finished_expr: Option<Expr>,
}

pub(super) enum TopExpr {
    Unfinished(UnfinishedExpr),
    Finished(Expr),
}

pub(super) enum TopExprRef<'a> {
    Unfinished(&'a UnfinishedExpr),
    Finished(&'a Expr),
    None,
}

impl From<Expr> for TopExpr {
    fn from(v: Expr) -> Self {
        Self::Finished(v)
    }
}

impl From<UnfinishedExpr> for TopExpr {
    fn from(v: UnfinishedExpr) -> Self {
        Self::Unfinished(v)
    }
}

impl ExprParserStack {
    pub(super) fn prev_unfinished_expr_precedence(&self) -> Option<Precedence> {
        self.unfinished_exprs
            .last()
            .map(|(_, precedence)| *precedence)
    }
}

impl Expr {
    pub fn base_entity_path(&self) -> BaseEntityPath {
        todo!()
        // match self {
        //     Expr::Literal(_) => todo!(),
        //     Expr::EntityPath(_) => todo!(),
        //     Expr::Variable { .. } => BaseEntityPath::None,
        //     Expr::Uncertain(_) => todo!(),
        //     Expr::Unrecognized(_) => BaseEntityPath::Uncertain,
        //     Expr::Opn { opn, opds } => match opn {
        //         Opn::Binary(_) => todo!(),
        //         Opn::CurlBracketed => todo!(),
        //         Opn::List { opr, .. } => match opr {
        //             ListOpr::NewTuple => todo!(),
        //             ListOpr::NewVec => BaseEntityPath::None,
        //             ListOpr::NewDict => todo!(),
        //             ListOpr::FunctionCall => todo!(),
        //             ListOpr::Index => todo!(),
        //             ListOpr::ModuloIndex => todo!(),
        //             ListOpr::StructInit => todo!(),
        //             ListOpr::MethodCall => todo!(),
        //             ListOpr::NewLambdaHead => todo!(),
        //             ListOpr::ImplicitParameterList => todo!(),
        //         },
        //         Opn::Abstraction => todo!(),
        //         Opn::Application => todo!(),
        //         Opn::Method { .. } | Opn::Field { .. } | Opn::Prefix(_) | Opn::Suffix { .. } => {
        //             BaseEntityPath::None
        //         }
        //     },
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
}

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(super) fn finished_expr(&self) -> Option<&Expr> {
        self.stack.finished_expr.as_ref()
    }

    pub(super) fn take_last_unfinished_expr(&mut self) -> Option<UnfinishedExpr> {
        self.stack.unfinished_exprs.pop().map(|(expr, _)| expr)
    }

    fn push_unfinished_expr(&mut self, unfinished_expr: UnfinishedExpr) {
        assert!(self.stack.finished_expr.is_none());
        let precedence = unfinished_expr.precedence();
        self.stack
            .unfinished_exprs
            .push((unfinished_expr, precedence))
    }

    pub(super) fn last_unfinished_expr(&self) -> Option<&UnfinishedExpr> {
        self.stack.unfinished_exprs.last().map(|(opr, _)| opr)
    }

    pub(super) fn last_unfinished_expr_mut(&mut self) -> Option<&mut UnfinishedExpr> {
        self.stack.unfinished_exprs.last_mut().map(|(opr, _)| opr)
    }

    pub(super) fn set_top_expr(&mut self, top_expr: TopExpr) {
        if let Some(function) = self.take_finished_expr() {
            self.push_unfinished_expr(UnfinishedExpr::Application { function });
        }
        match top_expr {
            TopExpr::Unfinished(unfinished_expr) => self.push_unfinished_expr(unfinished_expr),
            TopExpr::Finished(finished_expr) => self.stack.finished_expr = Some(finished_expr),
        }
    }

    pub(super) fn top_expr<'d>(&'d self) -> TopExprRef<'d> {
        if let Some(ref finished_expr) = self.stack.finished_expr {
            TopExprRef::Finished(finished_expr)
        } else if let Some((unfinished_expr, _)) = self.stack.unfinished_exprs.last() {
            TopExprRef::Unfinished(unfinished_expr)
        } else {
            TopExprRef::None
        }
    }

    pub(super) fn take_finished_expr(&mut self) -> Option<Expr> {
        std::mem::take(&mut self.stack.finished_expr)
    }

    pub(super) fn reduce(&mut self, next_precedence: Precedence) {
        while let Some(prev_precedence) = self.stack.prev_unfinished_expr_precedence() {
            if prev_precedence < next_precedence {
                break;
            }
            match self.stack.unfinished_exprs.pop().unwrap().0 {
                UnfinishedExpr::Binary {
                    lopd,
                    punctuation,
                    punctuation_token_idx,
                } => match self.take_finished_expr() {
                    Some(ropd) => {
                        self.stack.finished_expr = Some(Expr::BinaryOpn {
                            lopd: self.sheet.alloc_expr(lopd),
                            punctuation,
                            punctuation_token_idx,
                            ropd: self.sheet.alloc_expr(ropd),
                        })
                    }
                    None => {
                        let lopd = self.sheet.alloc_expr(lopd);
                        self.stack.finished_expr =
                            Some(Expr::Err(ExprError::NoRightOperandForBinaryOperator {
                                lopd,
                                punctuation,
                                punctuation_token_idx,
                            }))
                    }
                },
                UnfinishedExpr::Application { function } => {
                    let argument = self.take_finished_expr().unwrap();
                    self.stack.finished_expr = Some(Expr::Opn {
                        opn: Opn::Application,
                        opds: self.sheet.alloc_expr_batch([function, argument]),
                    })
                }
                UnfinishedExpr::Prefix {
                    punctuation,
                    punctuation_token_idx,
                } => match self.take_finished_expr() {
                    Some(opd) => {
                        self.stack.finished_expr = Some(Expr::PrefixOpn {
                            punctuation,
                            punctuation_token_idx,
                            opd: self.sheet.alloc_expr(opd),
                        })
                    }
                    None => {
                        self.stack.finished_expr =
                            Some(Expr::Err(ExprError::NoOperandForPrefixOperator {
                                prefix: punctuation,
                                prefix_token_idx: punctuation_token_idx,
                            }))
                    }
                },
                UnfinishedExpr::ListItem {
                    separator_token_idx,
                } => todo!(),
                UnfinishedExpr::List { .. } => todo!(),
                UnfinishedExpr::LambdaHead { inputs, start } => todo!(),
                UnfinishedExpr::Method {
                    this_expr,
                    ident_token,
                } => match self.take_finished_expr() {
                    Some(Expr::Opn {
                        opn:
                            Opn::List {
                                opr: ListOpr::ImplicitParameterList,
                                ..
                            },
                        ..
                    }) => todo!(),
                    Some(Expr::Opn {
                        opn:
                            Opn::List {
                                opr: ListOpr::MethodCall,
                                bracket,
                                bra_token_idx: lpar_token_idx,
                                ket_token_idx: rpar_token_idx,
                            },
                        opds: arguments,
                    }) => {
                        self.stack.finished_expr = Some(Expr::MethodCall {
                            this_expr: self.sheet.alloc_expr(this_expr),
                            arguments,
                            lpar_token_idx,
                            rpar_token_idx,
                        })
                    }
                    Some(_) => todo!(),
                    None => todo!(),
                },
            }
        }
    }

    pub(super) fn replace_finished_expr(
        &mut self,
        f: impl FnOnce(&mut Self, Option<Expr>) -> TopExpr,
    ) {
        let finished_expr = self.take_finished_expr();
        let top_expr = f(self, finished_expr);
        self.set_top_expr(top_expr)
    }

    pub(super) fn finish_batch(&mut self) -> Option<ExprIdx> {
        core::mem::take(&mut self.stack.finished_expr).map(|expr| self.sheet.alloc_expr(expr))
    }
}
