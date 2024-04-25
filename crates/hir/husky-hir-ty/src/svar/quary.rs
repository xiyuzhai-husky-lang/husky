use super::*;
use husky_term_prelude::Variance;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirQuaryTemplateVariable {
    pub(crate) attrs: HirTemplateVariableAttrs,
    pub(crate) variance: Option<Variance>,
    pub(crate) disambiguator: u8,
}

impl HirQuaryTemplateVariable {
    pub fn attrs(&self) -> &HirTemplateVariableAttrs {
        &self.attrs
    }

    pub fn variance(&self) -> Option<Variance> {
        self.variance
    }

    pub fn disambiguator(&self) -> u8 {
        self.disambiguator
    }
}
