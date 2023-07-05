use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum ExplicitParameterDeclarativeSignatureTemplate {
    Regular(ExplicitRegularParameterDeclarativeSignatureTemplate),
    Variadic(ExplicitVariadicParameterDeclarativeSignatureTemplate),
    KeyedWithoutDefault(ExplicitKeyedWithoutDefaultParameterDeclarativeSignatureTemplate),
    KeyedWithDefault(ExplicitKeyedWithDefaultParameterDeclarativeSignatureTemplate),
}

impl ExplicitParameterDeclarativeSignatureTemplate {
    pub fn into_ritchie_parameter_contracted_ty(
        self,
    ) -> DeclarativeTermRitchieParameterContractedType {
        match self {
            ExplicitParameterDeclarativeSignatureTemplate::Regular(signature) => {
                DeclarativeTermRitchieParameterContractedType::new(signature.contract, signature.ty)
            }
            ExplicitParameterDeclarativeSignatureTemplate::Variadic(_) => todo!(),
            ExplicitParameterDeclarativeSignatureTemplate::KeyedWithoutDefault(_) => todo!(),
            ExplicitParameterDeclarativeSignatureTemplate::KeyedWithDefault(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ExplicitParameterDeclarativeSignatureTemplates {
    data: SmallVec<[ExplicitParameterDeclarativeSignatureTemplate; 4]>,
}

impl std::ops::Deref for ExplicitParameterDeclarativeSignatureTemplates {
    type Target = [ExplicitParameterDeclarativeSignatureTemplate];

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
                .map(|(i, parameter)| {
                    Ok(match parameter {
                        ExplicitParameterDecl::Regular {
                            pattern,
                            variables,
                            colon,
                            ty,
                        } => ExplicitRegularParameterDeclarativeSignatureTemplate::new(
                            expr_region_data.pattern_contract(*pattern),
                            signature_region.expr_term(*ty).map_err(|_| {
                                DeclarativeSignatureError::ParameterTypeDeclarativeTermError(
                                    i.try_into().unwrap(),
                                )
                            })?,
                        )
                        .into(),
                        ExplicitParameterDecl::Variadic { .. } => {
                            todo!()
                            // ExplicitVariadicParameterDeclarativeSignature::new().into()
                        }
                        ExplicitParameterDecl::Keyed { .. } => todo!(),
                    })
                })
                .collect::<DeclarativeSignatureResult<_>>()?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitRegularParameterDeclarativeSignatureTemplate {
    contract: Contract,
    ty: DeclarativeTerm,
}

impl ExplicitRegularParameterDeclarativeSignatureTemplate {
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
pub struct ExplicitVariadicParameterDeclarativeSignatureTemplate {
    contract: Contract,
    ty: DeclarativeTerm,
}

impl ExplicitVariadicParameterDeclarativeSignatureTemplate {
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
pub struct ExplicitKeyedWithoutDefaultParameterDeclarativeSignatureTemplate {
    contract: Contract,
    ty: DeclarativeTerm,
}

impl ExplicitKeyedWithoutDefaultParameterDeclarativeSignatureTemplate {
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
pub struct ExplicitKeyedWithDefaultParameterDeclarativeSignatureTemplate {
    key: Ident,
    contract: Contract,
    ty: DeclarativeTerm,
    default: DeclarativeTerm,
}

impl ExplicitKeyedWithDefaultParameterDeclarativeSignatureTemplate {
    pub(crate) fn new(
        key: Ident,
        contract: Contract,
        ty: DeclarativeTerm,
        default: DeclarativeTerm,
    ) -> Self {
        Self {
            key,
            contract,
            ty,
            default: todo!(),
        }
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> DeclarativeTerm {
        self.ty
    }
}
