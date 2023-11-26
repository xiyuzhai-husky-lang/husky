use husky_ethereal_term::EtherealTermTemplateParameter;

use crate::{symbol::HirTemplateSymbol, trai::HirTrait, *};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirTypeDb, jar = HirTypeJar)]
pub struct HirTemplateParameters {
    data: SmallVec<[HirTemplateParameter; 2]>,
}

impl std::ops::Deref for HirTemplateParameters {
    type Target = [HirTemplateParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl HirTemplateParameters {
    pub fn from_ethereal(
        template_parameters: &[EtherealTermTemplateParameter],
        db: &dyn HirTypeDb,
    ) -> Self {
        Self {
            data: template_parameters
                .iter()
                .filter_map(|param| HirTemplateParameter::from_ethereal(param, db))
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirTemplateParameter {
    // annotated_variance: Option<Variance>,
    symbol: HirTemplateSymbol,
    traits: Vec<HirTrait>,
}
impl HirTemplateParameter {
    fn from_ethereal(
        template_parameter: &EtherealTermTemplateParameter,
        db: &dyn HirTypeDb,
    ) -> Option<Self> {
        Some(HirTemplateParameter {
            symbol: HirTemplateSymbol::from_ethereal(template_parameter.symbol(), db)?,
            traits: template_parameter
                .traits()
                .iter()
                .copied()
                .map(|trai| HirTrait::from_ethereal(trai, db))
                .collect(),
        })
    }

    pub fn symbol(&self) -> HirTemplateSymbol {
        self.symbol
    }

    pub fn traits(&self) -> &[HirTrait] {
        &self.traits
    }
}
