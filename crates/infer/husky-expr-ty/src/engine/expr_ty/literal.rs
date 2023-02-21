use super::*;
use husky_token::FloatLiteral;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_literal(
        &mut self,
        expr_idx: ExprIdx,
        literal_token_idx: TokenIdx,
        expectation: &impl ExpectLocalTerm,
    ) -> Result<LocalTerm, ExprTypeError> {
        let literal_token = self.token_sheet_data[literal_token_idx];
        match literal_token {
            Token::Literal(literal) => match literal {
                Literal::Unit => todo!(),
                Literal::Char(_) => todo!(),
                Literal::String(_) => Ok(self.reduced_term_menu.static_str_ref().into()),
                Literal::Integer(integer_literal) => match integer_literal {
                    IntegerLikeLiteral::Unspecified => match expectation.destination() {
                        // MOM
                        Some(term) if term == self.reduced_term_menu.i32().into() => {
                            Ok(self.reduced_term_menu.i32().into())
                        }
                        Some(term) if term == self.reduced_term_menu.i64().into() => todo!(),
                        Some(term) if term == self.reduced_term_menu.i8().into() => todo!(),
                        Some(term) if term == self.reduced_term_menu.i16().into() => todo!(),
                        Some(term) if term == self.reduced_term_menu.i64().into() => todo!(),
                        Some(term) if term == self.reduced_term_menu.i64().into() => todo!(),
                        Some(term) if term == self.reduced_term_menu.i64().into() => todo!(),
                        Some(term) if term == self.reduced_term_menu.i64().into() => todo!(),
                        Some(term) if term == self.reduced_term_menu.i64().into() => todo!(),
                        Some(term) if term == self.reduced_term_menu.i64().into() => todo!(),
                        Some(term) if term == self.reduced_term_menu.i64().into() => todo!(),
                        Some(term) if term == self.reduced_term_menu.i64().into() => todo!(),
                        Some(term) if term == self.reduced_term_menu.i64().into() => todo!(),
                        _ => Ok(self
                            .new_implicit_symbol(
                                expr_idx,
                                ImplicitSymbolVariant::UnspecifiedIntegerType,
                            )
                            .into()),
                    },
                    IntegerLikeLiteral::I8(_) => todo!(),
                    IntegerLikeLiteral::I16(_) => todo!(),
                    IntegerLikeLiteral::I32(_) => Ok(self.reduced_term_menu.i32().into()),
                    IntegerLikeLiteral::I64(_) => todo!(),
                    IntegerLikeLiteral::I128(_) => todo!(),
                    IntegerLikeLiteral::ISize(_) => todo!(),
                    IntegerLikeLiteral::R8(_) => todo!(),
                    IntegerLikeLiteral::R16(_) => todo!(),
                    IntegerLikeLiteral::R32(_) => Ok(self.reduced_term_menu.r32().into()),
                    IntegerLikeLiteral::R64(_) => todo!(),
                    IntegerLikeLiteral::R128(_) => todo!(),
                    IntegerLikeLiteral::RSize(_) => todo!(),
                    IntegerLikeLiteral::U8(_) => todo!(),
                    IntegerLikeLiteral::U16(_) => todo!(),
                    IntegerLikeLiteral::U32(_) => todo!(),
                    IntegerLikeLiteral::U64(_) => todo!(),
                    IntegerLikeLiteral::U128(_) => todo!(),
                    IntegerLikeLiteral::USize(_) => todo!(),
                },
                Literal::Float(float_literal) => match float_literal {
                    FloatLiteral::Unspecified => match expectation.destination() {
                        Some(term) if term == self.reduced_term_menu.f32().into() => {
                            Ok(self.reduced_term_menu.f32().into())
                        }
                        Some(term) if term == self.reduced_term_menu.f64().into() => {
                            Ok(self.reduced_term_menu.f64().into())
                        }
                        Some(_) => todo!(),
                        None => Ok(self
                            .new_implicit_symbol(
                                expr_idx,
                                ImplicitSymbolVariant::UnspecifiedFloatType,
                            )
                            .into()),
                    },
                    FloatLiteral::F32(_) => todo!(),
                    FloatLiteral::F64(_) => todo!(),
                },
                Literal::TupleIndex(_) => todo!(),
                Literal::Bool(_) => Ok(self.reduced_term_menu.bool().into()),
            },
            _ => unreachable!(),
        }
    }
}
