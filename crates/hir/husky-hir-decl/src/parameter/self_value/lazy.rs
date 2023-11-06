use super::*;
use husky_syn_expr::SelfValueParameterSyndicate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirLazySelfValueParameter {}

impl HirLazySelfValueParameter {
    pub(crate) fn from_syn(
        syndicate: Option<SelfValueParameterSyndicate>,
        db: &dyn HirDeclDb,
    ) -> Self {
        HirLazySelfValueParameter {}
    }
}
