use crate::*;
use either::*;
use husky_fly_term::{deref::DerefFlyCoersion, trival::TrivialFlyCoersion, FlyCoersion};
use husky_hir_ty::{lifetime::HirLifetime, quary::HirQuary};

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirEagerCoersion {
    Trivial(TrivialHirEagerCoersion),
    Never,
    WrapInSome,
    PlaceToLeash,
    Deref(DerefHirEagerCoersion),
}

impl HirEagerCoersion {
    pub const TRIVIAL_TRANSIENT: Self = HirEagerCoersion::Trivial(TrivialHirEagerCoersion {
        expectee_quary: HirQuary::Transient,
    });

    pub fn quary_after_coersion(self) -> HirQuary {
        match self {
            HirEagerCoersion::Trivial(slf) => slf.place_after_coersion(),
            HirEagerCoersion::Deref(slf) => slf.place_after_coersion(),
            HirEagerCoersion::Never
            | HirEagerCoersion::WrapInSome
            | HirEagerCoersion::PlaceToLeash => HirQuary::Transient,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TrivialHirEagerCoersion {
    expectee_quary: HirQuary,
}
impl TrivialHirEagerCoersion {
    fn place_after_coersion(self) -> HirQuary {
        self.expectee_quary
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DerefHirEagerCoersion {
    Leash,
    Ref { lifetime: HirLifetime },
}
impl DerefHirEagerCoersion {
    fn place_after_coersion(self) -> HirQuary {
        match self {
            DerefHirEagerCoersion::Leash => HirQuary::Leashed { place_idx: None },
            DerefHirEagerCoersion::Ref { lifetime } => HirQuary::Ref {
                guard: Right(lifetime),
            },
        }
    }
}

impl ToHirEager for FlyCoersion {
    type Output = HirEagerCoersion;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        match self {
            FlyCoersion::Trivial(slf) => HirEagerCoersion::Trivial(slf.to_hir_eager(builder)),
            FlyCoersion::Never => HirEagerCoersion::Never,
            FlyCoersion::WrapInSome => HirEagerCoersion::WrapInSome,
            FlyCoersion::PlaceToLeash => HirEagerCoersion::PlaceToLeash,
            FlyCoersion::Deref(slf) => HirEagerCoersion::Deref(slf.to_hir_eager(builder)),
        }
    }
}

impl ToHirEager for TrivialFlyCoersion {
    type Output = TrivialHirEagerCoersion;

    fn to_hir_eager(&self, _builder: &mut HirEagerExprBuilder) -> Self::Output {
        TrivialHirEagerCoersion {
            expectee_quary: HirQuary::from_fly(self.expectee_quary),
        }
    }
}

impl ToHirEager for DerefFlyCoersion {
    type Output = DerefHirEagerCoersion;

    fn to_hir_eager(&self, _builder: &mut HirEagerExprBuilder) -> Self::Output {
        match *self {
            DerefFlyCoersion::Leash => DerefHirEagerCoersion::Leash,
            DerefFlyCoersion::Ref { lifetime } => DerefHirEagerCoersion::Ref {
                lifetime: HirLifetime::from_fly(lifetime),
            },
        }
    }
}
