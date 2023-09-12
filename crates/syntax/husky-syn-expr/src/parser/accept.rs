use crate::RegularOrVariadicCallListItem;

use super::*;
use husky_print_utils::p;
use parsec::{parse_consecutive_list, parse_consecutive_vec_map, StreamParser};
use smallvec::smallvec;

impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
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

    fn accept_list_end(&mut self, ket: Bracket, ket_token_idx: RegionalTokenIdx) {
        self.reduce(Precedence::ListItem);
        let last_incomplete_expr = self.take_last_incomplete_expr().unwrap();
        match last_incomplete_expr {
            IncompleteExpr::CommaList {
                opr,
                bra,
                bra_token_idx,
                mut items,
            } => {
                if bra != ket {
                    todo!()
                }
                self.take_complete_and_push_to_top(|this, finished_expr| {
                    if let Some(expr) = finished_expr {
                        items.push(SynCommaListItem::new(
                            this.context_mut().alloc_expr(expr),
                            None,
                        ))
                    }
                    match opr {
                        IncompleteCommaListOpr::UnitOrBracketedOrNewTuple => match items.last() {
                            None => SynExpr::Unit {
                                lpar_token_idx: bra_token_idx,
                                rpar_token_idx: ket_token_idx,
                            },
                            Some(last_item) => {
                                if items.len() == 1 && last_item.comma_token_idx().is_none() {
                                    SynExpr::Bracketed {
                                        lpar_token_idx: bra_token_idx,
                                        item: last_item.expr_idx(),
                                        rpar_token_idx: ket_token_idx,
                                    }
                                } else {
                                    SynExpr::NewTuple {
                                        lpar_token_idx: bra_token_idx,
                                        items,
                                        rpar_token_idx: ket_token_idx,
                                    }
                                }
                            }
                        }
                        .into(),
                        IncompleteCommaListOpr::Index { owner } => {
                            SynExpr::IndexOrCompositionWithList {
                                owner,
                                lbox_token_idx: bra_token_idx,
                                items,
                                rbox_token_idx: ket_token_idx,
                            }
                            .into()
                        }
                        IncompleteCommaListOpr::BoxList => SynExpr::List {
                            lbox_token_idx: bra_token_idx,
                            items,
                            rbox_token_idx: ket_token_idx,
                        }
                        .into(),
                        IncompleteCommaListOpr::BoxColonList { colon_token_idx } => {
                            SynExpr::BoxColonList {
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
                            let generic_arguments: Option<SynGenericArgumentList> = None;
                            SynExpr::FunctionApplicationOrCall {
                                function,
                                generic_arguments,
                                lpar_token_idx: bra_token_idx,
                                items,
                                rpar_token_idx: ket_token_idx,
                            }
                            .into()
                        }
                        IncompleteCommaListOpr::MethodInstantiation { .. } => todo!(),
                        IncompleteCommaListOpr::MethodApplicationOrCall {
                            self_expr,
                            dot_token_idx,
                            ident_token,
                            generic_arguments,
                        } => SynExpr::MethodApplicationOrCall {
                            self_argument: self_expr,
                            dot_token_idx,
                            ident_token,
                            generic_arguments,
                            lpar_token_idx: bra_token_idx,
                            items,
                            rpar_token_idx: ket_token_idx,
                        }
                        .into(),
                        IncompleteCommaListOpr::TemplateInstantiation { template } => {
                            SynExpr::TemplateInstantiation {
                                template,
                                generic_arguments: SynGenericArgumentList::new(
                                    bra_token_idx,
                                    items,
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
                        } => match this.try_parse_option::<LightArrowToken>() {
                            Ok(Some(light_arrow_token)) => IncompleteExpr::Ritchie {
                                ritchie_kind_token_idx,
                                ritchie_kind,
                                lpar_token,
                                argument_tys: items,
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
                    generic_arguments,
                } => self.set_complete_expr(SynExpr::FunctionCall {
                    function,
                    generic_arguments,
                    lpar_token_idx,
                    items,
                    rpar_token_idx: ket_token_idx,
                }),
                IncompleteCallListOpr::MethodCall {
                    self_expr,
                    dot_token_idx,
                    ident_token,
                    generic_arguments,
                } => todo!(),
            },
            // IncompleteExpr::RitchieCallKeyedArgumentList {
            //     function,
            //     generic_arguments,
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
            //     generic_arguments,
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
                // p!(self.context.path.debug(self.db()));
                p!(ket_token_idx);
                todo!()
            }
        }
    }

    fn accept_atom(&mut self, atom: SynExpr) {
        self.push_top_expr(atom.into())
    }

    fn accept_prefix_opr(&mut self, prefix: PrefixOpr, prefix_token_idx: RegionalTokenIdx) {
        self.push_top_expr(
            IncompleteExpr::Prefix {
                punctuation: prefix,
                punctuation_token_idx: prefix_token_idx,
            }
            .into(),
        )
    }

    fn accept_suffix_opr(
        &mut self,
        punctuation: SuffixOpr,
        punctuation_token_idx: RegionalTokenIdx,
    ) {
        self.take_complete_and_push_to_top(|this, top_expr| match top_expr {
            Some(expr) => SynExpr::Suffix {
                opd: this.context_mut().alloc_expr(expr),
                opr: punctuation,
                opr_token_idx: punctuation_token_idx,
            }
            .into(),
            None => todo!(),
        })
    }

    fn accept_dot_opr(&mut self, dot_token_idx: RegionalTokenIdx) {
        self.take_complete_and_push_to_top(|this, finished_expr| match finished_expr {
            Some(self_expr) => {
                let self_expr = this.context_mut().alloc_expr(self_expr);
                match this.try_parse_option::<RegionalIdentToken>() {
                    Ok(Some(ident_token)) => match this.try_parse_option::<LparToken>() {
                        Ok(Some(lpar)) => IncompleteExpr::CommaList {
                            opr: IncompleteCommaListOpr::MethodApplicationOrCall {
                                self_expr,
                                dot_token_idx,
                                ident_token,
                                generic_arguments: None,
                            },
                            bra: Bracket::Par,
                            bra_token_idx: lpar.token_idx(),
                            items: smallvec![],
                        }
                        .into(),
                        Ok(None) => match this.try_parse_option::<ColonColonLaToken>() {
                            Ok(Some(langle)) => IncompleteExpr::CommaList {
                                opr: IncompleteCommaListOpr::MethodInstantiation {
                                    self_expr,
                                    dot_token_idx,
                                    ident_token,
                                },
                                bra: Bracket::TemplateAngle,
                                bra_token_idx: langle.token_idx(),
                                items: smallvec![],
                            }
                            .into(),
                            Ok(None) => SynExpr::Field {
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
                    _ => SynExpr::Err(
                        OriginalSynExprError::ExpectedIdentAfterDot { dot_token_idx }.into(),
                    )
                    .into(),
                }
            }
            None => {
                SynExpr::Err(OriginalSynExprError::ExpectedExprBeforeDot { dot_token_idx }.into())
                    .into()
            }
        })
    }

    fn accept_comma(&mut self, comma_token_idx: RegionalTokenIdx) {
        match self.take_complete_expr() {
            Some(item) => {
                let item = self.context_mut().alloc_expr(item);
                match self.last_incomplete_expr_mut() {
                    Some(expr) => match expr {
                        IncompleteExpr::CommaList {
                            opr,
                            bra,
                            bra_token_idx,
                            items,
                        } => items.push(SynCommaListItem::new(item, Some(comma_token_idx))),
                        IncompleteExpr::CallList { items, .. } => items.push(
                            RegularOrVariadicCallListItem::new(
                                item,
                                CallListSeparator::Comma(comma_token_idx),
                            )
                            .into(),
                        ),
                        _ => unreachable!(),
                    },
                    None => unreachable!(),
                }
            }
            None => match self.last_incomplete_expr_mut() {
                Some(expr) => match expr {
                    IncompleteExpr::CommaList {
                        opr,
                        bra,
                        bra_token_idx,
                        items,
                    } => todo!(),
                    IncompleteExpr::CallList { items, .. } => match items.last_mut() {
                        Some(last_item) => match last_item.separator() {
                            CallListSeparator::None => {
                                last_item.set_separator(CallListSeparator::Comma(comma_token_idx))
                            }
                            CallListSeparator::Comma(_) => todo!(),
                            CallListSeparator::Semicolon(_) => todo!(),
                        },
                        None => todo!(),
                    },
                    _ => unreachable!(),
                },
                None => unreachable!(),
            },
        }
    }

    fn accept_be_pattern(&mut self, be_token_idx: RegionalTokenIdx) {
        self.reduce(Precedence::Be);
        let src = self.take_complete_expr().unwrap_or(SynExpr::Err(
            OriginalSynExprError::ExpectedItemBeforeBe { be_token_idx }.into(),
        ));
        let src = self.context_mut().alloc_expr(src);
        let end = match self.env() {
            Some(env) => match env {
                ExprEnvironment::TypeBeforeEq => todo!(),
                ExprEnvironment::WithinBracketedParameterList(_) => todo!(),
                ExprEnvironment::Condition(end) => end,
            },
            None => todo!(),
        };
        let expr = SynExpr::Be {
            src,
            be_token_idx,
            target: self.parse_be_variables_pattern_expected(end),
        };
        self.push_top_expr(expr.into())
    }

    fn accept_binary_opr(&mut self, binary: BinaryOpr, binary_token_idx: RegionalTokenIdx) {
        self.reduce(binary.into());
        let lopd = self.take_complete_expr().unwrap_or(SynExpr::Err(
            OriginalSynExprError::NoLeftOperandForBinaryOperator { binary_token_idx }.into(),
        ));
        let unfinished_expr = IncompleteExpr::Binary {
            lopd,
            punctuation: binary,
            punctuation_token_idx: binary_token_idx,
        };
        self.push_top_expr(unfinished_expr.into())
    }

    fn accept_colon_right_after_lbox(&mut self, colon_token_idx: RegionalTokenIdx) {
        #[cfg(test)]
        assert!(self.complete_expr().is_none());
        let unfinished_expr = self.take_last_incomplete_expr().unwrap();
        match unfinished_expr {
            IncompleteExpr::CommaList {
                opr: IncompleteCommaListOpr::BoxList,
                bra,
                bra_token_idx,
                items,
            } => {
                assert!(items.is_empty());
                self.push_top_expr(
                    IncompleteExpr::CommaList {
                        opr: IncompleteCommaListOpr::BoxColonList { colon_token_idx },
                        bra,
                        bra_token_idx,
                        items,
                    }
                    .into(),
                )
            }
            _ => unreachable!(),
        }
    }

    fn accept_list_start(&mut self, bra: Bracket, bra_token_idx: RegionalTokenIdx) {
        self.take_complete_and_push_to_top(|parser, finished_expr| -> TopExpr {
            let finished_expr = finished_expr.map(|expr| parser.context_mut().alloc_expr(expr));
            match bra {
                Bracket::Par => match finished_expr {
                    Some(function) => IncompleteExpr::CommaList {
                        opr: IncompleteCommaListOpr::FunctionApplicationOrCall { function },
                        bra,
                        bra_token_idx,
                        items: smallvec![],
                    }
                    .into(),
                    None => IncompleteExpr::CommaList {
                        opr: IncompleteCommaListOpr::UnitOrBracketedOrNewTuple,
                        bra,
                        bra_token_idx,
                        items: smallvec![],
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
                    items: smallvec![],
                }
                .into(),
                Bracket::TemplateAngle => match finished_expr {
                    Some(template) => IncompleteExpr::CommaList {
                        opr: IncompleteCommaListOpr::TemplateInstantiation { template },
                        bra,
                        bra_token_idx,
                        items: smallvec![],
                    }
                    .into(),
                    None => todo!(),
                },
                Bracket::Curl => SynExpr::Err(
                    OriginalSynExprError::UnexpectedLeftCurlyBrace(bra_token_idx).into(),
                )
                .into(),
                Bracket::Lambda => todo!(),
                Bracket::HtmlAngle => {
                    let function_ident = match parser.try_parse_expected(
                        OriginalSynExprError::ExpectedFunctionIdentAfterOpeningHtmlBra,
                    ) {
                        Ok(function_ident) => function_ident,
                        Err(e) => return SynExpr::Err(e).into(),
                    };
                    let arguments = match parse_consecutive_vec_map(parser) {
                        Ok(arguments) => arguments,
                        Err(e) => return SynExpr::Err(e).into(),
                    };
                    match parser.try_parse_option::<EmptyHtmlKetToken>() {
                        Ok(Some(empty_html_ket)) => SynExpr::EmptyHtmlTag {
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

    fn accept_ritchie(
        &mut self,
        ritchie_kind_token_idx: RegionalTokenIdx,
        ritchie_kind: RitchieKind,
    ) {
        match self.try_parse_option::<LparToken>() {
            Ok(Some(lpar_token)) => self.push_top_expr(
                IncompleteExpr::CommaList {
                    opr: IncompleteCommaListOpr::RitchieArguments {
                        ritchie_kind_token_idx,
                        ritchie_kind,
                        lpar_token,
                    },
                    bra: Bracket::Par,
                    bra_token_idx: lpar_token.token_idx(),
                    items: smallvec![],
                }
                .into(),
            ),
            Ok(None) => todo!(),
            Err(_) => todo!(),
        }
    }

    fn accept_incomplete_keyword_argument(
        &mut self,
        key_token_idx: RegionalTokenIdx,
        key: Ident,
        eq_token: RegionalEqToken,
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
