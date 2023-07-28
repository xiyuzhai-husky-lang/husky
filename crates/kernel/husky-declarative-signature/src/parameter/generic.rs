use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DeclarativeTemplateParameter {
    annotated_variance: Option<Variance>,
    symbol: DeclarativeTermSymbol,
    traits: Vec<DeclarativeTerm>,
}

impl DeclarativeTemplateParameter {
    fn from_decl(
        parameter_decl_pattern: &TemplateParameterDecl,
        region: &DeclarativeTermRegion,
        declarative_term_menu: &DeclarativeTermMenu,
    ) -> DeclarativeTemplateParameter {
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
            TemplateParameterDeclPatternVariant::Type { .. } => {
                DeclarativeTemplateParameter {
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
            TemplateParameterDeclPatternVariant::Constant { .. } => DeclarativeTemplateParameter {
                symbol: region
                    .current_symbol_signature(symbol)
                    .expect("not none")
                    .term_symbol()
                    .expect("should have term"),
                traits: vec![],
                annotated_variance,
            },
            TemplateParameterDeclPatternVariant::Lifetime { .. } => {
                DeclarativeTemplateParameter {
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
            TemplateParameterDeclPatternVariant::Binding { .. } => todo!(),
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
pub struct DeclarativeTemplateParameterTemplates {
    data: SmallVec<[DeclarativeTemplateParameter; 4]>,
}

impl DeclarativeTemplateParameterTemplates {
    pub(crate) fn from_decl(
        template_parameters: &[TemplateParameterDecl],
        declarative_term_region: &DeclarativeTermRegion,
        declarative_term_menu: &DeclarativeTermMenu,
    ) -> Self {
        Self {
            data: template_parameters
                .iter()
                .map(|parameter| {
                    DeclarativeTemplateParameter::from_decl(
                        parameter,
                        declarative_term_region,
                        declarative_term_menu,
                    )
                })
                .collect(),
        }
    }

    pub fn data(&self) -> &[DeclarativeTemplateParameter] {
        self.data.as_ref()
    }
}

impl std::ops::Deref for DeclarativeTemplateParameterTemplates {
    type Target = [DeclarativeTemplateParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
