use husky_expr::{
    ExprRegionData, ImplicitParameterDeclPatternVariant, RegularParameterDeclPattern,
};
use husky_token::VarianceToken;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterDeclarativeSignature {
    annotated_variance: Option<Variance>,
    symbol: DeclarativeTermSymbol,
    traits: Vec<DeclarativeTerm>,
}

impl ImplicitParameterDeclarativeSignature {
    fn from_decl(
        parameter_decl: &ImplicitParameterDecl,
        region: &DeclarativeTermRegion,
        declarative_term_menu: &DeclarativeTermMenu,
    ) -> ImplicitParameterDeclarativeSignature {
        let pattern = &parameter_decl.pattern();
        let symbol = pattern.symbol();
        let annotated_variance = pattern.annotated_variance_token().map(|t| match t {
            VarianceToken::Covariant(_) => Variance::Covariant,
            VarianceToken::Contravariant(_) => Variance::Contravariant,
            VarianceToken::Invariant(_) => Variance::Invariant,
        });
        match parameter_decl.pattern().variant() {
            ImplicitParameterDeclPatternVariant::Type0 { .. } => {
                ImplicitParameterDeclarativeSignature {
                    symbol: region
                        .current_symbol_term(symbol)
                        .expect("not none")
                        .term_symbol()
                        .expect("should have term"),
                    // ad hoc
                    traits: vec![],
                    annotated_variance,
                }
            }
            ImplicitParameterDeclPatternVariant::Constant { .. } => todo!(),
            ImplicitParameterDeclPatternVariant::Lifetime { .. } => {
                ImplicitParameterDeclarativeSignature {
                    symbol: region
                        .current_symbol_term(symbol)
                        .expect("not none")
                        .term_symbol()
                        .expect("should have term"),
                    // ad hoc
                    traits: vec![],
                    annotated_variance,
                }
            }
            ImplicitParameterDeclPatternVariant::Binding { .. } => todo!(),
        }
    }

    pub fn symbol(&self) -> DeclarativeTermSymbol {
        self.symbol
    }

    pub fn ty(&self, db: &dyn DeclarativeTermDb) -> DeclarativeTerm {
        self.symbol.ty(db).expect("should be okay")
    }

    pub fn traits(&self) -> &[DeclarativeTerm] {
        self.traits.as_ref()
    }

    pub fn annotated_variance(&self) -> Option<Variance> {
        self.annotated_variance
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterDeclarativeSignatures {
    data: Vec<ImplicitParameterDeclarativeSignature>,
}

impl ImplicitParameterDeclarativeSignatures {
    pub(crate) fn from_decl(
        implicit_parameters: &[ImplicitParameterDecl],
        declarative_term_region: &DeclarativeTermRegion,
        declarative_term_menu: &DeclarativeTermMenu,
    ) -> Self {
        Self {
            data: implicit_parameters
                .iter()
                .map(|parameter| {
                    ImplicitParameterDeclarativeSignature::from_decl(
                        parameter,
                        declarative_term_region,
                        declarative_term_menu,
                    )
                })
                .collect(),
        }
    }

    pub fn data(&self) -> &[ImplicitParameterDeclarativeSignature] {
        self.data.as_ref()
    }
}

impl std::ops::Deref for ImplicitParameterDeclarativeSignatures {
    type Target = [ImplicitParameterDeclarativeSignature];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitParameterDeclarativeSignatureTemplate {
    contract: Contract,
    ty: DeclarativeTerm,
}

impl ExplicitParameterDeclarativeSignatureTemplate {
    pub fn into_ritchie_parameter_contracted_ty(
        self,
    ) -> DeclarativeTermRitchieParameterContractedType {
        DeclarativeTermRitchieParameterContractedType::new(self.contract, self.ty)
    }
}

impl ExplicitParameterDeclarativeSignatureTemplate {
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
        parameters: &[RegularParameterDeclPattern],
        expr_region_data: &ExprRegionData,
        signature_region: &DeclarativeTermRegion,
    ) -> DeclarativeSignatureResult<Self> {
        Ok(Self {
            data: parameters
                .iter()
                .enumerate()
                .map(|(i, parameter)| {
                    let ty = parameter.ty();
                    parameter.pattern();
                    let ty = match signature_region.expr_term(ty) {
                        Ok(ty) => ty,
                        Err(_) => {
                            return Err(
                                DeclarativeSignatureError::ParameterTypeDeclarativeTermError(
                                    i.try_into().unwrap(),
                                ),
                            )
                        }
                    };
                    Ok(ExplicitParameterDeclarativeSignatureTemplate::new(
                        expr_region_data.pattern_contract(parameter.pattern()),
                        ty,
                    ))
                })
                .collect::<DeclarativeSignatureResult<_>>()?,
        })
    }
}
