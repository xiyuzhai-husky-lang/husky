use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitRegularParameterDeclarativeSignature {
    contract: Contract,
    ty: DeclarativeTerm,
}

impl ExplicitRegularParameterDeclarativeSignature {
    pub(crate) fn new(contract: Contract, ty: DeclarativeTerm) -> Self {
        Self { contract, ty }
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> DeclarativeTerm {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum ExplicitParameterDeclarativeSignature {
    Regular(ExplicitRegularParameterDeclarativeSignature),
}

impl ExplicitParameterDeclarativeSignature {
    pub fn into_ritchie_parameter_contracted_ty(
        self,
    ) -> DeclarativeTermRitchieParameterContractedType {
        match self {
            ExplicitParameterDeclarativeSignature::Regular(signature) => {
                DeclarativeTermRitchieParameterContractedType::new(signature.contract, signature.ty)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ExplicitParameterDeclarativeSignatureTemplates {
    data: SmallVec<[ExplicitParameterDeclarativeSignature; 4]>,
}

impl std::ops::Deref for ExplicitParameterDeclarativeSignatureTemplates {
    type Target = [ExplicitParameterDeclarativeSignature];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl ExplicitParameterDeclarativeSignatureTemplates {
    pub(crate) fn from_decl(
        parameters: &[ExplicitParameterDecl],
        expr_region_data: &ExprRegionData,
        signature_region: &DeclarativeTermRegion,
    ) -> DeclarativeSignatureResult<Self> {
        Ok(Self {
            data: parameters
                .iter()
                .enumerate()
                .map(|(i, parameter)| match parameter {
                    ExplicitParameterDecl::Regular {
                        pattern,
                        variables,
                        colon,
                        ty,
                    } => {
                        Ok(ExplicitRegularParameterDeclarativeSignature::new(
                            expr_region_data.pattern_contract(*pattern),
                            match signature_region.expr_term(*ty) {
                                Ok(ty) => ty,
                                Err(_) => return Err(
                                    DeclarativeSignatureError::ParameterTypeDeclarativeTermError(
                                        i.try_into().unwrap(),
                                    ),
                                ),
                            },
                        )
                        .into())
                    }
                    ExplicitParameterDecl::KeyedWithoutDefault { .. } => todo!(),
                    ExplicitParameterDecl::KeyedWithDefault { .. } => todo!(),
                    ExplicitParameterDecl::Variadic { .. } => todo!(),
                })
                .collect::<DeclarativeSignatureResult<_>>()?,
        })
    }
}
