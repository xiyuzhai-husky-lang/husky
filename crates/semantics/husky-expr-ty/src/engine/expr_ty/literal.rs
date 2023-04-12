use super::*;
use husky_token::FloatLiteral;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_literal_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        literal_token_idx: TokenIdx,
        expectation: &impl ExpectLocalTerm,
    ) -> Result<FluffyTerm, ExprTypeError> {
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
                            .destination()
                            .map(|destination| (destination, destination.data(self)))
                        {
                            Some((
                                _,
                                FluffyTermData::TypeOntology {
                                    refined_path:
                                        Right(PreludeTypePath::Num(PreludeNumTypePath::Int(path))),
                                    ..
                                },
                            )) => match path {
                                PreludeIntTypePath::I32 => self.term_menu.i32_ty_ontology(),
                                PreludeIntTypePath::I64 => self.term_menu.i64_ty_ontology(),
                                PreludeIntTypePath::ISize => self.term_menu.isize_ty_ontology(),
                                PreludeIntTypePath::I8 => self.term_menu.i8_ty_ontology(),
                                PreludeIntTypePath::I16 => self.term_menu.i16_ty_ontology(),
                                PreludeIntTypePath::I128 => self.term_menu.i128_ty_ontology(),
                                PreludeIntTypePath::U8 => self.term_menu.u8_ty_ontology(),
                                PreludeIntTypePath::U16 => self.term_menu.u16_ty_ontology(),
                                PreludeIntTypePath::U32 => self.term_menu.u32_ty_ontology(),
                                PreludeIntTypePath::U64 => self.term_menu.u64_ty_ontology(),
                                PreludeIntTypePath::U128 => self.term_menu.u128_ty_ontology(),
                                PreludeIntTypePath::USize => self.term_menu.usize_ty_ontology(),
                                PreludeIntTypePath::R8 => self.term_menu.r8_ty_ontology(),
                                PreludeIntTypePath::R16 => self.term_menu.r16_ty_ontology(),
                                PreludeIntTypePath::R32 => self.term_menu.r32_ty_ontology(),
                                PreludeIntTypePath::R64 => self.term_menu.r64_ty_ontology(),
                                PreludeIntTypePath::R128 => self.term_menu.r128_ty_ontology(),
                                PreludeIntTypePath::RSize => self.term_menu.rsize_ty_ontology(),
                            },
                            Some((
                                destination,
                                FluffyTermData::Hole(HoleKind::UnspecifiedIntegerType, _),
                            )) => return Ok(destination),
                            _ => {
                                return Ok(self
                                    .new_hole(expr_idx, HoleKind::UnspecifiedIntegerType)
                                    .into())
                            }
                        }
                        .into())
                    }
                    IntegerLikeLiteral::I8(_) => Ok(self.term_menu.i8_ty_ontology().into()),
                    IntegerLikeLiteral::I16(_) => Ok(self.term_menu.i16_ty_ontology().into()),
                    IntegerLikeLiteral::I32(_) => Ok(self.term_menu.i32_ty_ontology().into()),
                    IntegerLikeLiteral::I64(_) => Ok(self.term_menu.i64_ty_ontology().into()),
                    IntegerLikeLiteral::I128(_) => Ok(self.term_menu.i128_ty_ontology().into()),
                    IntegerLikeLiteral::ISize(_) => Ok(self.term_menu.isize_ty_ontology().into()),
                    IntegerLikeLiteral::R8(_) => Ok(self.term_menu.r8_ty_ontology().into()),
                    IntegerLikeLiteral::R16(_) => Ok(self.term_menu.r16_ty_ontology().into()),
                    IntegerLikeLiteral::R32(_) => Ok(self.term_menu.r32_ty_ontology().into()),
                    IntegerLikeLiteral::R64(_) => Ok(self.term_menu.r64_ty_ontology().into()),
                    IntegerLikeLiteral::R128(_) => Ok(self.term_menu.r128_ty_ontology().into()),
                    IntegerLikeLiteral::RSize(_) => Ok(self.term_menu.rsize_ty_ontology().into()),
                    IntegerLikeLiteral::U8(_) => Ok(self.term_menu.u8_ty_ontology().into()),
                    IntegerLikeLiteral::U16(_) => Ok(self.term_menu.u16_ty_ontology().into()),
                    IntegerLikeLiteral::U32(_) => Ok(self.term_menu.u32_ty_ontology().into()),
                    IntegerLikeLiteral::U64(_) => Ok(self.term_menu.u64_ty_ontology().into()),
                    IntegerLikeLiteral::U128(_) => Ok(self.term_menu.u128_ty_ontology().into()),
                    IntegerLikeLiteral::USize(_) => Ok(self.term_menu.usize_ty_ontology().into()),
                },
                Literal::Float(float_literal) => match float_literal {
                    FloatLiteral::Unspecified => {
                        match expectation
                            .destination()
                            .map(|destination| destination.data(self))
                        {
                            Some(FluffyTermData::TypeOntology {
                                refined_path:
                                    Right(PreludeTypePath::Num(PreludeNumTypePath::Float(path))),
                                ..
                            }) => match path {
                                PreludeFloatTypePath::F32 => {
                                    Ok(self.term_menu.f32_ty_ontology().into())
                                }
                                PreludeFloatTypePath::F64 => {
                                    Ok(self.term_menu.f64_ty_ontology().into())
                                }
                            },
                            Some(FluffyTermData::Hole(HoleKind::UnspecifiedFloatType, idx)) => {
                                return Ok((*idx).into())
                            }
                            _ => todo!(),
                            // Ok(self
                            //     .fluffy_term_region
                            //     .new_implicit_symbol(
                            //         expr_idx,
                            //         ImplicitSymbolVariant::UnspecifiedFloatType,
                            //     )
                            //     .into()),
                        }
                    }
                    FloatLiteral::F32(_) => todo!(),
                    FloatLiteral::F64(_) => todo!(),
                },
                Literal::TupleIndex(_) => todo!(),
                Literal::Bool(_) => Ok(self.term_menu.bool_ty_ontology().into()),
            },
            _ => unreachable!(),
        }
    }
}
