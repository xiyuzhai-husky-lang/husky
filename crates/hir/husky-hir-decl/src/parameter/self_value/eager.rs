use super::*;
use husky_syn_expr::SelfValueParameterSyndicate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirEagerSelfValueParameter {}

impl HirEagerSelfValueParameter {
    pub(crate) fn from_syn(
        _syndicate: Option<SelfValueParameterSyndicate>,
        _db: &dyn HirDeclDb,
    ) -> Self {
        HirEagerSelfValueParameter {}
    }
}
