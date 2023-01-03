use husky_print_utils::p;
use parsec::ParseContext;

use super::*;

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(crate) fn accept_token(&mut self, token: ResolvedToken) {
        match token {
            ResolvedToken::Atom(atom) => self.accept_atom(atom),
            ResolvedToken::BinaryPunctuation(token_idx, opr) => {
                self.accept_binary_opr(opr, token_idx)
            }
            ResolvedToken::PrefixPunctuation(token_idx, opr) => {
                self.accept_prefix_opr(opr, token_idx)
            }
            ResolvedToken::SuffixPunctuation(token_idx, opr) => {
                self.accept_suffix_opr(opr, token_idx)
            }
            ResolvedToken::Bra(token_idx, bra) => self.accept_list_start(bra, token_idx),
            ResolvedToken::Ket(token_idx, ket) => self.accept_list_end(ket, token_idx),
            ResolvedToken::Dot(token_idx) => self.accept_dot_opr(token_idx),
            ResolvedToken::ListItem(token_idx) => self.accept_list_item(token_idx),
        }
    }

    pub(crate) fn accept_list_end(&mut self, ket: Bracket, ket_token_idx: TokenIdx) {
        self.reduce(Precedence::ListItem);
        match self.take_last_unfinished_expr().unwrap() {
            UnfinishedExpr::List {
                opr,
                bra,
                bra_token_idx,
                mut items,
            } => {
                if bra != ket {
                    todo!()
                }
                self.replace_top_expr(|this, finished_expr| {
                    if let Some(expr) = finished_expr {
                        items.push(expr)
                    }
                    let items = this.sheet.alloc_expr_batch(items);
                    match opr {
                        UnfinishedListOpr::NewTuple => Expr::NewTuple {
                            lpar_token_idx: bra_token_idx,
                            items,
                            rpar_token_idx: ket_token_idx,
                        }
                        .into(),
                        UnfinishedListOpr::NewVec => Expr::NewList {
                            lbox_token_idx: bra_token_idx,
                            items,
                            rbox_token_idx: ket_token_idx,
                        }
                        .into(),
                        UnfinishedListOpr::NewLambdaHead => todo!(),
                        UnfinishedListOpr::FunctionCall { .. } => todo!(),
                        UnfinishedListOpr::MethodInstantiation {} => todo!(),
                        UnfinishedListOpr::MethodCall {
                            this_expr,
                            implicit_arguments,
                        } => Expr::MethodCall {
                            this_expr,
                            implicit_arguments,
                            arguments: items,
                            lpar_token_idx: bra_token_idx,
                            rpar_token_idx: ket_token_idx,
                        }
                        .into(),
                        UnfinishedListOpr::TemplateInstantiation { template } => todo!(),
                    }
                })
            }
            _ => todo!(),
        }
    }

    fn accept_atom(&mut self, atom: Expr) {
        self.set_top_expr(atom.into())
    }

    fn accept_prefix_opr(&mut self, prefix: PrefixPunctuation, prefix_token_idx: TokenIdx) {
        self.set_top_expr(
            UnfinishedExpr::Prefix {
                punctuation: prefix,
                punctuation_token_idx: prefix_token_idx,
            }
            .into(),
        )
    }

    fn accept_suffix_opr(
        &mut self,
        punctuation: SuffixPunctuation,
        punctuation_token_idx: TokenIdx,
    ) {
        self.replace_top_expr(|this, top_expr| match top_expr {
            Some(expr) => Expr::SuffixOpn {
                opd: this.sheet.alloc_expr(expr),
                punctuation,
                punctuation_token_idx,
            }
            .into(),
            None => todo!(),
        })
    }

    fn accept_dot_opr(&mut self, dot_token_idx: TokenIdx) {
        self.replace_top_expr(|this, finished_expr| match finished_expr {
            Some(this_expr) => {
                let this_expr = this.sheet.alloc_expr(this_expr);
                match this.parse::<IdentifierToken>() {
                    Ok(Some(ident_token)) => match this.parse::<LeftParenthesisToken>() {
                        Ok(Some(lpar)) => UnfinishedExpr::List {
                            opr: UnfinishedListOpr::MethodCall {
                                this_expr,
                                implicit_arguments: None,
                            },
                            bra: Bracket::Par,
                            bra_token_idx: lpar.token_idx(),
                            items: vec![],
                        }
                        .into(),
                        Ok(None) => match this.parse::<LeftAngleBracketToken>() {
                            Ok(Some(langle)) => todo!(),
                            Ok(None) => Expr::Field {
                                this_expr,
                                dot_token_idx,
                                ident_token,
                            }
                            .into(),
                            Err(_) => todo!(),
                        },
                        Err(_) => todo!(),
                    },
                    _ => todo!(),
                }
            }
            None => todo!(),
        })
    }

    fn accept_list_item(&mut self, comma_token_idx: TokenIdx) {
        let item =
            self.take_finished_expr()
                .unwrap_or(Expr::Err(ExprError::MissingItemBeforeComma {
                    comma_token_idx,
                }));
        match self.last_unfinished_expr_mut() {
            Some(expr) => match expr {
                UnfinishedExpr::List {
                    opr,
                    bra,
                    bra_token_idx,
                    items,
                } => items.push(item),
                _ => unreachable!(),
            },
            None => unreachable!(),
        }
    }

    pub(crate) fn accept_binary_opr(
        &mut self,
        binary: BinaryPunctuation,
        binary_token_idx: TokenIdx,
    ) {
        let lopd = self
            .take_finished_expr()
            .unwrap_or(Expr::Err(ExprError::NoLeftOperandForBinaryOperator));
        let unfinished_expr = UnfinishedExpr::Binary {
            lopd,
            punctuation: binary,
            punctuation_token_idx: binary_token_idx,
        };
        self.reduce(unfinished_expr.precedence());
        self.set_top_expr(unfinished_expr.into())
    }

    pub(super) fn accept_list_start(&mut self, bra: Bracket, bra_token_idx: TokenIdx) {
        self.replace_top_expr(|this, finished_expr| match finished_expr {
            Some(expr) => {
                let expr = this.sheet.alloc_expr(expr);
                match bra {
                    Bracket::Par => UnfinishedExpr::List {
                        opr: UnfinishedListOpr::FunctionCall { function: expr },
                        bra,
                        bra_token_idx,
                        items: vec![],
                    }
                    .into(),
                    Bracket::Box => todo!(),
                    Bracket::Angle => UnfinishedExpr::List {
                        opr: UnfinishedListOpr::TemplateInstantiation { template: expr },
                        bra,
                        bra_token_idx,
                        items: vec![],
                    }
                    .into(),
                    Bracket::Curl => todo!(),
                    Bracket::Vertical => todo!(),
                }
            }
            None => match bra {
                Bracket::Par => UnfinishedExpr::List {
                    opr: UnfinishedListOpr::NewTuple,
                    bra,
                    bra_token_idx,
                    items: vec![],
                }
                .into(),
                Bracket::Box => UnfinishedExpr::List {
                    opr: UnfinishedListOpr::NewVec,
                    bra,
                    bra_token_idx,
                    items: vec![],
                }
                .into(),
                Bracket::Angle => todo!(),
                Bracket::Curl => todo!(),
                Bracket::Vertical => todo!(),
            },
        })
        // let opr = match bra {
        //     Bracket::Par => match self.top_expr() {
        //         TopExprRef::Finished(_) => UnfinishedListOpr::FunctionCall,
        //         _ => UnfinishedListOpr::NewTuple,
        //     },
        //     Bracket::Box => UnfinishedListOpr::NewVec,
        //     Bracket::Angle => match self.top_expr() {
        //         TopExprRef::Unfinished(_) => todo!(),
        //         TopExprRef::Finished(_) => todo!(),
        //         TopExprRef::None => todo!(),
        //     },
        //     Bracket::Curl => todo!(),
        //     Bracket::Vertical => todo!(),
        // };
        // self.set_top_expr(

        // );
    }
}
