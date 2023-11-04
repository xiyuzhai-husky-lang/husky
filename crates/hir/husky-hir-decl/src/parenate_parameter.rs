use crate::db::HirDeclDb;
use husky_syn_expr::{ParenateParameterSyndicate, SelfParameterSyndicate};
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirParenateParameter {
    Ordinary,
    Keyed,
    Variadic,
}

impl HirParenateParameter {
    pub(crate) fn from_self_value_parameter_syndicate(
        syndicate: Option<SelfParameterSyndicate>,
        db: &dyn HirDeclDb,
    ) -> Self {
        todo!()
    }

    pub(crate) fn from_syndicate(
        syndicate: &ParenateParameterSyndicate,
        db: &dyn HirDeclDb,
    ) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirParenateParameters(SmallVec<[HirParenateParameter; 4]>);

impl std::ops::Deref for HirParenateParameters {
    type Target = SmallVec<[HirParenateParameter; 4]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HirParenateParameters {
    pub(crate) fn from_syn(syndicates: &[ParenateParameterSyndicate], db: &dyn HirDeclDb) -> Self {
        todo!()
    }
}
