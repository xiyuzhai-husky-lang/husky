use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DeclarativeGenericParameter {
    annotated_variance: Option<Variance>,
    symbol: DeclarativeTermSymbol,
    traits: Vec<DeclarativeTerm>,
}

impl DeclarativeGenericParameter {
    fn from_decl(
        parameter_decl_pattern: &GenericParameterDecl,
        region: &DeclarativeTermRegion,
        declarative_term_menu: &DeclarativeTermMenu,
    ) -> DeclarativeGenericParameter {
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
            GenericParameterDeclPatternVariant::Type { .. } => {
                DeclarativeGenericParameter {
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
            GenericParameterDeclPatternVariant::Constant { .. } => DeclarativeGenericParameter {
                symbol: region
                    .current_symbol_signature(symbol)
                    .expect("not none")
                    .term_symbol()
                    .expect("should have term"),
                traits: vec![],
                annotated_variance,
            },
            GenericParameterDeclPatternVariant::Lifetime { .. } => {
                DeclarativeGenericParameter {
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
            GenericParameterDeclPatternVariant::Binding { .. } => todo!(),
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
pub struct DeclarativeGenericParameterTemplates {
    data: SmallVec<[DeclarativeGenericParameter; 4]>,
}

impl DeclarativeGenericParameterTemplates {
    pub(crate) fn from_decl(
        generic_parameters: &[GenericParameterDecl],
        declarative_term_region: &DeclarativeTermRegion,
        declarative_term_menu: &DeclarativeTermMenu,
    ) -> Self {
        Self {
            data: generic_parameters
                .iter()
                .map(|parameter| {
                    DeclarativeGenericParameter::from_decl(
                        parameter,
                        declarative_term_region,
                        declarative_term_menu,
                    )
                })
                .collect(),
        }
    }

    pub fn data(&self) -> &[DeclarativeGenericParameter] {
        self.data.as_ref()
    }
}

impl std::ops::Deref for DeclarativeGenericParameterTemplates {
    type Target = [DeclarativeGenericParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
