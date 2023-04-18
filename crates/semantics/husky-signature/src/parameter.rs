use husky_expr::{
    ExprRegionData, ImplicitParameterDeclPatternVariant, RegularParameterDeclPattern,
};
use husky_token::VarianceToken;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterSignature {
    annotated_variance: Option<Variance>,
    symbol: DeclarativeTermSymbol,
    traits: Vec<DeclarativeTerm>,
}

impl ImplicitParameterSignature {
    fn from_decl(
        parameter_decl: &ImplicitParameterDecl,
        region: &DeclarativeTermRegion,
        declarative_term_menu: &DeclarativeTermMenu,
    ) -> ImplicitParameterSignature {
        let pattern = &parameter_decl.pattern();
        let symbol = pattern.symbol();
        let annotated_variance = pattern.annotated_variance_token().map(|t| match t {
            VarianceToken::Covariant(_) => Variance::Covariant,
            VarianceToken::Contravariant(_) => Variance::Contravariant,
            VarianceToken::Invariant(_) => Variance::Invariant,
        });
        match parameter_decl.pattern().variant() {
            ImplicitParameterDeclPatternVariant::Type0 { .. } => {
                ImplicitParameterSignature {
                    symbol: region
                        .current_symbol_term(symbol)
                        .expect("not none")
                        .symbol()
                        .expect("should have term"),
                    // ad hoc
                    traits: vec![],
                    annotated_variance,
                }
            }
            ImplicitParameterDeclPatternVariant::Constant { .. } => todo!(),
            ImplicitParameterDeclPatternVariant::Lifetime { .. } => {
                ImplicitParameterSignature {
                    symbol: region
                        .current_symbol_term(symbol)
                        .expect("not none")
                        .symbol()
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
pub struct ImplicitParameterSignatures {
    data: Vec<ImplicitParameterSignature>,
}

impl ImplicitParameterSignatures {
    pub(crate) fn from_decl(
        implicit_parameters: &[ImplicitParameterDecl],
        declarative_term_region: &DeclarativeTermRegion,
        declarative_term_menu: &DeclarativeTermMenu,
    ) -> Self {
        Self {
            data: implicit_parameters
                .iter()
                .map(|parameter| {
                    ImplicitParameterSignature::from_decl(
                        parameter,
                        declarative_term_region,
                        declarative_term_menu,
                    )
                })
                .collect(),
        }
    }

    pub fn decls(&self) -> &[ImplicitParameterSignature] {
        self.data.as_ref()
    }
}

impl std::ops::Deref for ImplicitParameterSignatures {
    type Target = [ImplicitParameterSignature];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitParameterSignature {
    contract: Contract,
    ty: DeclarativeTerm,
}

impl ExplicitParameterSignature {
    pub fn into_ritchie_parameter_contracted_ty(
        self,
    ) -> DeclarativeTermRitchieParameterContractedType {
        DeclarativeTermRitchieParameterContractedType::new(self.contract, self.ty)
    }
}

impl ExplicitParameterSignature {
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
pub struct ExplicitParameterSignatures {
    parameters: Vec<ExplicitParameterSignature>,
}

impl std::ops::Deref for ExplicitParameterSignatures {
    type Target = [ExplicitParameterSignature];

    fn deref(&self) -> &Self::Target {
        &self.parameters
    }
}

impl ExplicitParameterSignatures {
    pub(crate) fn from_decl(
        parameters: &[RegularParameterDeclPattern],
        expr_region_data: &ExprRegionData,
        signature_region: &DeclarativeTermRegion,
    ) -> DeclarativeSignatureResult<Self> {
        Ok(Self {
            parameters: parameters
                .iter()
                .enumerate()
                .map(|(i, parameter)| {
                    let ty = parameter.ty();
                    parameter.pattern();
                    let ty = match signature_region.expr_term(ty) {
                        Ok(ty) => ty,
                        Err(_) => {
                            return Err(SignatureError::ParameterTypeDeclarativeTermError(
                                i.try_into().unwrap(),
                            ))
                        }
                    };
                    Ok(ExplicitParameterSignature::new(
                        expr_region_data.pattern_contract(parameter.pattern()),
                        ty,
                    ))
                })
                .collect::<Result<Vec<ExplicitParameterSignature>, SignatureError>>()?,
        })
    }
}
