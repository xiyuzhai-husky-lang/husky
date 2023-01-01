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
            ResolvedToken::Bra(token_idx, bra) => {
                let opr = match bra {
                    Bracket::Par => match self.top_expr() {
                        TopExprRef::Unfinished(UnfinishedExpr::Method { .. }) => {
                            ListOpr::MethodCall
                        }
                        TopExprRef::Finished(_) => ListOpr::FunctionCall,
                        _ => ListOpr::NewTuple,
                    },
                    Bracket::Box => ListOpr::NewVec,
                    Bracket::Angle => todo!(),
                    Bracket::Curl => todo!(),
                    Bracket::Vertical => ListOpr::NewLambdaHead,
                };
                self.accept_list_start(bra, token_idx, opr)
            }
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
                if let Some(expr) = self.take_finished_expr() {
                    items.push(expr)
                }
                let opds = self.sheet.alloc_expr_batch(items);
                self.set_top_expr(
                    Expr::Opn {
                        opn: Opn::List {
                            opr,
                            bracket: bra,
                            bra_token_idx,
                            ket_token_idx,
                        },
                        opds,
                    }
                    .into(),
                )
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
                prefix,
                prefix_token_idx,
            }
            .into(),
        )
    }

    fn accept_suffix_opr(&mut self, suffix: SuffixPunctuation, suffix_token_idx: TokenIdx) {
        self.replace_finished_expr(|this, top_expr| match top_expr {
            Some(expr) => Expr::Opn {
                opn: Opn::Suffix {
                    suffix,
                    suffix_token_idx,
                },
                opds: ExprIdxRange::new_single(this.sheet.alloc_expr(expr)),
            }
            .into(),
            None => todo!(),
        })
    }

    fn accept_dot_opr(&mut self, dot_token_idx: TokenIdx) {
        self.replace_finished_expr(|this, finished_expr| match finished_expr {
            Some(this_expr) => match this.parse::<IdentifierToken>() {
                Ok(Some(ident_token)) => match this.peek() {
                    Some(Token {
                        kind:
                            TokenKind::Punctuation(Punctuation::Bra(Bracket::Angle | Bracket::Par)),
                        ..
                    }) => UnfinishedExpr::Method {
                        ident_token,
                        this_expr,
                    }
                    .into(),
                    _ => Expr::Opn {
                        opn: Opn::Field { ident_token },
                        opds: this.sheet.alloc_expr_batch([this_expr]),
                    }
                    .into(),
                },
                _ => todo!(),
            },
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
            binary,
            binary_token_idx,
        };
        self.reduce(unfinished_expr.precedence());
        self.set_top_expr(unfinished_expr.into())
    }

    pub(crate) fn accept_list_start(
        &mut self,
        bra: Bracket,
        bra_token_idx: TokenIdx,
        opr: ListOpr,
    ) {
        self.set_top_expr(
            UnfinishedExpr::List {
                opr,
                bra,
                bra_token_idx,
                items: vec![],
            }
            .into(),
        );
    }
}
