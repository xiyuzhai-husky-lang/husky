use husky_hir_ty::{place::HirQuary, ritchie::HirEagerContract};
use husky_place::place::Place;
use husky_sema_place_contract::site::SemaPlaceContractSite;
use vec_like::SmallVecPairMap;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerPlaceContractSite {
    data: SmallVecPairMap<Place, HirEagerContract, 2>,
}

impl HirEagerPlaceContractSite {
    pub(crate) fn from_sema(sema_site: &SemaPlaceContractSite) -> Self {
        HirEagerPlaceContractSite {
            data: Default::default(),
        }
    }

    pub(crate) fn self_value_expr(
        &self,
        self_value_place: HirQuary,
        contract: HirEagerContract,
    ) -> Self {
        let mut slf = self.clone();
        if let Some(place) = self_value_place.place()
            && contract != HirEagerContract::At
        {
            slf.insert(place, contract)
        }
        slf
    }

    fn insert(&mut self, place: Place, contract: HirEagerContract) {
        todo!()
    }
}
