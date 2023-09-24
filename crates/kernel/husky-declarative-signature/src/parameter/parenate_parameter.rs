use super::*;
use either::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb)]
pub struct DeclarativeParenateParameters {
    data: SmallVec<[DeclarativeTermRitchieParameter; 4]>,
}

impl std::ops::Deref for DeclarativeParenateParameters {
    type Target = [DeclarativeTermRitchieParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DeclarativeParenateParameters {
    pub(crate) fn from_decl(
        parameters: &[SpecificParameterObelisk],
        expr_region_data: &SynExprRegionData,
        signature_region: &DeclarativeTermRegion,
    ) -> DeclarativeSignatureResult<Self> {
        Ok(Self {
            data: parameters
                .iter()
                .enumerate()
                .map(|(i, parameter)| {
                    Ok(match parameter {
                        SpecificParameterObelisk::Regular {
                            syn_pattern_root,
                            variables,
                            colon,
                            ty,
                        } => DeclarativeTermRitchieRegularParameter::new(
                            expr_region_data.pattern_contract(syn_pattern_root.syn_pattern_expr_idx()),
                            signature_region.expr_term(*ty).map_err(|_| {
                                DeclarativeSignatureError::ParameterTypeDeclarativeTermError(
                                    i.try_into().unwrap(),
                                )
                            })?,
                        )
                        .into(),
                        SpecificParameterObelisk::Variadic {
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
                        SpecificParameterObelisk::Keyed {
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
