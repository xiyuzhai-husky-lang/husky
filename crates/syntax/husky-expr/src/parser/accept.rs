use super::*;

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(crate) fn next_token(&mut self) -> Option<&'b Token> {
        self.token_iter.next()
    }

    pub(crate) fn accept_token(&mut self, token: ResolvedToken) {
        match token.kind() {
            ResolvedTokenKind::Atom(_atom) => {
                self.accept_atom(token.to_expr(self.sheet.expr_arena()))
            }
            ResolvedTokenKind::BinaryPunctuation(opr) => {
                self.accept_binary_opr(*opr, token.token_idx())
            }
            ResolvedTokenKind::PrefixPunctuation(opr) => {
                self.accept_prefix_opr(*opr, token.token_idx())
            }
            ResolvedTokenKind::SuffixPunctuation(opr) => {
                self.accept_suffix_opr(*opr, token.token_idx())
            }
            ResolvedTokenKind::Bra(bra) => {
                let opr = match bra {
                    Bracket::Par => todo!(),
                    Bracket::Box => ListOpr::NewVec,
                    Bracket::Angle => todo!(),
                    Bracket::Curl => todo!(),
                };
                self.accept_list_start(*bra, token.token_idx(), opr)
            }
            ResolvedTokenKind::Ket(ket) => {
                self.accept_list_end(*ket, token.token_idx(), ListEndAttr::None)
            }
            ResolvedTokenKind::Dot => self.accept_dot_opr(token.token_idx()),
            ResolvedTokenKind::Comma => self.accept_comma(token.token_idx()),
        }
    }

    pub(crate) fn accept_list_end(
        &mut self,
        ket: Bracket,
        ket_token_idx: TokenIdx,
        attr: ListEndAttr,
    ) {
        self.reduce(Precedence::ListItem);
        match self.take_top_unfinished_expr().unwrap() {
            UnfinishedExpr::List {
                opr,
                bra,
                bra_token_idx,
                mut items,
            } => {
                if bra != ket {
                    todo!()
                }
                if let Some(expr) = self.take_top_expr() {
                    items.push(expr)
                }
                let opds = self.sheet.alloc_expr_batch(items);
                self.set_top_expr(Expr::Opn {
                    opn: Opn::List {
                        opr,
                        bracket: bra,
                        bra_token_idx,
                        ket_token_idx,
                    },
                    opds,
                })
            }
            _ => todo!(),
        }
        // let original_number_of_oprs = self.number_of_oprs();
        // let (start_attr, bra_token) = {
        //     loop {
        //         match self.pop_opr() {
        //             Some(opr) => match opr {
        //                 UnfinishedExpr::ListItem { .. } => (),
        //                 UnfinishedExpr::ListStart {
        //                     bra,
        //                     bra_token_idx,
        //                     attr,
        //                 } => {
        //                     if ket != bra {
        //                         return Err(ExprError::MisMatchingBracket {
        //                             bra,
        //                             bra_token_idx,
        //                             ket,
        //                             ket_token_idx,
        //                         });
        //                     };
        //                     break (attr, bra_token_idx);
        //                 }
        //                 _ => return Err(ExprError::NoMatchingBra { ket, ket_token_idx }),
        //             },
        //             None => return Err(ExprError::NoMatchingBra { ket, ket_token_idx }),
        //         }
        //     }
        // };
        // let list_len = original_number_of_oprs - self.number_of_oprs() - 1;
        // let (opds, paths) = self.drain_exprs(list_len);
        // let opds = self.sheet.alloc_expr_batch(opds, paths);
        // self.push_expr(new_list_expr(ket, start_attr, attr, opds)?);
        // Ok(())
    }

    fn accept_atom(&mut self, atom: Expr) {
        self.push_expr(atom)
    }

    fn accept_suffix_opr(&mut self, suffix: SuffixPunctuation, suffix_token_idx: TokenIdx) {
        {
            let suffix_token_idx = suffix_token_idx;
            self.replace_top_expr(|top_expr, sheet| match top_expr {
                Some(expr) => Expr::Opn {
                    opn: Opn::Suffix(suffix),
                    opds: ExprIdxRange::new_single(sheet.alloc_expr(expr)),
                },
                None => todo!(),
            })
            // self.synthesize_opn(suffix.into(), 1)
        }
    }

    fn accept_dot_opr(&mut self, dot_token_idx: TokenIdx) {
        // let (ident, ident_token_idx) = self
        //     .try_get_identifier()
        //     .ok_or(ExprError::ExpectIdentifierAfterDot)?;
        // self.push_opr(StackOpr::Dot { dot_token_idx });
        todo!()
    }

    fn accept_comma(&mut self, token_idx: TokenIdx) {
        match self.last_unfinished_expr() {
            Some(_) => todo!(),
            None => todo!(),
        }
    }

    pub(crate) fn accept_binary_opr(
        &mut self,
        binary: BinaryPunctuation,
        binary_token_idx: TokenIdx,
    ) {
        let lopd = self
            .take_top_expr()
            .unwrap_or(Expr::Err(ExprError::NoLeftOperandForBinaryOperator));
        let unfinished_expr = UnfinishedExpr::Binary {
            lopd,
            binary,
            binary_token_idx,
        };
        self.reduce(unfinished_expr.precedence());
        self.push_unfinished_expr(unfinished_expr)
    }

    pub(crate) fn accept_list_start(
        &mut self,
        bra: Bracket,
        bra_token_idx: TokenIdx,
        opr: ListOpr,
    ) {
        self.push_unfinished_expr(UnfinishedExpr::List {
            opr,
            bra,
            bra_token_idx,
            items: vec![],
        });
    }

    pub(crate) fn accept_list_item(&mut self, separator_token_idx: Option<TokenIdx>) {
        self.reduce(Precedence::ListItem);
        self.push_unfinished_expr(UnfinishedExpr::ListItem {
            separator_token_idx,
        })
    }
}
