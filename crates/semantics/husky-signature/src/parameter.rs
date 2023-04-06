use husky_expr::{ImplicitParameterDeclPatternVariant, RegularParameterDeclPattern};
use husky_token::VarianceToken;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterSignature {
    annotated_variance: Option<Variance>,
    symbol: RawTermSymbol,
    traits: Vec<RawTerm>,
}

impl ImplicitParameterSignature {
    fn from_decl(
        parameter_decl: &ImplicitParameterDecl,
        region: &SignatureRegion,
        raw_term_menu: &RawTermMenu,
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
                    symbol: region.current_symbol_term(symbol).expect("not none"),
                    // ad hoc
                    traits: vec![],
                    annotated_variance,
                }
            }
            ImplicitParameterDeclPatternVariant::Constant { .. } => todo!(),
            ImplicitParameterDeclPatternVariant::Lifetime { .. } => {
                ImplicitParameterSignature {
                    symbol: region.current_symbol_term(symbol).expect("not none"),
                    // ad hoc
                    traits: vec![],
                    annotated_variance,
                }
            }
            ImplicitParameterDeclPatternVariant::Binding { .. } => todo!(),
        }
    }

    pub fn symbol(&self) -> RawTermSymbol {
        self.symbol
    }

    pub fn ty(&self, db: &dyn RawTermDb) -> RawTerm {
        self.symbol
            .qualified_ty(db)
            .expect("should be okay")
            .base_ty(db)
    }

    pub fn traits(&self) -> &[RawTerm] {
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
        signature_term_region: &SignatureRegion,
        raw_term_menu: &RawTermMenu,
    ) -> Self {
        Self {
            data: implicit_parameters
                .iter()
                .map(|parameter| {
                    ImplicitParameterSignature::from_decl(
                        parameter,
                        signature_term_region,
                        raw_term_menu,
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
    ty: RawTerm,
}

impl ExplicitParameterSignature {
    pub fn into_ritchie_parameter_contracted_ty(self) -> RawTermRitchieParameterContractedType {
        RawTermRitchieParameterContractedType::new(self.contract, self.ty)
    }
}

impl ExplicitParameterSignature {
    pub(crate) fn new(contract: Contract, ty: RawTerm) -> Self {
        Self { contract, ty }
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> RawTerm {
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
        region: &SignatureRegion,
    ) -> SignatureResult<Self> {
        Ok(Self {
            parameters: parameters
                .iter()
                .enumerate()
                .map(|(i, parameter)| {
                    let ty = parameter.ty();
                    parameter.pattern();
                    let ty = match region.expr_term(ty) {
                        Ok(ty) => ty,
                        Err(_) => {
                            return Err(SignatureError::ParameterTypeRawTermError(
                                i.try_into().unwrap(),
                            ))
                        }
                    };
                    Ok(ExplicitParameterSignature::new(
                        region.pattern_modifier(parameter.pattern()).into(),
                        ty,
                    ))
                })
                .collect::<Result<Vec<ExplicitParameterSignature>, SignatureError>>()?,
        })
    }
}
