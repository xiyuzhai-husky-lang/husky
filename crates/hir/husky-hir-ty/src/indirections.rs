use crate::quary::HirQuary;
use husky_fly_term::dispatch::{FlyIndirection, FlyIndirections};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirIndirections {
    initial_place: HirQuary,
    indirections: SmallVec<[HirIndirection; 2]>,
    final_place: HirQuary,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirIndirection {
    Place(HirQuary),
    Leash,
}

impl std::ops::Deref for HirIndirections {
    type Target = [HirIndirection];

    fn deref(&self) -> &Self::Target {
        &self.indirections
    }
}

impl HirIndirections {
    pub fn from_fly(indirections: &FlyIndirections) -> Self {
        HirIndirections {
            initial_place: HirQuary::from_fly(indirections.initial_place()),
            indirections: indirections
                .iter()
                .map(|&indirection| HirIndirection::from_fly(indirection))
                .collect(),
            final_place: HirQuary::from_fly(indirections.final_place()),
        }
    }
}

impl HirIndirection {
    fn from_fly(indiretion: FlyIndirection) -> Self {
        match indiretion {
            FlyIndirection::QualifiedPlace(place) => {
                HirIndirection::Place(HirQuary::from_fly(place))
            }
            FlyIndirection::Leash => HirIndirection::Leash,
        }
    }
}
