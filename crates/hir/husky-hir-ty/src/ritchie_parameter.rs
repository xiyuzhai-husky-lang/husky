pub mod keyed;
pub mod regular;
pub mod variadic;

use self::keyed::*;
use self::regular::*;
use self::variadic::*;
use crate::*;
use husky_coword::Ident;
use husky_term_prelude::Contract;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirRitchieParameters {
    data: SmallVec<[HirRitchieParameter; 4]>,
}

impl HirRitchieParameters {
    // pub(crate) fn from_syn(params: &EtherealTermParenateParameters, db: &dyn HirDeclDb) -> Self {
    //     HirRitchieParameters {
    //         data: params
    //             .iter()
    //             .copied()
    //             .map(|param| HirRitchieParameter::from_ethereal(param, db))
    //             .collect(),
    //     }
    // }
}

impl std::ops::Deref for HirRitchieParameters {
    type Target = [HirRitchieParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirRitchieParameter {
    Regular(HirRitchieRegularParameter),
    Variadic(HirRitchieVariadicParameter),
    Keyed(HirRitchieKeyedParameter),
}
