use super::*;
use either::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb)]
pub struct DeclarativeParenicParameters {
    data: SmallVec<[DeclarativeTermRitchieParameter; 4]>,
}

impl std::ops::Deref for DeclarativeParenicParameters {
    type Target = [DeclarativeTermRitchieParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DeclarativeParenicParameters {
    pub(crate) fn from_decl(
        parameters: &[SpecificParameterDecl],
        expr_region_data: &SynExprRegionData,
        signature_region: &DeclarativeTermRegion,
    ) -> DeclarativeSignatureResult<Self> {
        Ok(Self {
            data: parameters
                .iter()
                .enumerate()
                .map(|(i, parameter)| {
                    Ok(match parameter {
                        SpecificParameterDecl::Regular {
                            pattern,
                            variables,
                            colon,
                            ty,
                        } => DeclarativeTermRitchieRegularParameter::new(
                            expr_region_data.pattern_contract(*pattern),
                            signature_region.expr_term(*ty).map_err(|_| {
                                DeclarativeSignatureError::ParameterTypeDeclarativeTermError(
                                    i.try_into().unwrap(),
                                )
                            })?,
                        )
                        .into(),
                        SpecificParameterDecl::Variadic {
                            symbol_modifier_keyword_group,
                            ty,
                            ..
                        } => DeclarativeTermRitchieVariadicParameter::new(
                            Contract::new(*symbol_modifier_keyword_group),
                            signature_region.expr_term(*ty).map_err(|_| {
                                DeclarativeSignatureError::ParameterTypeDeclarativeTermError(
                                    i.try_into().unwrap(),
                                )
                            })?,
                        )
                        .into(),
                        SpecificParameterDecl::Keyed {
                            symbol_modifier_keyword_group,
                            ident_token,
                            ty,
                            default,
                            ..
                        } => DeclarativeTermRitchieKeyedParameter::new(
                            ident_token.ident(),
                            Contract::new(*symbol_modifier_keyword_group),
                            signature_region.expr_term(*ty).map_err(|_| {
                                DeclarativeSignatureError::ParameterTypeDeclarativeTermError(
                                    i.try_into().unwrap(),
                                )
                            })?,
                            match *default {
                                Left(_) => todo!(),
                                Right(default_expr_idx) => Some(
                                    signature_region.expr_term(default_expr_idx).map_err(|_| {
                                        DeclarativeSignatureError::ParameterTypeDeclarativeTermError(
                                            i.try_into().unwrap(),
                                        )
                                    })?,
                                ),
                            },
                        )
                        .into(),
                    })
                })
                .collect::<DeclarativeSignatureResult<_>>()?,
        })
    }

    pub fn data(&self) -> &[DeclarativeTermRitchieParameter] {
        &self.data
    }
}
