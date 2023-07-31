use crate::{symbol::HirSymbol, trai::HirTrait, *};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirTypeDb)]
pub struct HirTemplateParameters {
    data: SmallVec<[HirTemplateParameter; 2]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirTemplateParameter {
    // annotated_variance: Option<Variance>,
    symbol: HirSymbol,
    traits: Vec<HirTrait>,
}
