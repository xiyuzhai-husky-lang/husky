use crate::{symbol::HirSymbol, trai::HirTrait, *};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirTypeDb)]
pub struct HirTemplateParameters {
    data: SmallVec<[HirTemplateParameter; 2]>,
}

impl std::ops::Deref for HirTemplateParameters {
    type Target = [HirTemplateParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirTemplateParameter {
    // annotated_variance: Option<Variance>,
    symbol: HirSymbol,
    traits: Vec<HirTrait>,
}
