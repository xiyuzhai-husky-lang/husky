use super::*;
use husky_entity_path::path::major_item::ty::PreludeIntTypePath;
use husky_fly_term::quary::FlyQuary;
use husky_regional_token::RegionalTokenIdx;
use husky_token_data::FloatLiteralTokenData;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn calc_literal_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        literal_token_idx: RegionalTokenIdx,
        expectation: &impl ExpectFlyTerm,
    ) -> SemExprTypeResult<FlyTerm> {
        self.calc_literal_expr_ty_aux(expr_idx, literal_token_idx, expectation)
            .map(|term| term.with_quary(FlyQuary::Compterm))
    }

    fn calc_literal_expr_ty_aux(
        &mut self,
        expr_idx: SynExprIdx,
        literal_token_idx: RegionalTokenIdx,
        expectation: &impl ExpectFlyTerm,
    ) -> SemExprTypeResult<FlyTerm> {
        let literal_token: TokenData = self.token_data(literal_token_idx);
        match literal_token {
            TokenData::Literal(literal) => match literal {
                LiteralTokenData::Unit => todo!(),
                LiteralTokenData::Char(_) => todo!(),
                LiteralTokenData::String(_) => Ok(self.term_menu().static_str_ref().into()),
                LiteralTokenData::Integer(integer_literal) => {
                    match integer_literal {
                        IntegerLikeLiteralTokenData::UnspecifiedRegular(_)
                        | IntegerLikeLiteralTokenData::UnspecifiedLarge() => {
                            // match in the order of most frequent to least frequent
                            Ok(match expectation.destination() {
                                FlyTermDestination::Specific(destination) => {
                                    match destination.base_term_data(self) {
                                        FlyTermData::TypeOntology {
                                            refined_ty_path:
                                                Left(PreludeTypePath::Num(PreludeNumTypePath::Int(
                                                    path,
                                                ))),
                                            ..
                                        } => match path {
                                            PreludeIntTypePath::I32 => {
                                                self.term_menu().i32_ty_ontology()
                                            }
                                            PreludeIntTypePath::I64 => {
                                                self.term_menu().i64_ty_ontology()
                                            }
                                            PreludeIntTypePath::ISize => {
                                                self.term_menu().isize_ty_ontology()
                                            }
                                            PreludeIntTypePath::I8 => {
                                                self.term_menu().i8_ty_ontology()
                                            }
                                            PreludeIntTypePath::I16 => {
                                                self.term_menu().i16_ty_ontology()
                                            }
                                            PreludeIntTypePath::I128 => {
                                                self.term_menu().i128_ty_ontology()
                                            }
                                            PreludeIntTypePath::U8 => {
                                                self.term_menu().u8_ty_ontology()
                                            }
                                            PreludeIntTypePath::U16 => {
                                                self.term_menu().u16_ty_ontology()
                                            }
                                            PreludeIntTypePath::U32 => {
                                                self.term_menu().u32_ty_ontology()
                                            }
                                            PreludeIntTypePath::U64 => {
                                                self.term_menu().u64_ty_ontology()
                                            }
                                            PreludeIntTypePath::U128 => {
                                                self.term_menu().u128_ty_ontology()
                                            }
                                            PreludeIntTypePath::USize => {
                                                self.term_menu().usize_ty_ontology()
                                            }
                                            PreludeIntTypePath::R8 => {
                                                self.term_menu().r8_ty_ontology()
                                            }
                                            PreludeIntTypePath::R16 => {
                                                self.term_menu().r16_ty_ontology()
                                            }
                                            PreludeIntTypePath::R32 => {
                                                self.term_menu().r32_ty_ontology()
                                            }
                                            PreludeIntTypePath::R64 => {
                                                self.term_menu().r64_ty_ontology()
                                            }
                                            PreludeIntTypePath::R128 => {
                                                self.term_menu().r128_ty_ontology()
                                            }
                                            PreludeIntTypePath::RSize => {
                                                self.term_menu().rsize_ty_ontology()
                                            }
                                        },
                                        FlyTermData::Hole(
                                            HoleKind::UnspecifiedIntegerType,
                                            destination,
                                        ) => return Ok(destination.into()),
                                        _ => {
                                            return Ok(self
                                                .new_hole(
                                                    expr_idx,
                                                    HoleKind::UnspecifiedIntegerType,
                                                )
                                                .into())
                                        }
                                    }
                                }
                                FlyTermDestination::AnyOriginal
                                | FlyTermDestination::AnyDerived => {
                                    return Ok(self
                                        .new_hole(expr_idx, HoleKind::UnspecifiedIntegerType)
                                        .into())
                                }
                            }
                            .into())
                        }
                        IntegerLikeLiteralTokenData::I8(_) => {
                            Ok(self.term_menu().i8_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::I16(_) => {
                            Ok(self.term_menu().i16_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::I32(_) => {
                            Ok(self.term_menu().i32_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::I64(_) => {
                            Ok(self.term_menu().i64_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::I128(_) => {
                            Ok(self.term_menu().i128_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::ISize(_) => {
                            Ok(self.term_menu().isize_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::R8(_) => {
                            Ok(self.term_menu().r8_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::R16(_) => {
                            Ok(self.term_menu().r16_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::R32(_) => {
                            Ok(self.term_menu().r32_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::R64(_) => {
                            Ok(self.term_menu().r64_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::R128(_) => {
                            Ok(self.term_menu().r128_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::RSize(_) => {
                            Ok(self.term_menu().rsize_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::U8(_) => {
                            Ok(self.term_menu().u8_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::U16(_) => {
                            Ok(self.term_menu().u16_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::U32(_) => {
                            Ok(self.term_menu().u32_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::U64(_) => {
                            Ok(self.term_menu().u64_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::U128(_) => {
                            Ok(self.term_menu().u128_ty_ontology().into())
                        }
                        IntegerLikeLiteralTokenData::USize(_) => {
                            Ok(self.term_menu().usize_ty_ontology().into())
                        }
                    }
                }
                LiteralTokenData::Float(float_literal) => match float_literal {
                    FloatLiteralTokenData::Unspecified(_) => match expectation.destination() {
                        FlyTermDestination::Specific(destination) => {
                            match destination.base_term_data(self) {
                                FlyTermData::TypeOntology {
                                    refined_ty_path:
                                        Left(PreludeTypePath::Num(PreludeNumTypePath::Float(path))),
                                    ..
                                } => match path {
                                    PreludeFloatTypePath::F32 => {
                                        Ok(self.term_menu().f32_ty_ontology().into())
                                    }
                                    PreludeFloatTypePath::F64 => {
                                        Ok(self.term_menu().f64_ty_ontology().into())
                                    }
                                },
                                FlyTermData::Hole(HoleKind::UnspecifiedFloatType, destination) => {
                                    return Ok(destination.into())
                                }
                                _ => {
                                    return Ok(self
                                        .new_hole(expr_idx, HoleKind::UnspecifiedFloatType)
                                        .into())
                                }
                            }
                        }
                        FlyTermDestination::AnyOriginal | FlyTermDestination::AnyDerived => {
                            return Ok(self
                                .new_hole(expr_idx, HoleKind::UnspecifiedFloatType)
                                .into())
                        }
                    },
                    FloatLiteralTokenData::F32(_) => Ok(self.term_menu().f32_ty_ontology().into()),
                    FloatLiteralTokenData::F64(_) => Ok(self.term_menu().f64_ty_ontology().into()),
                },
                LiteralTokenData::Bool(_) => Ok(self.term_menu().bool_ty_ontology().into()),
            },
            _ => unreachable!(),
        }
    }
}
