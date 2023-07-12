use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterDeclarativeSignature {
    annotated_variance: Option<Variance>,
    symbol: DeclarativeTermSymbol,
    traits: Vec<DeclarativeTerm>,
}

impl ImplicitParameterDeclarativeSignature {
    fn from_decl(
        parameter_decl_pattern: &ImplicitParameterDecl,
        region: &DeclarativeTermRegion,
        declarative_term_menu: &DeclarativeTermMenu,
    ) -> ImplicitParameterDeclarativeSignature {
        let symbol = parameter_decl_pattern.symbol();
        let annotated_variance =
            parameter_decl_pattern
                .annotated_variance_token()
                .map(|t| match t {
                    VarianceToken::Covariant(_) => Variance::Covariant,
                    VarianceToken::Contravariant(_) => Variance::Contravariant,
                    VarianceToken::Invariant(_) => Variance::Invariant,
                });
        match parameter_decl_pattern.variant() {
            ImplicitParameterDeclPatternVariant::Type { .. } => {
                ImplicitParameterDeclarativeSignature {
                    symbol: region
                        .current_symbol_signature(symbol)
                        .expect("not none")
                        .term_symbol()
                        .expect("should have term"),
                    // ad hoc
                    traits: vec![],
                    annotated_variance,
                }
            }
            ImplicitParameterDeclPatternVariant::Constant { .. } => {
                ImplicitParameterDeclarativeSignature {
                    symbol: region
                        .current_symbol_signature(symbol)
                        .expect("not none")
                        .term_symbol()
                        .expect("should have term"),
                    traits: vec![],
                    annotated_variance,
                }
            }
            ImplicitParameterDeclPatternVariant::Lifetime { .. } => {
                ImplicitParameterDeclarativeSignature {
                    symbol: region
                        .current_symbol_signature(symbol)
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

    pub fn ty(
        &self,
        db: &dyn DeclarativeTermDb,
    ) -> DeclarativeTermSymbolTypeResult<DeclarativeTerm> {
        self.symbol.ty(db)
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
    data: SmallVec<[ImplicitParameterDeclarativeSignature; 4]>,
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
