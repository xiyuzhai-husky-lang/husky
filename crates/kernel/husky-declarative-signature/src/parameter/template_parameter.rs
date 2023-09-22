use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum DeclarativeTemplateParameter {
    Explicit {
        annotated_variance: Option<Variance>,
        symbol: DeclarativeTermSymbol,
        traits: Vec<DeclarativeTerm>,
    },
    Implicit {
        symbol: DeclarativeTermSymbol,
        kind: ImplicitTemplateParameterSymbolKind,
    },
}

impl DeclarativeTemplateParameter {
    fn new_explicit_from_decl(
        parameter_decl_pattern: &TemplateParameterObelisk,
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
            TemplateParameterObeliskData::Type { .. } => {
                DeclarativeTemplateParameter::Explicit {
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
            TemplateParameterObeliskData::Constant { .. } => {
                DeclarativeTemplateParameter::Explicit {
                    symbol: region
                        .current_symbol_signature(symbol)
                        .expect("not none")
                        .term_symbol()
                        .expect("should have term"),
                    traits: vec![],
                    annotated_variance,
                }
            }
            TemplateParameterObeliskData::Lifetime { .. } => {
                DeclarativeTemplateParameter::Explicit {
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
            TemplateParameterObeliskData::Place { .. } => {
                DeclarativeTemplateParameter::Explicit {
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
        }
    }

    fn new_implicit(symbol: ImplicitTemplateParameterSymbol) -> Self {
        match symbol.kind() {
            ImplicitTemplateParameterSymbolKind::SelfType => todo!(),
            ImplicitTemplateParameterSymbolKind::SelfLifetime => todo!(),
            ImplicitTemplateParameterSymbolKind::SelfPlace => todo!(),
        }
        DeclarativeTemplateParameter::Implicit {
            symbol: symbol.symbol(),
            kind: symbol.kind(),
        }
    }

    pub fn symbol(&self) -> DeclarativeTermSymbol {
        match self {
            DeclarativeTemplateParameter::Explicit { symbol, .. }
            | DeclarativeTemplateParameter::Implicit { symbol, .. } => *symbol,
        }
    }

    pub fn ty(
        &self,
        db: &dyn DeclarativeTermDb,
    ) -> DeclarativeTermSymbolTypeResult<DeclarativeTerm> {
        self.symbol().ty(db)
    }

    pub fn traits(&self) -> &[DeclarativeTerm] {
        match self {
            DeclarativeTemplateParameter::Explicit {
                annotated_variance,
                symbol,
                traits,
            } => todo!(),
            DeclarativeTemplateParameter::Implicit { symbol, kind } => todo!(),
        }
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
        template_parameter_obelisks: &[TemplateParameterObelisk],
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
