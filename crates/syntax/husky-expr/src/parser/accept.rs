use super::*;
use husky_print_utils::p;
use parsec::{parse_consecutive_list, parse_consecutive_vec_map, StreamParser};
use smallvec::smallvec;

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn accept_token(&mut self, token: DisambiguatedToken) {
        match token {
            DisambiguatedToken::AtomicExpr(atom) => self.accept_atom(atom),
            DisambiguatedToken::BinaryOpr(token_idx, opr) => self.accept_binary_opr(opr, token_idx),
            DisambiguatedToken::PrefixOpr(token_idx, opr) => self.accept_prefix_opr(opr, token_idx),
            DisambiguatedToken::SuffixOpr(token_idx, opr) => self.accept_suffix_opr(opr, token_idx),
            DisambiguatedToken::Bra(token_idx, bra) => self.accept_list_start(bra, token_idx),
            DisambiguatedToken::Ket(token_idx, ket) => self.accept_list_end(ket, token_idx),
            DisambiguatedToken::Dot(token_idx) => self.accept_dot_opr(token_idx),
            DisambiguatedToken::Comma(token_idx) => self.accept_comma(token_idx),
            DisambiguatedToken::Be(token_idx) => self.accept_be_pattern(token_idx),
            DisambiguatedToken::ColonRightAfterLBox(colon_token_idx) => {
                self.accept_colon_right_after_lbox(colon_token_idx)
            }
            DisambiguatedToken::Ritchie(token_idx, ritchie_kind) => {
                self.accept_ritchie(token_idx, ritchie_kind)
            }
            DisambiguatedToken::IncompleteKeywordArgument {
                token_idx: ident_token_idx,
                ident,
                eq_token,
            } => self.accept_incomplete_keyword_argument(ident_token_idx, ident, eq_token),
        }
    }

    fn accept_list_end(&mut self, ket: Bracket, ket_token_idx: TokenIdx) {
        self.reduce(Precedence::ListItem);
        let last_incomplete_expr = self.take_last_incomplete_expr().unwrap();
        match last_incomplete_expr {
            IncompleteExpr::CommaList {
                opr,
                bra,
                bra_token_idx,
                mut items,
                commas,
            } => {
                if bra != ket {
                    todo!()
                }
                self.take_complete_and_push_to_top(|this, finished_expr| {
                    if let Some(expr) = finished_expr {
                        items.push(expr)
                    }
                    let items = this.alloc_expr_batch(items);
                    match opr {
                        IncompleteCommaListOpr::NewTuple => match (items.len(), commas.len()) {
                            (0, 0) => Expr::Unit {
                                lpar_token_idx: bra_token_idx,
                                rpar_token_idx: ket_token_idx,
                            },
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
                        IncompleteCommaListOpr::Index { owner } => {
                            Expr::IndexOrCompositionWithList {
                                owner,
                                lbox_token_idx: bra_token_idx,
                                items,
                                rbox_token_idx: ket_token_idx,
                            }
                            .into()
                        }
                        IncompleteCommaListOpr::BoxList => Expr::List {
                            lbox_token_idx: bra_token_idx,
                            items,
                            rbox_token_idx: ket_token_idx,
                        }
                        .into(),
                        IncompleteCommaListOpr::BoxColonList { colon_token_idx } => {
                            Expr::BoxColonList {
                                lbox_token_idx: bra_token_idx,
                                colon_token_idx,
                                items,
                                rbox_token_idx: ket_token_idx,
                            }
                            .into()
                        }
                        IncompleteCommaListOpr::NewLambdaHead => todo!(),
                        IncompleteCommaListOpr::FunctionApplicationOrCall { function } => {
                            // ad hoc
                            let implicit_arguments: Option<ImplicitArgumentList> = None;
                            Expr::FunctionApplicationOrCall {
                                function,
                                implicit_arguments,
                                lpar_token_idx: bra_token_idx,
                                items,
                                commas,
                                rpar_token_idx: ket_token_idx,
                            }
                            .into()
                        }
                        IncompleteCommaListOpr::MethodInstantiation { .. } => todo!(),
                        IncompleteCommaListOpr::MethodApplicationOrCall {
                            self_expr,
                            dot_token_idx,
                            ident_token,
                            implicit_arguments,
                        } => Expr::MethodApplicationOrCall {
                            self_argument: self_expr,
                            dot_token_idx,
                            ident_token,
                            implicit_arguments,
                            lpar_token_idx: bra_token_idx,
                            items,
                            commas,
                            rpar_token_idx: ket_token_idx,
                        }
                        .into(),
                        IncompleteCommaListOpr::TemplateInstantiation { template } => {
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
                        IncompleteCommaListOpr::FunctionInstantiation {} => todo!(),
                        IncompleteCommaListOpr::RitchieArguments {
                            ritchie_kind_token_idx,
                            ritchie_kind,
                            lpar_token,
                        } => match this.parse::<LightArrowToken>() {
                            Ok(Some(light_arrow_token)) => IncompleteExpr::Ritchie {
                                ritchie_kind_token_idx,
                                ritchie_kind,
                                lpar_token,
                                argument_tys: items,
                                commas,
                                rpar_token_idx: ket_token_idx,
                                light_arrow_token,
                            }
                            .into(),
                            Ok(None) => todo!(),
                            Err(_) => todo!(),
                        },
                    }
                })
            }
            IncompleteExpr::CallList {
                opr,
                lpar_token_idx,
                items,
            } => match opr {
                IncompleteCallListOpr::FunctionCall {
                    function,
                    implicit_arguments,
                } => self.set_complete_expr(Expr::FunctionCall {
                    function,
                    implicit_arguments,
                    lpar_token_idx,
                    items,
                    rpar_token_idx: ket_token_idx,
                }),
                IncompleteCallListOpr::MethodCall {
                    self_expr,
                    dot_token_idx,
                    ident_token,
                    implicit_arguments,
                } => todo!(),
            },
            // IncompleteExpr::RitchieCallKeyedArgumentList {
            //     function,
            //     implicit_arguments,
            //     bra,
            //     lpar_token_idx,
            //     arguments,
            //     keyed_arguments,
            //     commas,
            // } => {
            //     if ket != Bracket::Par {
            //         todo!()
            //     }
            // IncompleteExpr::MethodRitchieCallKeyedArgumentList {
            //     self_expr,
            //     dot_token_idx,
            //     ident_token,
            //     implicit_arguments,
            //     bra,
            //     bra_token_idx,
            //     arguments,
            //     commas,
            //     keyed_arguments,
            // } => {
            //     if ket != Bracket::Par {
            //         todo!()
            //     }
            //     todo!()
            // }
            _ => {
                p!(last_incomplete_expr);
                p!(self.parser.path.debug(self.db()));
                p!(ket_token_idx);
                todo!()
            }
        }
    }

    fn accept_atom(&mut self, atom: Expr) {
        self.push_top_expr(atom.into())
    }

    fn accept_prefix_opr(&mut self, prefix: PrefixOpr, prefix_token_idx: TokenIdx) {
        self.push_top_expr(
            IncompleteExpr::Prefix {
                punctuation: prefix,
                punctuation_token_idx: prefix_token_idx,
            }
            .into(),
        )
    }

    fn accept_suffix_opr(&mut self, punctuation: SuffixOpr, punctuation_token_idx: TokenIdx) {
        self.take_complete_and_push_to_top(|this, top_expr| match top_expr {
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
        self.take_complete_and_push_to_top(|this, finished_expr| match finished_expr {
            Some(self_expr) => {
                let self_expr = this.alloc_expr(self_expr);
                match this.parse::<IdentToken>() {
                    Ok(Some(ident_token)) => match this.parse::<LeftParenthesisToken>() {
                        Ok(Some(lpar)) => IncompleteExpr::CommaList {
                            opr: IncompleteCommaListOpr::MethodApplicationOrCall {
                                self_expr,
                                dot_token_idx,
                                ident_token,
                                implicit_arguments: None,
                            },
                            bra: Bracket::Par,
                            bra_token_idx: lpar.token_idx(),
                            items: vec![],
                            commas: smallvec![],
                        }
                        .into(),
                        Ok(None) => match this.parse::<ColonColonLeftAngleBracketToken>() {
                            Ok(Some(langle)) => IncompleteExpr::CommaList {
                                opr: IncompleteCommaListOpr::MethodInstantiation {
                                    self_expr,
                                    dot_token_idx,
                                    ident_token,
                                },
                                bra: Bracket::TemplateAngle,
                                bra_token_idx: langle.token_idx(),
                                items: vec![],
                                commas: smallvec![],
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
                    _ => {
                        Expr::Err(OriginalExprError::ExpectedIdentAfterDot { dot_token_idx }.into())
                            .into()
                    }
                }
            }
            None => {
                Expr::Err(OriginalExprError::ExpectedExprBeforeDot { dot_token_idx }.into()).into()
            }
        })
    }

    fn accept_comma(&mut self, comma_token_idx: TokenIdx) {
        let item = self.take_complete_expr().unwrap_or(Expr::Err(
            OriginalExprError::ExpectedItemBeforeComma { comma_token_idx }.into(),
        ));
        match self.last_incomplete_expr_mut() {
            Some(expr) => match expr {
                IncompleteExpr::CommaList {
                    opr,
                    bra,
                    bra_token_idx,
                    items,
                    commas,
                } => {
                    items.push(item);
                    commas.push(comma_token_idx)
                }
                IncompleteExpr::CallList { .. } => todo!(),
                _ => unreachable!(),
            },
            None => unreachable!(),
        }
    }

    fn accept_be_pattern(&mut self, be_token_idx: TokenIdx) {
        self.reduce(Precedence::Be);
        let src = self.take_complete_expr().unwrap_or(Expr::Err(
            OriginalExprError::ExpectedItemBeforeBe { be_token_idx }.into(),
        ));
        let src = self.alloc_expr(src);
        let end = match self.env() {
            Some(env) => match env {
                ExprEnvironment::TypeBeforeEq => todo!(),
                ExprEnvironment::WithinBracket(_) => todo!(),
                ExprEnvironment::Condition(end) => end,
            },
            None => todo!(),
        };
        let expr = Expr::Be {
            src,
            be_token_idx,
            target: self.parse_be_variables_pattern_expected(end),
        };
        self.push_top_expr(expr.into())
    }

    fn accept_binary_opr(&mut self, binary: BinaryOpr, binary_token_idx: TokenIdx) {
        self.reduce(binary.into());
        let lopd = self.take_complete_expr().unwrap_or(Expr::Err(
            OriginalExprError::NoLeftOperandForBinaryOperator { binary_token_idx }.into(),
        ));
        let unfinished_expr = IncompleteExpr::Binary {
            lopd,
            punctuation: binary,
            punctuation_token_idx: binary_token_idx,
        };
        self.push_top_expr(unfinished_expr.into())
    }

    fn accept_colon_right_after_lbox(&mut self, colon_token_idx: TokenIdx) {
        #[cfg(test)]
        assert!(self.complete_expr().is_none());
        let unfinished_expr = self.take_last_incomplete_expr().unwrap();
        match unfinished_expr {
            IncompleteExpr::CommaList {
                opr: IncompleteCommaListOpr::BoxList,
                bra,
                bra_token_idx,
                items,
                commas,
            } => {
                assert!(items.is_empty());
                self.push_top_expr(
                    IncompleteExpr::CommaList {
                        opr: IncompleteCommaListOpr::BoxColonList { colon_token_idx },
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

    fn accept_list_start(&mut self, bra: Bracket, bra_token_idx: TokenIdx) {
        self.take_complete_and_push_to_top(|parser, finished_expr| -> TopExpr {
            let finished_expr = finished_expr.map(|expr| parser.alloc_expr(expr));
            match bra {
                Bracket::Par => match finished_expr {
                    Some(function) => IncompleteExpr::CommaList {
                        opr: IncompleteCommaListOpr::FunctionApplicationOrCall { function },
                        bra,
                        bra_token_idx,
                        items: vec![],
                        commas: smallvec![],
                    }
                    .into(),
                    None => IncompleteExpr::CommaList {
                        opr: IncompleteCommaListOpr::NewTuple,
                        bra,
                        bra_token_idx,
                        items: vec![],
                        commas: smallvec![],
                    }
                    .into(),
                },
                Bracket::Box => IncompleteExpr::CommaList {
                    opr: match finished_expr {
                        Some(finished_expr) => IncompleteCommaListOpr::Index {
                            owner: finished_expr,
                        },
                        None => IncompleteCommaListOpr::BoxList,
                    },
                    bra,
                    bra_token_idx,
                    items: vec![],
                    commas: smallvec![],
                }
                .into(),
                Bracket::TemplateAngle => match finished_expr {
                    Some(template) => IncompleteExpr::CommaList {
                        opr: IncompleteCommaListOpr::TemplateInstantiation { template },
                        bra,
                        bra_token_idx,
                        items: vec![],
                        commas: smallvec![],
                    }
                    .into(),
                    None => todo!(),
                },
                Bracket::Curl => {
                    Expr::Err(OriginalExprError::UnexpectedLeftCurlyBrace(bra_token_idx).into())
                        .into()
                }
                Bracket::Lambda => todo!(),
                Bracket::HtmlAngle => {
                    let function_ident = match parser
                        .parse_expected(OriginalExprError::ExpectedFunctionIdentAfterOpeningHtmlBra)
                    {
                        Ok(function_ident) => function_ident,
                        Err(e) => return Expr::Err(e).into(),
                    };
                    let arguments = match parse_consecutive_vec_map(parser) {
                        Ok(arguments) => arguments,
                        Err(e) => return Expr::Err(e).into(),
                    };
                    match parser.parse::<EmptyHtmlKetToken>() {
                        Ok(Some(empty_html_ket)) => Expr::EmptyHtmlTag {
                            empty_html_bra_idx: bra_token_idx,
                            function_ident,
                            arguments,
                            empty_html_ket,
                        }
                        .into(),
                        Ok(None) => todo!(),
                        Err(_) => todo!(),
                    }
                }
            }
        })
    }

    fn accept_ritchie(&mut self, ritchie_kind_token_idx: TokenIdx, ritchie_kind: RitchieKind) {
        match self.parse::<LeftParenthesisToken>() {
            Ok(Some(lpar_token)) => self.push_top_expr(
                IncompleteExpr::CommaList {
                    opr: IncompleteCommaListOpr::RitchieArguments {
                        ritchie_kind_token_idx,
                        ritchie_kind,
                        lpar_token,
                    },
                    bra: Bracket::Par,
                    bra_token_idx: lpar_token.token_idx(),
                    items: vec![],
                    commas: smallvec![],
                }
                .into(),
            ),
            Ok(None) => todo!(),
            Err(_) => todo!(),
        }
    }

    fn accept_incomplete_keyword_argument(
        &mut self,
        key_token_idx: TokenIdx,
        key: Ident,
        eq_token: EqToken,
    ) {
        self.push_top_expr(
            IncompleteExpr::KeyedArgument {
                key_token_idx,
                key,
                eq_token,
            }
            .into(),
        )
    }
}
