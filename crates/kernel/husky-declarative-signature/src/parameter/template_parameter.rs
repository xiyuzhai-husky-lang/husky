use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DeclarativeTemplateParameter {
    annotated_variance: Option<Variance>,
    symbol: DeclarativeTermSymbol,
    annotated_traits: Vec<DeclarativeTerm>,
}

impl DeclarativeTemplateParameter {
    fn new_explicit_from_decl(
        parameter_decl_pattern: &TemplateParameterSyndicate,
        region: &DeclarativeTermRegion,
        declarative_term_menu: &DeclarativeTermMenu,
    ) -> DeclarativeTemplateParameter {
        let symbol = parameter_decl_pattern.symbol();
        let annotated_variance =
            parameter_decl_pattern
                .annotated_variance_token()
                .map(|t| match t {
                    VarianceRegionalToken::Covariant(_) => Variance::Covariant,
                    VarianceRegionalToken::Contravariant(_) => Variance::Contravariant,
                    VarianceRegionalToken::Invariant(_) => Variance::Invariant,
                });
        match parameter_decl_pattern.data() {
            TemplateParameterSyndicateData::Type { .. } => {
                DeclarativeTemplateParameter {
                    symbol: region
                        .current_symbol_signature(symbol)
                        .expect("not none")
                        .term_symbol()
                        .expect("should have term"),
                    // ad hoc
                    annotated_traits: vec![],
                    annotated_variance,
                }
            }
            TemplateParameterSyndicateData::Constant { .. } => DeclarativeTemplateParameter {
                symbol: region
                    .current_symbol_signature(symbol)
                    .expect("not none")
                    .term_symbol()
                    .expect("should have term"),
                annotated_traits: vec![],
                annotated_variance,
            },
            TemplateParameterSyndicateData::Lifetime { .. } => {
                DeclarativeTemplateParameter {
                    symbol: region
                        .current_symbol_signature(symbol)
                        .expect("not none")
                        .term_symbol()
                        .expect("should have term"),
                    // ad hoc
                    annotated_traits: vec![],
                    annotated_variance,
                }
            }
            TemplateParameterSyndicateData::Place { .. } => {
                DeclarativeTemplateParameter {
                    symbol: region
                        .current_symbol_signature(symbol)
                        .expect("not none")
                        .term_symbol()
                        .expect("should have term"),
                    // ad hoc
                    annotated_traits: vec![],
                    annotated_variance,
                }
            }
        }
    }

    fn new_implicit(symbol: DeclarativeTermSymbol) -> Self {
        DeclarativeTemplateParameter {
            symbol: symbol,
            annotated_variance: None,
            annotated_traits: vec![],
        }
    }

    pub fn symbol(&self) -> DeclarativeTermSymbol {
        self.symbol
    }

    pub fn ty(
        &self,
        db: &dyn DeclarativeTermDb,
    ) -> DeclarativeTermSymbolTypeResult<DeclarativeTerm> {
        self.symbol().ty(db)
    }

    pub fn traits(&self) -> &[DeclarativeTerm] {
        self.annotated_traits.as_ref()
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
        template_parameter_obelisks: &[TemplateParameterSyndicate],
        declarative_term_region: &DeclarativeTermRegion,
        declarative_term_menu: &DeclarativeTermMenu,
    ) -> Self {
        Self {
            data: template_parameter_obelisks
                .iter()
                .map(|parameter_obelisk| {
                    DeclarativeTemplateParameter::new_explicit_from_decl(
                        parameter_obelisk,
                        declarative_term_region,
                        declarative_term_menu,
                    )
                })
                .chain(
                    declarative_term_region
                        .term_symbol_region()
                        .implicit_template_parameter_symbols()
                        .iter()
                        .map(|&a| DeclarativeTemplateParameter::new_implicit(a)),
                )
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
