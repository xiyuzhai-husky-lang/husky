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
            ResolvedTokenKind::BinaryOpr(opr) => self.accept_binary_opr(*opr, token.token_idx()),
            ResolvedTokenKind::Prefix(opr) => self.accept_prefix_opr(*opr, token.token_idx()),
            ResolvedTokenKind::Suffix(opr) => self.accept_suffix_opr(*opr, token.token_idx()),
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

    fn accept_atom(&mut self, atom: Expr) {
        self.push_expr(atom)
    }

    fn accept_suffix_opr(&mut self, suffix: SuffixPunctuation, suffix_token_idx: TokenIdx) {
        self.synthesize_suffix(suffix, suffix_token_idx)
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
        self.synthesize_all_above(unfinished_expr.precedence());
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
        self.synthesize_all_above(Precedence::ListItem);
        self.push_unfinished_expr(UnfinishedExpr::ListItem {
            separator_token_idx,
        })
    }
}

// fn new_list_expr(
//     bracket: Bracket,
//     start_attr: ListStartAttr,
//     end_attr: ListEndAttr,
//     opds: ExprIdxRange,
// ) {
//     if bracket == Bracket::Par && start_attr == ListStartAttr::None && idx_arena::len(&opds) == 1 {
//         return Ok(Expr::Bracketed(opds.start()));
//     }
//     let opn_variant = Opn::List(match start_attr {
//         ListStartAttr::None => match bracket {
//             Bracket::Par => ListOpr::NewTuple,
//             Bracket::Box => ListOpr::NewVec,
//             Bracket::Curl => ListOpr::NewDict,
//             Bracket::Angle => todo!(),
//         },
//         ListStartAttr::Attach => match bracket {
//             Bracket::Par => ListOpr::FunctionCall,
//             Bracket::Box => match end_attr {
//                 ListEndAttr::None => {
//                     if idx_arena::len(&opds) < 2 {
//                         todo!()
//                         // return err!(format!("expect index expr inside `[]`"), range);
//                     }
//                     ListOpr::Index
//                 }
//                 ListEndAttr::Modulo => ListOpr::ModuloIndex,
//                 ListEndAttr::Attach => todo!(),
//             },
//             Bracket::Curl => ListOpr::StructInit,
//             Bracket::Angle => todo!(),
//         },
//         ListStartAttr::MethodAttach { ranged_ident } => ListOpr::MethodCall { ranged_ident },
//     });
//     Ok(Expr::Opn {
//         opn: opn_variant,
//         opds,
//     })
// }
