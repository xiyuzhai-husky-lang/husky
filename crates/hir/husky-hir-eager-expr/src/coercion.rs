use crate::*;
use either::*;
use husky_fly_term::{deref::DerefFlyCoercion, trival::TrivialFlyCoercion, FlyCoercion};
use husky_hir_ty::{lifetime::HirLifetime, quary::HirQuary};

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirEagerCoercion {
    Trivial(TrivialHirEagerCoercion),
    Never,
    WrapInSome,
    Releash,
    Deref(DerefHirEagerCoercion),
}

impl HirEagerCoercion {
    pub const TRIVIAL_TRANSIENT: Self = HirEagerCoercion::Trivial(TrivialHirEagerCoercion {
        expectee_quary: HirQuary::Transient,
    });

    pub fn quary_after_coercion(self) -> HirQuary {
        match self {
            HirEagerCoercion::Trivial(slf) => slf.place_after_coercion(),
            HirEagerCoercion::Deref(slf) => slf.place_after_coercion(),
            HirEagerCoercion::Never | HirEagerCoercion::WrapInSome | HirEagerCoercion::Releash => {
                HirQuary::Transient
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TrivialHirEagerCoercion {
    expectee_quary: HirQuary,
}
impl TrivialHirEagerCoercion {
    fn place_after_coercion(self) -> HirQuary {
        self.expectee_quary
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DerefHirEagerCoercion {
    Leash,
    Ref { lifetime: HirLifetime },
}
impl DerefHirEagerCoercion {
    fn place_after_coercion(self) -> HirQuary {
        match self {
            DerefHirEagerCoercion::Leash => HirQuary::Leashed { place_idx: None },
            DerefHirEagerCoercion::Ref { lifetime } => HirQuary::Ref {
                guard: Right(lifetime),
            },
        }
    }
}

impl ToHirEager for FlyCoercion {
    type Output = HirEagerCoercion;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        match self {
            FlyCoercion::Trivial(slf) => HirEagerCoercion::Trivial(slf.to_hir_eager(builder)),
            FlyCoercion::Never => HirEagerCoercion::Never,
            FlyCoercion::WrapInSome => HirEagerCoercion::WrapInSome,
            FlyCoercion::Releash => HirEagerCoercion::Releash,
            FlyCoercion::Deref(slf) => HirEagerCoercion::Deref(slf.to_hir_eager(builder)),
        }
    }
}

impl ToHirEager for TrivialFlyCoercion {
    type Output = TrivialHirEagerCoercion;

    fn to_hir_eager(&self, _builder: &mut HirEagerExprBuilder) -> Self::Output {
        TrivialHirEagerCoercion {
            expectee_quary: HirQuary::from_fly(self.expectee_quary),
        }
    }
}

impl ToHirEager for DerefFlyCoercion {
    type Output = DerefHirEagerCoercion;

    fn to_hir_eager(&self, _builder: &mut HirEagerExprBuilder) -> Self::Output {
        match *self {
            DerefFlyCoercion::Leash => DerefHirEagerCoercion::Leash,
            DerefFlyCoercion::Ref { lifetime } => DerefHirEagerCoercion::Ref {
                lifetime: HirLifetime::from_fly(lifetime),
            },
        }
    }
}
