use super::*;
use husky_term_prelude::Variance;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirPlaceSvar {
    pub(crate) attrs: HirTemplateSvarAttrs,
    pub(crate) variance: Option<Variance>,
    pub(crate) disambiguator: u8,
}

impl HirPlaceSvar {
    pub fn attrs(&self) -> &HirTemplateSvarAttrs {
        &self.attrs
    }

    pub fn variance(&self) -> Option<Variance> {
        self.variance
    }

    pub fn disambiguator(&self) -> u8 {
        self.disambiguator
    }
}
