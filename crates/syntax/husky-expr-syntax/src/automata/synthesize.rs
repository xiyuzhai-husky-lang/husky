use thin_vec::ThinVec;

use super::*;

impl<'a> Automata<'a> {
    pub(super) fn synthesize_all_above(&mut self, threshold: Precedence) -> ExprSyntaxResult<()> {
        while let Some(stack_opr) = self.stack.top_opr() {
            if stack_opr.precedence() >= threshold {
                let stack_opr = self.stack.pop_opr().unwrap();
                match stack_opr.variant {
                    OnStackOprVariant::Binary(binary) => self.synthesize_binary(binary),
                    OnStackOprVariant::Prefix { prefix, start } => {
                        self.synthesize_prefix(prefix, start)
                    }
                    OnStackOprVariant::LambdaHead { inputs, start } => {
                        self.synthesize_lambda(inputs, start)
                    }
                    OnStackOprVariant::ListItem(position) => {
                        let (bra, start) = loop {
                            if let Some(opr) = self.stack.pop_opr() {
                                match opr.variant {
                                    OnStackOprVariant::ListStart { bra, start, .. } => {
                                        break (bra, start)
                                    }
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
                    OnStackOprVariant::ListStart { bra, start, .. } => {
                        todo!()
                        // return Err(AstError {
                        //     variant: AstErrorVariant::Original {
                        //         message: format!("expect a matching `{}`", bra.ket_code()),
                        //         range: (start..(start.to_right(1))).into(),
                        //     },
                        //     dev_src: dev_src!(),
                        // })
                    }
                }
            } else {
                return Ok(());
            }
        }
        Ok(())
    }

    fn synthesize_binary(&mut self, binary: BinaryOpr) {
        use husky_text::TextRanged;
        let len = self.stack.number_of_exprs();
        let range = self.stack.topk_exprs(2).text_range();
        self.synthesize_opn(binary.into(), 2, range)
    }

    fn synthesize_prefix(&mut self, prefix: PrefixOpr, start: TextPosition) {
        let range = (start..self.stack.top_expr().unwrap().range.end).into();
        self.synthesize_opn(prefix.into(), 1, range)
    }

    pub(super) fn synthesize_suffix(&mut self, suffix: RawSuffixOpr, end: TextPosition) {
        let range = (self.stack.top_expr().unwrap().text_start()..end).into();
        self.synthesize_opn(suffix.into(), 1, range)
    }

    fn synthesize_field_access(&mut self, field_ident: RangedCustomIdentifier, end: TextPosition) {
        todo!()
        // let range = (self.exprs.last().unwrap().range.start..end).into();
        // self.synthesize_opn(
        //     RawOpnVariant::Field(field_ident),
        //     Default::default(),
        //     1,
        //     range,
        // )
    }

    fn synthesize_opn(&mut self, opn_variant: RawOpnVariant, n_opds: usize, range: TextRange) {
        let len = self.stack.number_of_exprs();
        let opds = self.arena.alloc(self.stack.drain_exprs(n_opds).into());
        self.stack.push_expr(RawExpr::new(
            RawExprVariant::Opn { opn_variant, opds },
            range,
            self.arena,
        ));
    }

    fn synthesize_lambda(
        &mut self,
        inputs: Vec<(RangedCustomIdentifier, Option<RawExprIdx>)>,
        start: TextPosition,
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
