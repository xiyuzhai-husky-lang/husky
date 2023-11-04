use super::*;
use husky_ethereal_term::{EtherealRitchieParameter, EtherealRitchieRegularParameter};
use husky_term_prelude::Contract;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirTypeDb)]
pub struct HirRitchieParameter {
    contract: Contract,
    ty: HirType,
}

impl HirRitchieParameter {
    pub fn from_ethereal(param: EtherealRitchieParameter, db: &dyn HirTypeDb) -> Self {
        match param {
            EtherealRitchieParameter::Regular(param) => Self::from_ethereal_regular(param, db),
            EtherealRitchieParameter::Variadic(_) => todo!(),
            EtherealRitchieParameter::Keyed(_) => todo!(),
        }
    }

    pub fn from_ethereal_regular(
        param: EtherealRitchieRegularParameter,
        db: &dyn HirTypeDb,
    ) -> Self {
        Self {
            contract: param.contract(),
            ty: HirType::from_ethereal(param.ty(), db),
        }
    }
}
