use crate::db::HirDeclDb;
use husky_syn_expr::SynParenateParameterObelisk;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirParenateParameter {
    Ordinary,
    Keyed,
    Variadic,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirParenateParameters(SmallVec<[HirParenateParameter; 4]>);

impl HirParenateParameters {
    pub(crate) fn from_syn(obelisks: &[SynParenateParameterObelisk], db: &dyn HirDeclDb) -> Self {
        todo!()
    }
}
