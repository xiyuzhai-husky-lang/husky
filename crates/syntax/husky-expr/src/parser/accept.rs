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
                    Bracket::Par => todo!(),
                    Bracket::Box => ListOpr::NewVec,
                    Bracket::Angle => todo!(),
                    Bracket::Curl => todo!(),
                };
                self.accept_list_start(bra, token_idx, opr)
            }
            ResolvedToken::Ket(token_idx, ket) => self.accept_list_end(ket, token_idx),
            ResolvedToken::Dot(token_idx) => self.accept_dot_opr(token_idx),
            ResolvedToken::Comma(token_idx) => self.accept_comma(token_idx),
        }
    }

    pub(crate) fn accept_list_end(&mut self, ket: Bracket, ket_token_idx: TokenIdx) {
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
    }

    fn accept_atom(&mut self, atom: Expr) {
        self.push_expr(atom)
    }

    fn accept_suffix_opr(&mut self, suffix: SuffixPunctuation, suffix_token_idx: TokenIdx) {
        self.replace_top_expr(|top_expr, sheet| match top_expr {
            Some(expr) => Expr::Opn {
                opn: Opn::Suffix(suffix),
                opds: ExprIdxRange::new_single(sheet.alloc_expr(expr)),
            },
            None => todo!(),
        })
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
