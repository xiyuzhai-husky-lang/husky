mod keyed;
mod regular;
mod variadic;

pub use self::keyed::*;
pub use self::regular::*;
pub use self::variadic::*;

use super::*;
use either::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum DeclarativeSpecificParameter {
    Regular(SpecificRegularParameterDeclarativeSignatureTemplate),
    Variadic(SpecificVariadicParameterDeclarativeSignatureTemplate),
    Keyed(SpecificKeyedParameterDeclarativeSignatureTemplate),
}

impl DeclarativeSpecificParameter {
    pub fn into_ritchie_parameter_contracted_ty(self) -> DeclarativeTermRitchieParameter {
        match self {
            DeclarativeSpecificParameter::Regular(signature_template) => {
                DeclarativeTermRitchieRegularParameter::new(
                    signature_template.contract(),
                    signature_template.ty(),
                )
                .into()
            }
            DeclarativeSpecificParameter::Variadic(signature_template) => {
                DeclarativeTermRitchieVariadicParameter::new(
                    signature_template.contract(),
                    signature_template.ty(),
                )
                .into()
            }
            DeclarativeSpecificParameter::Keyed(signature_template) => {
                DeclarativeTermRitchieKeyedParameter::new(
                    signature_template.key(),
                    signature_template.contract(),
                    signature_template.ty(),
                    signature_template.default(),
                )
                .into()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DeclarativeSpecificParameters {
    data: SmallVec<[DeclarativeSpecificParameter; 4]>,
}

impl std::ops::Deref for DeclarativeSpecificParameters {
    type Target = [DeclarativeSpecificParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DeclarativeSpecificParameters {
    pub(crate) fn from_decl(
        parameters: &[SpecificParameterDecl],
        expr_region_data: &ExprRegionData,
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
                        } => SpecificRegularParameterDeclarativeSignatureTemplate::new(
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
                        } => SpecificVariadicParameterDeclarativeSignatureTemplate::new(
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
                        } => SpecificKeyedParameterDeclarativeSignatureTemplate::new(
                            ident_token.ident(),
                            Contract::new(*symbol_modifier_keyword_group),
                            signature_region.expr_term(*ty).map_err(|_| {
                                DeclarativeSignatureError::ParameterTypeDeclarativeTermError(
                                    i.try_into().unwrap(),
                                )
                            })?,
                            match *default {
                                Left(_) => todo!(),
                                Right(default_expr_idx) => Some(signature_region.expr_term(default_expr_idx).map_err(|_| {
                                    DeclarativeSignatureError::ParameterTypeDeclarativeTermError(
                                        i.try_into().unwrap(),
                                    )
                                })?),
                            },
                        )
                        .into(),
                    })
                })
                .collect::<DeclarativeSignatureResult<_>>()?,
        })
    }
}
