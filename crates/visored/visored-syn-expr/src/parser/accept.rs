use super::*;
use disambiguate_token::DisambiguatedToken;
use either::*;
use expr::{VdSynExprClass, VdSynExprData};
use incomplete_expr::{IncompleteSeparatedListOpr, IncompleteVdSynExprData};
use latex_token::idx::LxTokenIdx;
use smallvec::smallvec;
use visored_opr::opr::{binary::VdBinaryOpr, prefix::VdPrefixOpr, suffix::VdSuffixOpr, VdOpr};

impl<'a, 'db> VdSynExprParser<'a, 'db> {
    pub(crate) fn accept_token(&mut self, token: DisambiguatedToken) {
        match token {
            DisambiguatedToken::Expr(expr, class) => match class {
                VdSynExprClass::Atom => self.accept_atom(expr),
                VdSynExprClass::Prefix => todo!(),
                VdSynExprClass::Suffix => todo!(),
                VdSynExprClass::Separator => todo!(),
            },
            DisambiguatedToken::Opr(opr) => self.accept_opr(opr),
            DisambiguatedToken::Separator(sep) => self.accept_separator(sep),
        }
    }

    fn accept_list_end(&mut self, ket: Delimiter, ket_regional_token_idx: RegionalTokenIdx) {
        self.reduce(Precedence::ListItem);
        let last_incomplete_expr = self.take_last_incomplete_expr().unwrap();
        match last_incomplete_expr {
            IncompleteSynExprData::CommaList {
                opr,
                bra,
                bra_regional_token_idx,
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
                        IncompleteCommaListOpr::UnitOrDelimiteredOrNewTuple => match items.last() {
                            None => SynExprData::Unit {
                                lpar_regional_token_idx: bra_regional_token_idx,
                                rpar_regional_token_idx: ket_regional_token_idx,
                            },
                            Some(last_item) => {
                                if items.len() == 1
                                    && last_item.comma_regional_token_idx().is_none()
                                {
                                    SynExprData::Delimitered {
                                        lpar_regional_token_idx: bra_regional_token_idx,
                                        item: last_item.syn_expr_idx(),
                                        rpar_regional_token_idx: ket_regional_token_idx,
                                    }
                                } else {
                                    SynExprData::NewTuple {
                                        lpar_regional_token_idx: bra_regional_token_idx,
                                        items,
                                        rpar_regional_token_idx: ket_regional_token_idx,
                                    }
                                }
                            }
                        }
                        .into(),
                        IncompleteCommaListOpr::Index { owner } => {
                            SynExprData::IndexOrCompositionWithList {
                                owner,
                                lbox_regional_token_idx: bra_regional_token_idx,
                                items,
                                rbox_regional_token_idx: ket_regional_token_idx,
                            }
                            .into()
                        }
                        IncompleteCommaListOpr::BoxList => SynExprData::List {
                            lbox_regional_token_idx: bra_regional_token_idx,
                            items,
                            rbox_regional_token_idx: ket_regional_token_idx,
                        }
                        .into(),
                        IncompleteCommaListOpr::BoxColonList {
                            colon_regional_token_idx,
                        } => SynExprData::BoxColonList {
                            lbox_regional_token_idx: bra_regional_token_idx,
                            colon_regional_token_idx,
                            items,
                            rbox_regional_token_idx: ket_regional_token_idx,
                        }
                        .into(),
                        IncompleteCommaListOpr::FunctionApplicationOrCall { function } => {
                            // ad hoc
                            let generic_arguments: Option<SynTemplateArguments> = None;
                            SynExprData::FunctionApplicationOrCall {
                                function,
                                template_arguments: generic_arguments,
                                lpar_regional_token_idx: bra_regional_token_idx,
                                items,
                                rpar_regional_token_idx: ket_regional_token_idx,
                            }
                            .into()
                        }
                        IncompleteCommaListOpr::MethodInstantiation { .. } => {
                            todo!()
                        }
                        IncompleteCommaListOpr::MethodApplicationOrCall {
                            self_expr,
                            dot_regional_token_idx,
                            ident_token,
                            template_arguments,
                        } => SynExprData::MethodApplicationOrCall {
                            self_argument: self_expr,
                            dot_regional_token_idx,
                            ident_token,
                            template_arguments,
                            lpar_regional_token_idx: bra_regional_token_idx,
                            items,
                            rpar_regional_token_idx: ket_regional_token_idx,
                        }
                        .into(),
                        IncompleteCommaListOpr::TemplateInstantiation { template } => {
                            SynExprData::TemplateInstantiation {
                                template,
                                template_arguments: SynTemplateArguments::new(
                                    bra_regional_token_idx,
                                    items,
                                    ket_regional_token_idx,
                                ),
                            }
                            .into()
                        }
                        IncompleteCommaListOpr::RitchieArguments {
                            ritchie_kind_regional_token_idx,
                            ritchie_kind,
                            lpar_token,
                        } => match this.try_parse_option::<LightArrowRegionalToken>() {
                            Ok(Some(light_arrow_token)) => IncompleteSynExprData::Ritchie {
                                ritchie_kind_regional_token_idx,
                                ritchie_kind,
                                lpar_token,
                                argument_tys: items,
                                rpar_regional_token_idx: ket_regional_token_idx,
                                light_arrow_token,
                            }
                            .into(),
                            Ok(None) => todo!(),
                            Err(_) => todo!(),
                        },
                    }
                })
            }
            IncompleteSynExprData::CallList {
                opr,
                lpar_regional_token_idx,
                items,
            } => match opr {
                IncompleteCallListOpr::FunctionCall {
                    function,
                    generic_arguments,
                } => self.set_complete_expr(SynExprData::FunctionCall {
                    function,
                    template_arguments: generic_arguments,
                    lpar_regional_token_idx,
                    items,
                    rpar_regional_token_idx: ket_regional_token_idx,
                }),
                IncompleteCallListOpr::MethodCall { .. } => todo!(),
            },
            _ => {
                p!(last_incomplete_expr);
                // p!(self.context.path.debug(self.db()));
                p!(ket_regional_token_idx);
                todo!()
            }
        }
    }

    fn accept_atom(&mut self, atom: SynExprData) {
        self.push_top_syn_expr(atom.into())
    }

    fn accept_opr(&mut self, opr: VdOpr) {
        match opr {
            VdOpr::Binary(opr) => todo!(),
            VdOpr::Prefix(opr) => self.accept_prefix_opr(Left(opr)),
            VdOpr::Suffix(opr) => self.accept_suffix_opr(Left(opr)),
        }
    }

    fn accept_prefix_opr(&mut self, opr: Either<VdPrefixOpr, VdSynExprIdx>) {
        self.push_top_syn_expr(IncompleteVdSynExprData::Prefix { opr }.into())
    }

    fn accept_suffix_opr(&mut self, opr: Either<VdSuffixOpr, VdSynExprIdx>) {
        self.take_complete_and_push_to_top(|slf, top_expr| match top_expr {
            Some(expr) => VdSynExprData::Suffix {
                opd: slf.builder.alloc_expr(expr, todo!()),
                opr,
            }
            .into(),
            None => todo!(),
        })
    }

    fn accept_dot_opr(&mut self, dot_regional_token_idx: RegionalTokenIdx) {
        self.take_complete_and_push_to_top(|this, finished_expr| match finished_expr {
            Some(self_expr) => {
                let self_expr = this.context_mut().alloc_expr(self_expr);
                match this.try_parse_option::<IdentRegionalToken>() {
                    Ok(Some(ident_token)) => match this.try_parse_option::<LparRegionalToken>() {
                        Ok(Some(lpar)) => IncompleteSynExprData::CommaList {
                            opr: IncompleteCommaListOpr::MethodApplicationOrCall {
                                self_expr,
                                dot_regional_token_idx,
                                ident_token,
                                template_arguments: None,
                            },
                            bra: Delimiter::Par,
                            bra_regional_token_idx: lpar.regional_token_idx(),
                            items: smallvec![],
                        }
                        .into(),
                        Ok(None) => match this.try_parse_option::<ColonColonLaRegionalToken>() {
                            Ok(Some(langle)) => IncompleteSynExprData::CommaList {
                                opr: IncompleteCommaListOpr::MethodInstantiation {
                                    self_expr,
                                    dot_regional_token_idx,
                                    ident_token,
                                },
                                bra: Delimiter::TurboFish,
                                bra_regional_token_idx: langle.regional_token_idx(),
                                items: smallvec![],
                            }
                            .into(),
                            Ok(None) => SynExprData::Field {
                                owner: self_expr,
                                dot_regional_token_idx,
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
                    _ => SynExprData::Err(
                        OriginalSynExprError::ExpectedIdentAfterDot {
                            dot_regional_token_idx,
                        }
                        .into(),
                    )
                    .into(),
                }
            }
            None => SynExprData::Err(
                OriginalSynExprError::ExpectedExprBeforeDot {
                    dot_regional_token_idx,
                }
                .into(),
            )
            .into(),
        })
    }

    fn accept_comma(&mut self, comma_regional_token_idx: RegionalTokenIdx) {
        match self.take_complete_expr() {
            Some(item) => {
                let item = self.context_mut().alloc_expr(item);
                match self.last_incomplete_expr_mut() {
                    Some(expr) => match expr {
                        IncompleteSynExprData::CommaList {
                            opr: _,
                            bra: _,
                            bra_regional_token_idx: _,
                            items,
                        } => {
                            items.push(SynCommaListItem::new(item, Some(comma_regional_token_idx)))
                        }
                        IncompleteSynExprData::CallList { items, .. } => items.push(
                            SynSimpleOrVariadicCallListItem::new(
                                item,
                                CallListSeparator::Comma(comma_regional_token_idx),
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
                    IncompleteSynExprData::CommaList {
                        opr: _,
                        bra: _,
                        bra_regional_token_idx: _,
                        items: _,
                    } => todo!(),
                    IncompleteSynExprData::CallList { items, .. } => match items.last_mut() {
                        Some(last_item) => match last_item.separator() {
                            CallListSeparator::None => last_item
                                .set_separator(CallListSeparator::Comma(comma_regional_token_idx)),
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

    fn accept_be_pattern(&mut self, be_regional_token_idx: RegionalTokenIdx) {
        self.reduce(Precedence::Be);
        let src = self.take_complete_expr().unwrap_or(SynExprData::Err(
            OriginalSynExprError::ExpectedItemBeforeBe {
                be_regional_token_idx,
            }
            .into(),
        ));
        let src = self.context_mut().alloc_expr(src);
        let end = match self.env() {
            Some(env) => match env {
                ExprEnvironment::TypeBeforeEq => todo!(),
                ExprEnvironment::WithinDelimiteredParameterList(_) => todo!(),
                ExprEnvironment::Condition(end) => end,
            },
            None => todo!(),
        };
        let expr = SynExprData::Be {
            src,
            be_regional_token_idx,
            target: self.parse_be_variables_pattern_expected(end),
        };
        self.push_top_syn_expr(expr.into())
    }

    fn accept_binary_opr(&mut self, binary: Either<VdBinaryOpr, VdSynExprIdx>) {
        self.reduce(binary.precedence());
        let lopd = self.take_complete_expr().unwrap_or(VdSynExprData::Err(
            OriginalSynExprError::NoLeftOperandForBinaryOperator {
                binary_regional_token_idx,
            }
            .into(),
        ));
        let lopd = self.builder.alloc_expr(lopd, todo!());
        let unfinished_expr = IncompleteVdSynExprData::Binary {
            lopd,
            punctuation: binary,
            punctuation_regional_token_idx: binary_regional_token_idx,
        };
        self.push_top_syn_expr(unfinished_expr.into())
    }

    fn accept_list_start(&mut self, bra: Delimiter, bra_regional_token_idx: RegionalTokenIdx) {
        self.reduce(Precedence::Application);
        if bra == Delimiter::Vert {
            let lvert = bra_regional_token_idx;
            let closure = self.parse_closure(bra_regional_token_idx);
            self.push_top_syn_expr(closure.into())
        } else {
            self.take_complete_and_push_to_top(|parser, finished_expr| -> TopSynExpr {
                let finished_expr = finished_expr.map(|expr| parser.context_mut().alloc_expr(expr));
                match bra {
                    Delimiter::Par => match finished_expr {
                        Some(function) => IncompleteSynExprData::CommaList {
                            opr: IncompleteCommaListOpr::FunctionApplicationOrCall { function },
                            bra,
                            bra_regional_token_idx,
                            items: smallvec![],
                        }
                        .into(),
                        None => IncompleteSynExprData::CommaList {
                            opr: IncompleteCommaListOpr::UnitOrDelimiteredOrNewTuple,
                            bra,
                            bra_regional_token_idx,
                            items: smallvec![],
                        }
                        .into(),
                    },
                    Delimiter::Box => IncompleteSynExprData::CommaList {
                        opr: match finished_expr {
                            Some(finished_expr) => IncompleteCommaListOpr::Index {
                                owner: finished_expr,
                            },
                            None => IncompleteCommaListOpr::BoxList,
                        },
                        bra,
                        bra_regional_token_idx,
                        items: smallvec![],
                    }
                    .into(),
                    Delimiter::TurboFish => match finished_expr {
                        Some(template) => IncompleteSynExprData::CommaList {
                            opr: IncompleteCommaListOpr::TemplateInstantiation { template },
                            bra,
                            bra_regional_token_idx,
                            items: smallvec![],
                        }
                        .into(),
                        None => todo!(),
                    },
                    Delimiter::NestedCurl => todo!(),
                    Delimiter::InlineCurl => SynExprData::Err(
                        OriginalSynExprError::UnexpectedInlineLcurl(bra_regional_token_idx).into(),
                    )
                    .into(),
                    Delimiter::Vert => {
                        unreachable!("Handled already")
                    }
                    Delimiter::HtmlAngle => {
                        let function_ident = match parser.try_parse_expected(
                            OriginalSynExprError::ExpectedFunctionIdentAfterOpeningHtmlBra,
                        ) {
                            Ok(function_ident) => function_ident,
                            Err(e) => return SynExprData::Err(e).into(),
                        };
                        let arguments = match parse_consecutive_vec_map(parser) {
                            Ok(arguments) => arguments,
                            Err(e) => return SynExprData::Err(e).into(),
                        };
                        match parser.try_parse_option::<EmptyHtmxKetRegionalToken>() {
                            Ok(Some(empty_html_ket)) => SynExprData::EmptyHtmlTag {
                                empty_html_bra_idx: bra_regional_token_idx,
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
    }
}
