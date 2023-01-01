use super::*;

#[derive(Default, Debug)]
pub(crate) struct ExprParserStack {
    unfinished_exprs: Vec<(UnfinishedExpr, Precedence)>,
    top_expr: Option<Expr>,
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
        match self {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath(_) => todo!(),
            Expr::Variable { .. } => BaseEntityPath::None,
            Expr::Uncertain(_) => todo!(),
            Expr::Unrecognized(_) => BaseEntityPath::Uncertain,
            Expr::Opn { opn, opds } => match opn {
                Opn::Binary(_) => todo!(),
                Opn::Prefix(_) | Opn::Suffix { .. } => BaseEntityPath::None,
                Opn::CurlBracketed => todo!(),
                Opn::List { opr, .. } => match opr {
                    ListOpr::NewTuple => todo!(),
                    ListOpr::NewVec => BaseEntityPath::None,
                    ListOpr::NewDict => todo!(),
                    ListOpr::FunctionCall => todo!(),
                    ListOpr::Index => todo!(),
                    ListOpr::ModuloIndex => todo!(),
                    ListOpr::StructInit => todo!(),
                    ListOpr::MethodCall { ranged_ident } => todo!(),
                },
                Opn::Field(_) => todo!(),
                Opn::Abstraction => todo!(),
                Opn::Application => todo!(),
            },
            Expr::Bracketed(_) => todo!(),
            Expr::Err(_) => todo!(),
        }
    }
}

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(super) fn top_expr(&self) -> Option<&Expr> {
        self.stack.top_expr.as_ref()
    }

    pub(super) fn finish_batch(&mut self) -> Option<ExprIdx> {
        core::mem::take(&mut self.stack.top_expr).map(|expr| self.sheet.alloc_expr(expr))
    }

    pub(super) fn push_expr(&mut self, expr: Expr) {
        if let Some(expr) = self.take_top_expr() {
            self.push_unfinished_expr(UnfinishedExpr::Application { function: expr });
        }
        self.stack.top_expr = Some(expr)
    }

    pub(super) fn push_unfinished_expr(&mut self, unfinished_expr: UnfinishedExpr) {
        assert!(self.stack.top_expr.is_none());
        let precedence = unfinished_expr.precedence();
        self.stack
            .unfinished_exprs
            .push((unfinished_expr, precedence))
    }

    pub(super) fn last_unfinished_expr(&self) -> Option<&UnfinishedExpr> {
        self.stack.unfinished_exprs.last().map(|(opr, _)| opr)
    }

    pub(super) fn set_top_expr(&mut self, expr: Expr) {
        assert!(self.stack.top_expr.is_none());
        self.stack.top_expr = Some(expr)
    }

    pub(super) fn take_top_expr(&mut self) -> Option<Expr> {
        std::mem::take(&mut self.stack.top_expr)
    }

    pub(super) fn accept_prefix_opr(
        &mut self,
        prefix: PrefixPunctuation,
        prefix_token_idx: TokenIdx,
    ) {
        match self.take_top_expr() {
            Some(_) => todo!(),
            None => self.push_unfinished_expr(UnfinishedExpr::Prefix {
                prefix,
                prefix_token_idx,
            }),
        }
    }

    pub(super) fn take_top_unfinished_expr(&mut self) -> Option<UnfinishedExpr> {
        self.stack.unfinished_exprs.pop().map(|(expr, _)| expr)
    }

    pub(super) fn reduce(&mut self, next_precedence: Precedence) {
        while let Some(prev_precedence) = self.stack.prev_unfinished_expr_precedence() {
            if prev_precedence < next_precedence {
                break;
            }
            match self.stack.unfinished_exprs.pop().unwrap().0 {
                UnfinishedExpr::Binary {
                    lopd,
                    binary,
                    binary_token_idx,
                } => match self.take_top_expr() {
                    Some(ropd) => {
                        self.stack.top_expr = Some(Expr::Opn {
                            opn: Opn::Binary(binary),
                            opds: self.sheet.alloc_expr_batch([lopd, ropd]),
                        })
                    }
                    None => {
                        let lopd = self.sheet.alloc_expr(lopd);
                        self.stack.top_expr =
                            Some(Expr::Err(ExprError::NoRightOperandForBinaryOperator {
                                lopd,
                                binary,
                                binary_token_idx,
                            }))
                    }
                },
                UnfinishedExpr::Application { function } => {
                    let argument = self.take_top_expr().unwrap();
                    self.stack.top_expr = Some(Expr::Opn {
                        opn: Opn::Application,
                        opds: self.sheet.alloc_expr_batch([function, argument]),
                    })
                }
                UnfinishedExpr::Prefix {
                    prefix,
                    prefix_token_idx,
                } => match self.take_top_expr() {
                    Some(opd) => {
                        self.stack.top_expr = Some(Expr::Opn {
                            opn: Opn::Prefix(prefix),
                            opds: self.sheet.alloc_expr_batch([opd]),
                        })
                    }
                    None => {
                        self.stack.top_expr =
                            Some(Expr::Err(ExprError::NoOperandForPrefixOperator {
                                prefix,
                                prefix_token_idx,
                            }))
                    }
                },
                UnfinishedExpr::ListItem {
                    separator_token_idx,
                } => todo!(),
                UnfinishedExpr::List { .. } => todo!(),
                UnfinishedExpr::LambdaHead { inputs, start } => todo!(),
                UnfinishedExpr::Dot { dot_token_idx } => todo!(),
            }
        }
    }

    pub(super) fn replace_top_expr(
        &mut self,
        f: impl FnOnce(Option<Expr>, &mut ExprSheet) -> Expr,
    ) {
        let top_expr = self.take_top_expr();
        let new_expr = f(top_expr, &mut self.sheet);
        self.stack.top_expr = Some(new_expr)
    }
}
