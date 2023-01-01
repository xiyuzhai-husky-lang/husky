use super::*;

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(super) fn synthesize_all_above(&mut self, threshold: Precedence) {
        while let Some(stack_opr) = self.last_unfinished_expr() {
            if stack_opr.precedence() >= threshold {
                let stack_opr = self.pop_opr().unwrap();
                match stack_opr {
                    UnfinishedExpr::Binary { binary, .. } => self.synthesize_binary(binary),
                    UnfinishedExpr::Prefix {
                        prefix,
                        prefix_token_idx,
                    } => self.synthesize_prefix(prefix, prefix_token_idx),
                    UnfinishedExpr::LambdaHead { inputs, start } => {
                        self.synthesize_lambda(inputs, start)
                    }
                    UnfinishedExpr::ListItem { .. } => {
                        let (_bra, bra_token) = loop {
                            if let Some(opr) = self.pop_opr() {
                                match opr {
                                    UnfinishedExpr::List {
                                        bra,
                                        bra_token_idx: bra_token,
                                        ..
                                    } => break (bra, bra_token),
                                    _ => (),
                                }
                            } else {
                                todo!()
                                // return err!(
                                //     format!("improper use of `,`"),
                                //     (position..(position.to_right(1))).into()
                                // );
                            }
                        };
                        todo!()
                        // return Err(AstError {
                        //     variant: AstErrorVariant::Original {
                        //         message: format!("expect a matching `{}`", bra.ket_code()),
                        //         range: (start..(start.to_right(1))).into(),
                        //     },
                        //     dev_src: dev_src!(),
                        // });
                    }
                    UnfinishedExpr::List { .. } => {
                        todo!()
                        // return Err(AstError {
                        //     variant: AstErrorVariant::Original {
                        //         message: format!("expect a matching `{}`", bra.ket_code()),
                        //         range: (start..(start.to_right(1))).into(),
                        //     },
                        //     dev_src: dev_src!(),
                        // })
                    }
                    UnfinishedExpr::Dot { dot_token_idx } => todo!(),
                }
            } else {
                return;
            }
        }
    }

    fn synthesize_binary(&mut self, binary: BinaryPunctuation) {
        todo!()
        // use husky_text::HasTextRange;
        // let _len = self.stack.number_of_exprs();
        // let range = self.stack.topk_exprs(2).text_range();
        // self.synthesize_opn(binary.into(), 2, range)
    }

    fn synthesize_prefix(&mut self, prefix: PrefixPunctuation, prefix_token_idx: TokenIdx) {
        todo!()
        // let range = (start..self.stack.top_expr().unwrap().range.end).into();
        // self.synthesize_opn(prefix.into(), 1, range)
    }

    fn synthesize_field_access(&mut self, _field_ident: RangedIdentifier, _end: TextPosition) {
        todo!()
        // let range = (self.exprs.last().unwrap().range.start..end).into();
        // self.synthesize_opn(
        //     RawOpnVariant::Field(field_ident),
        //     Default::default(),
        //     1,
        //     range,
        // )
    }

    // fn synthesize_opn(&mut self, opn: Opn, nopds: usize) {
    //     let opds = self.take_opds(nopds);
    //     self.push_expr(Expr::Opn { opn, opds });
    // }

    // fn take_opds(&mut self, nopds: usize) -> ExprIdxRange {
    //     let (drained_exprs, paths) = self.drain_exprs(nopds);
    //     self.sheet.alloc_expr_batch(drained_exprs, paths)
    // }

    fn synthesize_lambda(
        &mut self,
        _inputs: Vec<(RangedIdentifier, Option<ExprIdx>)>,
        _start: TextPosition,
    ) {
        todo!()
        // let range = (start..self.exprs.last().unwrap().range.end).into();
        // let lambda_expr = RawExpr {
        //     range,
        //     variant: RawExprVariant::Lambda(
        //         inputs,
        //         self.arena.alloc_one(self.exprs.pop().unwrap()),
        //     ),
        // };
        // self.exprs.push(lambda_expr);
    }
}
