use husky_expr::{ImplicitParameterDeclPatternVariant, RegularParameterDeclPattern};
use husky_token::VarianceToken;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterSignature {
    annotated_variance: Option<Variance>,
    term_symbol: TermSymbol,
    ty: Term,
    traits: Vec<Term>,
}

impl ImplicitParameterSignature {
    fn from_decl(
        parameter_decl: &ImplicitParameterDecl,
        region: &SignatureTermRegion,
        term_menu: &TermMenu,
    ) -> ImplicitParameterSignature {
        let pattern = &parameter_decl.pattern();
        let symbol = pattern.symbol();
        let variant = parameter_decl.pattern().variant();
        match variant {
            ImplicitParameterDeclPatternVariant::Type0 { .. } => {
                ImplicitParameterSignature {
                    term_symbol: region.current_symbol_term(symbol).expect("not none"),
                    ty: term_menu.ty0(),
                    // ad hoc
                    traits: vec![],
                    annotated_variance: pattern.annotated_variance_token().map(|t| match t {
                        VarianceToken::Covariant(_) => Variance::Covariant,
                        VarianceToken::Contravariant(_) => Variance::Contravariant,
                    }),
                }
            }
            ImplicitParameterDeclPatternVariant::Constant => todo!(),
            ImplicitParameterDeclPatternVariant::Lifetime => todo!(),
            ImplicitParameterDeclPatternVariant::Binding => todo!(),
        }
    }

    pub fn term_symbol(&self) -> TermSymbol {
        self.term_symbol
    }

    pub fn ty(&self) -> Term {
        self.ty
    }

    pub fn traits(&self) -> &[Term] {
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
        term_menu: &TermMenu,
    ) -> Self {
        Self {
            data: implicit_parameters
                .iter()
                .map(|parameter| {
                    ImplicitParameterSignature::from_decl(
                        parameter,
                        signature_term_region,
                        term_menu,
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
    type Target = Vec<ImplicitParameterSignature>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ParameterSignature {
    pattern: ParameterSignaturePattern,
    ty: Term,
}

impl ParameterSignature {
    pub fn ty(&self) -> Term {
        self.ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ParameterSignaturePattern {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ParameterSignatures {
    parameters: Vec<ParameterSignature>,
}

impl std::ops::Deref for ParameterSignatures {
    type Target = Vec<ParameterSignature>;

    fn deref(&self) -> &Self::Target {
        &self.parameters
    }
}

impl ParameterSignatures {
    pub(crate) fn from_decl(
        parameters: &[RegularParameterDeclPattern],
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
                            return Err(SignatureError::ParameterTypeTermError(
                                i.try_into().unwrap(),
                            ))
                        }
                    };
                    Ok(ParameterSignature {
                        pattern: ParameterSignaturePattern {},
                        ty,
                    })
                })
                .collect::<Result<Vec<ParameterSignature>, SignatureError>>()?,
        })
    }
}
