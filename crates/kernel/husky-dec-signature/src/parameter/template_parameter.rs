use super::*;
use husky_syn_expr::syndicates::{TemplateParameterSyndicateVariant, TemplateSynParameterData};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DeclarativeTemplateParameter {
    annotated_variance: Option<Variance>,
    svar: DecSymbolicVariable,
    annotated_traits: Vec<DecTerm>,
}

impl DeclarativeTemplateParameter {
    fn new_explicit_from_decl(
        parameter_decl_pattern: &TemplateSynParameterData,
        region: &SynExprDecTermRegion,
        dec_term_menu: &DecTermMenu,
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
        match parameter_decl_pattern.variant() {
            TemplateParameterSyndicateVariant::Type { .. } => {
                DeclarativeTemplateParameter {
                    svar: region
                        .current_variable_signature(symbol)
                        .expect("not none")
                        .term()
                        .expect("should have term"),
                    // ad hoc
                    annotated_traits: vec![],
                    annotated_variance,
                }
            }
            TemplateParameterSyndicateVariant::Compterm { .. } => DeclarativeTemplateParameter {
                svar: region
                    .current_variable_signature(symbol)
                    .expect("not none")
                    .term()
                    .expect("should have term"),
                annotated_traits: vec![],
                annotated_variance,
            },
            TemplateParameterSyndicateVariant::Lifetime { .. } => {
                DeclarativeTemplateParameter {
                    svar: region
                        .current_variable_signature(symbol)
                        .expect("not none")
                        .term()
                        .expect("should have term"),
                    // ad hoc
                    annotated_traits: vec![],
                    annotated_variance,
                }
            }
            TemplateParameterSyndicateVariant::Place { .. } => {
                DeclarativeTemplateParameter {
                    svar: region
                        .current_variable_signature(symbol)
                        .expect("not none")
                        .term()
                        .expect("should have term"),
                    // ad hoc
                    annotated_traits: vec![],
                    annotated_variance,
                }
            }
        }
    }

    fn new_implicit(symbol: DecSymbolicVariable) -> Self {
        DeclarativeTemplateParameter {
            svar: symbol,
            annotated_variance: None,
            annotated_traits: vec![],
        }
    }

    pub fn symbol(&self) -> DecSymbolicVariable {
        self.svar
    }

    pub fn ty(&self, db: &::salsa::Db) -> DecSymbolicVariableTypeResult<DecTerm> {
        self.symbol().ty(db)
    }

    pub fn traits(&self) -> &[DecTerm] {
        self.annotated_traits.as_ref()
    }

    pub fn annotated_variance(&self) -> Option<Variance> {
        self.annotated_variance
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DecTemplateParameters {
    data: SmallVec<[DeclarativeTemplateParameter; 4]>,
}

impl DecTemplateParameters {
    pub(crate) fn from_decl(
        template_parameter_obelisks: &[TemplateSynParameterData],
        dec_term_region: &SynExprDecTermRegion,
        dec_term_menu: &DecTermMenu,
    ) -> Self {
        Self {
            data: template_parameter_obelisks
                .iter()
                .map(|parameter_obelisk| {
                    DeclarativeTemplateParameter::new_explicit_from_decl(
                        parameter_obelisk,
                        dec_term_region,
                        dec_term_menu,
                    )
                })
                .chain(
                    dec_term_region
                        .symbolic_variable_region()
                        .auto_template_parameter_symbols()
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

impl std::ops::Deref for DecTemplateParameters {
    type Target = [DeclarativeTemplateParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
