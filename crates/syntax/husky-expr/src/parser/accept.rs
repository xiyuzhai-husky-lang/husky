use husky_print_utils::p;
use parsec::ParseContext;

use super::*;

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn accept_token(&mut self, token: ResolvedToken) {
        match token {
            ResolvedToken::AtomicExpr(atom) => self.accept_atom(atom),
            ResolvedToken::BinaryOpr(token_idx, opr) => self.accept_binary_opr(opr, token_idx),
            ResolvedToken::PrefixOpr(token_idx, opr) => self.accept_prefix_opr(opr, token_idx),
            ResolvedToken::SuffixOpr(token_idx, opr) => self.accept_suffix_opr(opr, token_idx),
            ResolvedToken::Bra(token_idx, bra) => self.accept_list_start(bra, token_idx),
            ResolvedToken::Ket(token_idx, ket) => self.accept_list_end(ket, token_idx),
            ResolvedToken::Dot(token_idx) => self.accept_dot_opr(token_idx),
            ResolvedToken::ListItem(token_idx) => self.accept_list_item(token_idx),
            ResolvedToken::Be(token_idx) => self.accept_be_pattern(token_idx),
            ResolvedToken::ColonRightAfterLBox(colon_token_idx) => {
                self.accept_colon_right_after_lbox(colon_token_idx)
            }
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
                commas,
            } => {
                if bra != ket {
                    todo!()
                }
                self.replace_top_expr(|this, finished_expr| {
                    if let Some(expr) = finished_expr {
                        items.push(expr)
                    }
                    let items = this.alloc_expr_batch(items);
                    match opr {
                        UnfinishedListOpr::NewTuple => match (items.len(), commas.len()) {
                            (1, 0) => Expr::Bracketed {
                                lpar_token_idx: bra_token_idx,
                                item: items.start(),
                                rpar_token_idx: ket_token_idx,
                            },
                            _ => Expr::NewTuple {
                                lpar_token_idx: bra_token_idx,
                                items,
                                commas,
                                rpar_token_idx: ket_token_idx,
                            },
                        }
                        .into(),
                        UnfinishedListOpr::Index { owner } => Expr::IndexOrCompositionWithList {
                            owner,
                            lbox_token_idx: bra_token_idx,
                            items,
                            rbox_token_idx: ket_token_idx,
                        }
                        .into(),
                        UnfinishedListOpr::BoxList => Expr::List {
                            lbox_token_idx: bra_token_idx,
                            items,
                            rbox_token_idx: ket_token_idx,
                        }
                        .into(),
                        UnfinishedListOpr::BoxColonList { colon_token_idx } => Expr::BoxColonList {
                            lbox_token_idx: bra_token_idx,
                            colon_token_idx,
                            items,
                            rbox_token_idx: ket_token_idx,
                        }
                        .into(),
                        UnfinishedListOpr::NewLambdaHead => todo!(),
                        UnfinishedListOpr::FunctionCall { function } => {
                            // ad hoc
                            let implicit_arguments: Option<ImplicitArgumentList> = None;
                            Expr::ExplicitApplicationOrRitchieCall {
                                function,
                                implicit_arguments,
                                lpar_token_idx: bra_token_idx,
                                items,
                                commas,
                                rpar_token_idx: ket_token_idx,
                            }
                            .into()
                        }
                        UnfinishedListOpr::MethodInstantiation { .. } => todo!(),
                        UnfinishedListOpr::MethodCall {
                            self_expr,
                            dot_token_idx,
                            ident_token,
                            implicit_arguments,
                        } => Expr::MethodCall {
                            self_argument: self_expr,
                            dot_token_idx,
                            ident_token,
                            implicit_arguments,
                            lpar_token_idx: bra_token_idx,
                            nonself_arguments: items,
                            rpar_token_idx: ket_token_idx,
                        }
                        .into(),
                        UnfinishedListOpr::TemplateInstantiation { template } => {
                            Expr::TemplateInstantiation {
                                template,
                                implicit_arguments: ImplicitArgumentList::new(
                                    bra_token_idx,
                                    items,
                                    commas,
                                    ket_token_idx,
                                ),
                            }
                            .into()
                        }
                        UnfinishedListOpr::FunctionInstantiation {} => todo!(),
                    }
                })
            }
            _ => todo!(),
        }
    }

    fn accept_atom(&mut self, atom: Expr) {
        self.set_top_expr(atom.into())
    }

    fn accept_prefix_opr(&mut self, prefix: PrefixOpr, prefix_token_idx: TokenIdx) {
        self.set_top_expr(
            UnfinishedExpr::Prefix {
                punctuation: prefix,
                punctuation_token_idx: prefix_token_idx,
            }
            .into(),
        )
    }

    fn accept_suffix_opr(&mut self, punctuation: SuffixOpr, punctuation_token_idx: TokenIdx) {
        self.replace_top_expr(|this, top_expr| match top_expr {
            Some(expr) => Expr::Suffix {
                opd: this.alloc_expr(expr),
                opr: punctuation,
                opr_token_idx: punctuation_token_idx,
            }
            .into(),
            None => todo!(),
        })
    }

    fn accept_dot_opr(&mut self, dot_token_idx: TokenIdx) {
        self.replace_top_expr(|this, finished_expr| match finished_expr {
            Some(self_expr) => {
                let self_expr = this.alloc_expr(self_expr);
                match this.parse::<IdentToken>() {
                    Ok(Some(ident_token)) => match this.parse::<LeftParenthesisToken>() {
                        Ok(Some(lpar)) => UnfinishedExpr::List {
                            opr: UnfinishedListOpr::MethodCall {
                                self_expr,
                                dot_token_idx,
                                ident_token,
                                implicit_arguments: None,
                            },
                            bra: Bracket::Par,
                            bra_token_idx: lpar.token_idx(),
                            items: vec![],
                            commas: vec![],
                        }
                        .into(),
                        Ok(None) => match this.parse::<ColonColonLeftAngleBracketToken>() {
                            Ok(Some(langle)) => UnfinishedExpr::List {
                                opr: UnfinishedListOpr::MethodInstantiation {
                                    self_expr,
                                    dot_token_idx,
                                    ident_token,
                                },
                                bra: Bracket::Angle,
                                bra_token_idx: langle.token_idx(),
                                items: vec![],
                                commas: vec![],
                            }
                            .into(),
                            Ok(None) => Expr::Field {
                                owner: self_expr,
                                dot_token_idx,
                                ident_token,
                            }
                            .into(),
                            Err(_) => todo!(),
                        },
                        Err(e) => {
                            p!(e);
                            todo!()
                        }
                    },
                    _ => Expr::Err(
                        OriginalExprError::ExpectedIdentAfterDot {
                            self_expr,
                            dot_token_idx,
                        }
                        .into(),
                    )
                    .into(),
                }
            }
            None => {
                Expr::Err(OriginalExprError::ExpectedExprBeforeDot { dot_token_idx }.into()).into()
            }
        })
    }

    fn accept_list_item(&mut self, comma_token_idx: TokenIdx) {
        let item = self.take_finished_expr().unwrap_or(Expr::Err(
            OriginalExprError::ExpectItemBeforeComma { comma_token_idx }.into(),
        ));
        match self.last_unfinished_expr_mut() {
            Some(expr) => match expr {
                UnfinishedExpr::List {
                    opr,
                    bra,
                    bra_token_idx,
                    items,
                    commas,
                } => {
                    items.push(item);
                    commas.push(comma_token_idx)
                }
                _ => unreachable!(),
            },
            None => unreachable!(),
        }
    }

    fn accept_be_pattern(&mut self, be_token_idx: TokenIdx) {
        self.reduce(Precedence::Be);
        let src = self.take_finished_expr().unwrap_or(Expr::Err(
            OriginalExprError::ExpectItemBeforeBe { be_token_idx }.into(),
        ));
        let src = self.alloc_expr(src);
        let expr = Expr::Be {
            src,
            be_token_idx,
            target: self.parse_expected(OriginalExprError::ExpectedPatternExprAfterBe),
        };
        self.set_top_expr(expr.into())
    }

    pub(crate) fn accept_binary_opr(&mut self, binary: BinaryOpr, binary_token_idx: TokenIdx) {
        self.reduce(binary.into());
        let lopd = self.take_finished_expr().unwrap_or(Expr::Err(
            OriginalExprError::NoLeftOperandForBinaryOperator { binary_token_idx }.into(),
        ));
        let unfinished_expr = UnfinishedExpr::Binary {
            lopd,
            punctuation: binary,
            punctuation_token_idx: binary_token_idx,
        };
        self.set_top_expr(unfinished_expr.into())
    }

    fn accept_colon_right_after_lbox(&mut self, colon_token_idx: TokenIdx) {
        #[cfg(test)]
        assert!(self.finished_expr().is_none());
        let unfinished_expr = self.take_last_unfinished_expr().unwrap();
        match unfinished_expr {
            UnfinishedExpr::List {
                opr: UnfinishedListOpr::BoxList,
                bra,
                bra_token_idx,
                items,
                commas,
            } => {
                assert!(items.is_empty());
                self.set_top_expr(
                    UnfinishedExpr::List {
                        opr: UnfinishedListOpr::BoxColonList { colon_token_idx },
                        bra,
                        bra_token_idx,
                        items,
                        commas,
                    }
                    .into(),
                )
            }
            _ => unreachable!(),
        }
    }

    pub(super) fn accept_list_start(&mut self, bra: Bracket, bra_token_idx: TokenIdx) {
        self.replace_top_expr(|this, finished_expr| {
            let finished_expr = finished_expr.map(|expr| this.alloc_expr(expr));
            match bra {
                Bracket::Par => match finished_expr {
                    Some(function) => UnfinishedExpr::List {
                        opr: UnfinishedListOpr::FunctionCall { function },
                        bra,
                        bra_token_idx,
                        items: vec![],
                        commas: vec![],
                    }
                    .into(),
                    None => UnfinishedExpr::List {
                        opr: UnfinishedListOpr::NewTuple,
                        bra,
                        bra_token_idx,
                        items: vec![],
                        commas: vec![],
                    }
                    .into(),
                },
                Bracket::Box => UnfinishedExpr::List {
                    opr: match finished_expr {
                        Some(finished_expr) => UnfinishedListOpr::Index {
                            owner: finished_expr,
                        },
                        None => UnfinishedListOpr::BoxList,
                    },
                    bra,
                    bra_token_idx,
                    items: vec![],
                    commas: vec![],
                }
                .into(),
                Bracket::Angle => match finished_expr {
                    Some(template) => UnfinishedExpr::List {
                        opr: UnfinishedListOpr::TemplateInstantiation { template },
                        bra,
                        bra_token_idx,
                        items: vec![],
                        commas: vec![],
                    }
                    .into(),
                    None => todo!(),
                },
                Bracket::Curl => todo!(),
                Bracket::Vertical => todo!(),
            }
        })
    }
}
