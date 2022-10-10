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
        todo!()
        // let len = self.exprs.len();
        // let range = (self.exprs[len - 2].range.start..self.exprs[len - 1].range.end).into();
        // self.synthesize_opn(binary.into(), Default::default(), 2, range)
    }

    fn synthesize_prefix(&mut self, prefix: PrefixOpr, start: TextPosition) {
        todo!()
        // let range = (start..self.exprs.last().unwrap().range.end).into();
        // if prefix == PrefixOpr::Minus {
        //     if let RawExprVariant::PrimitiveLiteral(lit) = self.exprs.last().unwrap().variant {
        //         self.exprs.pop();
        //         match lit {
        //             RawLiteralData::I32(i) => self.exprs.push(RawExpr {
        //                 range,
        //                 variant: RawExprVariant::PrimitiveLiteral(RawLiteralData::I32(-i)),
        //             }),
        //             RawLiteralData::F32(f) => self.exprs.push(RawExpr {
        //                 range,
        //                 variant: RawExprVariant::PrimitiveLiteral(RawLiteralData::F32(-f)),
        //             }),
        //             RawLiteralData::Void
        //             | RawLiteralData::B32(_)
        //             | RawLiteralData::Bool(_)
        //             | RawLiteralData::B64(_) => todo!(),
        //             RawLiteralData::Integer(i) => self.exprs.push(RawExpr {
        //                 range,
        //                 variant: RawExprVariant::PrimitiveLiteral(RawLiteralData::Integer(-i)),
        //             }),
        //             RawLiteralData::I64(_) => todo!(),
        //             RawLiteralData::Float(f) => self.exprs.push(RawExpr {
        //                 range,
        //                 variant: RawExprVariant::PrimitiveLiteral(RawLiteralData::Float(-f)),
        //             }),
        //             RawLiteralData::F64(_) => todo!(),
        //             RawLiteralData::Bits(_) => todo!(),
        //         }
        //         return;
        //     }
        // }
        // self.synthesize_opn(prefix.into(), Default::default(), 1, range)
    }

    fn synthesize_suffix(&mut self, suffix: RawSuffixOpr, end: TextPosition) {
        todo!()
        // let range = (self.exprs.last().unwrap().range.start..end).into();
        // self.synthesize_opn(suffix.into(), Default::default(), 1, range)
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
        todo!()
        // let len = self.exprs.len();
        // let opds = self.arena.alloc(self.exprs[(len - n_opds)..len].into());
        // self.exprs.truncate(len - n_opds);
        // self.exprs.push(RawExpr {
        //     range,
        //     variant: RawExprVariant::Opn { opn_variant, opds },
        // });
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
