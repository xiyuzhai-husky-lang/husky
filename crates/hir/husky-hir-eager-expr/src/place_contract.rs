use husky_hir_ty::{place::HirQuary, ritchie::HirEagerContract};
use husky_place::place::Place;
use husky_sema_place_contract::site::SemaPlaceContractSite;
use husky_term_prelude::Contract;
use vec_like::SmallVecPairMap;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerPlaceContractSite {
    data: SmallVecPairMap<Place, HirEagerContract, 2>,
}

impl HirEagerPlaceContractSite {
    pub(crate) fn from_sema(sema_site: &SemaPlaceContractSite) -> Self {
        HirEagerPlaceContractSite {
            data: SmallVecPairMap::from_iter(
                sema_site
                    .place_contracts()
                    .iter()
                    .copied()
                    .filter_map(|(place, contract)| {
                        (contract != Contract::At)
                            .then_some((place, HirEagerContract::from_contract(contract)))
                    }),
            ),
        }
    }
}
