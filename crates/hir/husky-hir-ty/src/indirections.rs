use crate::place::HirPlace;
use husky_fluffy_term::{FluffyIndirection, FluffyIndirections};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirIndirections {
    initial_place: HirPlace,
    indirections: SmallVec<[HirIndirection; 2]>,
    final_place: HirPlace,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirIndirection {
    Place(HirPlace),
    Leash,
}

impl std::ops::Deref for HirIndirections {
    type Target = [HirIndirection];

    fn deref(&self) -> &Self::Target {
        &self.indirections
    }
}

impl HirIndirections {
    pub fn from_fluffy(indirections: &FluffyIndirections) -> Self {
        HirIndirections {
            initial_place: HirPlace::from_fluffy(indirections.initial_place()),
            indirections: indirections
                .iter()
                .map(|&indirection| HirIndirection::from_fluffy(indirection))
                .collect(),
            final_place: HirPlace::from_fluffy(indirections.final_place()),
        }
    }
}

impl HirIndirection {
    fn from_fluffy(indiretion: FluffyIndirection) -> Self {
        match indiretion {
            FluffyIndirection::Place(place) => HirIndirection::Place(HirPlace::from_fluffy(place)),
            FluffyIndirection::Leash => HirIndirection::Leash,
        }
    }
}
