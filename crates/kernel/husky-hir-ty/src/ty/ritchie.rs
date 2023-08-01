use super::*;
use husky_ethereal_term::{EtherealTermRitchieParameter, EtherealTermRitchieRegularParameter};
use husky_term_prelude::Contract;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirTypeDb)]
pub struct HirRitchieParameter {
    contract: Contract,
    ty: HirType,
}

impl HirRitchieParameter {
    pub fn from_ethereal(param: EtherealTermRitchieParameter, db: &dyn HirTypeDb) -> Self {
        match param {
            EtherealTermRitchieParameter::Regular(param) => Self::from_ethereal_regular(param, db),
            EtherealTermRitchieParameter::Variadic(_) => todo!(),
            EtherealTermRitchieParameter::Keyed(_) => todo!(),
        }
    }

    pub fn from_ethereal_regular(
        param: EtherealTermRitchieRegularParameter,
        db: &dyn HirTypeDb,
    ) -> Self {
        Self {
            contract: param.contract(),
            ty: HirType::from_ethereal(param.ty(), db),
        }
    }
}
