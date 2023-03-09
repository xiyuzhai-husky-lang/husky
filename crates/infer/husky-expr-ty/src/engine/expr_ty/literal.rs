use super::*;
use husky_token::FloatLiteral;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_literal_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        literal_token_idx: TokenIdx,
        expectation: &impl ExpectLocalTerm,
        local_term_region: &mut LocalTermRegion,
    ) -> Result<LocalTerm, ExprTypeError> {
        let literal_token = self.token_sheet_data[literal_token_idx];
        match literal_token {
            Token::Literal(literal) => match literal {
                Literal::Unit => todo!(),
                Literal::Char(_) => todo!(),
                Literal::String(_) => Ok(self.term_menu.static_str_ref().into()),
                Literal::Integer(integer_literal) => match integer_literal {
                    IntegerLikeLiteral::Unspecified => {
                        // match in the order of most frequent to least frequent
                        Ok(match expectation
                            .final_destination(self.db, local_term_region.unresolved_terms())
                        {
                            FinalDestination::TypePath(Right(PreludeTypePath::Num(
                                PreludeNumTypePath::Int(path),
                            ))) => match path {
                                PreludeIntTypePath::I32 => self.term_menu.i32(),
                                PreludeIntTypePath::I64 => self.term_menu.i64(),
                                PreludeIntTypePath::ISize => self.term_menu.isize(),
                                PreludeIntTypePath::I8 => self.term_menu.i8(),
                                PreludeIntTypePath::I16 => todo!(),
                                PreludeIntTypePath::I128 => todo!(),
                                PreludeIntTypePath::U8 => todo!(),
                                PreludeIntTypePath::U16 => todo!(),
                                PreludeIntTypePath::U32 => todo!(),
                                PreludeIntTypePath::U64 => todo!(),
                                PreludeIntTypePath::U128 => todo!(),
                                PreludeIntTypePath::USize => todo!(),
                                PreludeIntTypePath::R8 => todo!(),
                                PreludeIntTypePath::R16 => todo!(),
                                PreludeIntTypePath::R32 => todo!(),
                                PreludeIntTypePath::R64 => todo!(),
                                PreludeIntTypePath::R128 => todo!(),
                                PreludeIntTypePath::RSize => todo!(),
                            },
                            FinalDestination::TypePath(_)
                            | FinalDestination::AnyOriginal
                            | FinalDestination::AnyDerived => {
                                return Ok(local_term_region
                                    .new_implicit_symbol(
                                        expr_idx,
                                        ImplicitSymbolVariant::UnspecifiedIntegerType,
                                    )
                                    .into())
                            }
                            FinalDestination::Sort => todo!(),
                        }
                        .into())
                    }
                    IntegerLikeLiteral::I8(_) => todo!(),
                    IntegerLikeLiteral::I16(_) => todo!(),
                    IntegerLikeLiteral::I32(_) => Ok(self.term_menu.i32().into()),
                    IntegerLikeLiteral::I64(_) => todo!(),
                    IntegerLikeLiteral::I128(_) => todo!(),
                    IntegerLikeLiteral::ISize(_) => todo!(),
                    IntegerLikeLiteral::R8(_) => todo!(),
                    IntegerLikeLiteral::R16(_) => todo!(),
                    IntegerLikeLiteral::R32(_) => Ok(self.term_menu.r32().into()),
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
                    FloatLiteral::Unspecified => match expectation
                        .final_destination(self.db, local_term_region.unresolved_terms())
                    {
                        FinalDestination::TypePath(Right(PreludeTypePath::Num(
                            PreludeNumTypePath::Float(path),
                        ))) => match path {
                            PreludeFloatTypePath::F32 => Ok(self.term_menu.f32().into()),
                            PreludeFloatTypePath::F64 => Ok(self.term_menu.f64().into()),
                        },
                        FinalDestination::TypePath(_)
                        | FinalDestination::AnyOriginal
                        | FinalDestination::AnyDerived => Ok(local_term_region
                            .new_implicit_symbol(
                                expr_idx,
                                ImplicitSymbolVariant::UnspecifiedFloatType,
                            )
                            .into()),
                        FinalDestination::Sort => todo!(),
                    },
                    FloatLiteral::F32(_) => todo!(),
                    FloatLiteral::F64(_) => todo!(),
                },
                Literal::TupleIndex(_) => todo!(),
                Literal::Bool(_) => Ok(self.term_menu.bool().into()),
            },
            _ => unreachable!(),
        }
    }
}
