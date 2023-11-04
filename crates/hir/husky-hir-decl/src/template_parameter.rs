use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirTemplateParameter {}

pub type HirTemplateParameters = SmallVec<[HirTemplateParameter; 2]>;
