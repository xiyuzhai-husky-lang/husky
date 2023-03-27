use husky_expr::{ExplicitParameterDeclPattern, ImplicitParameterDeclPatternVariant};
use husky_token::VarianceToken;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterSignature {
    annotated_variance: Option<Variance>,
    symbol: RawTermOriginalSymbol,
    ty: RawTerm,
    traits: Vec<RawTerm>,
}

impl ImplicitParameterSignature {
    fn from_decl(
        parameter_decl: &ImplicitParameterDecl,
        region: &SignatureTermRegion,
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
                    ty: raw_term_menu.ty0().into(),
                    // ad hoc
                    traits: vec![],
                    annotated_variance,
                }
            }
            ImplicitParameterDeclPatternVariant::Constant { .. } => todo!(),
            ImplicitParameterDeclPatternVariant::Lifetime { .. } => {
                ImplicitParameterSignature {
                    symbol: region.current_symbol_term(symbol).expect("not none"),
                    ty: raw_term_menu.lifetime_ty().into(),
                    // ad hoc
                    traits: vec![],
                    annotated_variance,
                }
            }
            ImplicitParameterDeclPatternVariant::Binding { .. } => todo!(),
        }
    }

    pub fn symbol(&self) -> RawTermOriginalSymbol {
        self.symbol
    }

    pub fn ty(&self) -> RawTerm {
        self.ty
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
        signature_term_region: &SignatureTermRegion,
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct RegularParameterSignature {
    pattern: ParameterSignaturePattern,
    ty: RawTerm,
}

impl RegularParameterSignature {
    pub fn ty(&self) -> RawTerm {
        self.ty
    }

    pub(crate) fn new_self_parameter(ty: RawTerm) -> Self {
        Self {
            pattern: ParameterSignaturePattern {},
            ty,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ParameterSignaturePattern {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct RegularParameterSignatures {
    parameters: Vec<RegularParameterSignature>,
}

impl std::ops::Deref for RegularParameterSignatures {
    type Target = [RegularParameterSignature];

    fn deref(&self) -> &Self::Target {
        &self.parameters
    }
}

impl RegularParameterSignatures {
    pub(crate) fn from_decl(
        parameters: &[ExplicitParameterDeclPattern],
        sheet: &SignatureTermRegion,
    ) -> SignatureResult<Self> {
        Ok(Self {
            parameters: parameters
                .iter()
                .enumerate()
                .map(|(i, parameter)| {
                    let ty = parameter.ty();
                    let ty = match sheet.expr_term(ty) {
                        Ok(ty) => ty,
                        Err(_) => {
                            return Err(SignatureError::ParameterTypeRawTermError(
                                i.try_into().unwrap(),
                            ))
                        }
                    };
                    Ok(RegularParameterSignature {
                        pattern: ParameterSignaturePattern {},
                        ty,
                    })
                })
                .collect::<Result<Vec<RegularParameterSignature>, SignatureError>>()?,
        })
    }
}
