use super::*;
use husky_regional_token::RegionalTokenIdx;
use husky_token_data::FloatLiteralData;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_literal_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        literal_token_idx: RegionalTokenIdx,
        expectation: &impl ExpectFluffyTerm,
    ) -> SemaExprResult<FluffyTerm> {
        let literal_token: TokenData = self.token_data(literal_token_idx);
        match literal_token {
            TokenData::Literal(literal) => match literal {
                LiteralData::Unit => todo!(),
                LiteralData::Char(_) => todo!(),
                LiteralData::String(_) => Ok(self.term_menu.static_str_ref().into()),
                LiteralData::Integer(integer_literal) => match integer_literal {
                    IntegerLikeLiteralData::UnspecifiedRegular(_)
                    | IntegerLikeLiteralData::UnspecifiedLarge() => {
                        // match in the order of most frequent to least frequent
                        Ok(match expectation
                            .destination()
                            .map(|destination| destination.data(self))
                        {
                            Some(FluffyTermData::TypeOntology {
                                refined_ty_path:
                                    Left(PreludeTypePath::Num(PreludeNumTypePath::Int(path))),
                                ..
                            }) => match path {
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
                            Some(FluffyTermData::Hole(
                                HoleKind::UnspecifiedIntegerType,
                                destination,
                            )) => return Ok(destination.into()),
                            _ => {
                                return Ok(self
                                    .new_hole(expr_idx, HoleKind::UnspecifiedIntegerType)
                                    .into())
                            }
                        }
                        .into())
                    }
                    IntegerLikeLiteralData::I8(_) => Ok(self.term_menu.i8_ty_ontology().into()),
                    IntegerLikeLiteralData::I16(_) => Ok(self.term_menu.i16_ty_ontology().into()),
                    IntegerLikeLiteralData::I32(_) => Ok(self.term_menu.i32_ty_ontology().into()),
                    IntegerLikeLiteralData::I64(_) => Ok(self.term_menu.i64_ty_ontology().into()),
                    IntegerLikeLiteralData::I128(_) => Ok(self.term_menu.i128_ty_ontology().into()),
                    IntegerLikeLiteralData::ISize(_) => {
                        Ok(self.term_menu.isize_ty_ontology().into())
                    }
                    IntegerLikeLiteralData::R8(_) => Ok(self.term_menu.r8_ty_ontology().into()),
                    IntegerLikeLiteralData::R16(_) => Ok(self.term_menu.r16_ty_ontology().into()),
                    IntegerLikeLiteralData::R32(_) => Ok(self.term_menu.r32_ty_ontology().into()),
                    IntegerLikeLiteralData::R64(_) => Ok(self.term_menu.r64_ty_ontology().into()),
                    IntegerLikeLiteralData::R128(_) => Ok(self.term_menu.r128_ty_ontology().into()),
                    IntegerLikeLiteralData::RSize(_) => {
                        Ok(self.term_menu.rsize_ty_ontology().into())
                    }
                    IntegerLikeLiteralData::U8(_) => Ok(self.term_menu.u8_ty_ontology().into()),
                    IntegerLikeLiteralData::U16(_) => Ok(self.term_menu.u16_ty_ontology().into()),
                    IntegerLikeLiteralData::U32(_) => Ok(self.term_menu.u32_ty_ontology().into()),
                    IntegerLikeLiteralData::U64(_) => Ok(self.term_menu.u64_ty_ontology().into()),
                    IntegerLikeLiteralData::U128(_) => Ok(self.term_menu.u128_ty_ontology().into()),
                    IntegerLikeLiteralData::USize(_) => {
                        Ok(self.term_menu.usize_ty_ontology().into())
                    }
                },
                LiteralData::Float(float_literal) => match float_literal {
                    FloatLiteralData::Unspecified(_) => {
                        match expectation
                            .destination()
                            .map(|destination| destination.data(self))
                        {
                            Some(FluffyTermData::TypeOntology {
                                refined_ty_path:
                                    Left(PreludeTypePath::Num(PreludeNumTypePath::Float(path))),
                                ..
                            }) => match path {
                                PreludeFloatTypePath::F32 => {
                                    Ok(self.term_menu.f32_ty_ontology().into())
                                }
                                PreludeFloatTypePath::F64 => {
                                    Ok(self.term_menu.f64_ty_ontology().into())
                                }
                            },
                            Some(FluffyTermData::Hole(
                                HoleKind::UnspecifiedFloatType,
                                destination,
                            )) => return Ok(destination.into()),
                            _ => {
                                return Ok(self
                                    .new_hole(expr_idx, HoleKind::UnspecifiedFloatType)
                                    .into())
                            }
                        }
                    }
                    FloatLiteralData::F32(_) => todo!(),
                    FloatLiteralData::F64(_) => todo!(),
                },
                LiteralData::TupleIndex(_) => todo!(),
                LiteralData::Bool(_) => Ok(self.term_menu.bool_ty_ontology().into()),
            },
            _ => unreachable!(),
        }
    }
}
