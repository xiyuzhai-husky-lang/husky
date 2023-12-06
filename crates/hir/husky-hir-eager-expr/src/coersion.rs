use crate::*;
use husky_fluffy_term::{
    deref::DerefFluffyCoersion, trival::TrivialFluffyCoersion, FluffyCoersion,
};
use husky_hir_ty::{lifetime::HirLifetime, place::HirPlace};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirEagerCoersion {
    Trivial(TrivialHirEagerCoersion),
    Never,
    WrapInSome,
    PlaceToLeash,
    Deref(DerefHirEagerCoersion),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TrivialHirEagerCoersion {
    expectee_place: HirPlace,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DerefHirEagerCoersion {
    Leash,
    Ref { lifetime: HirLifetime },
}

impl ToHirEager for FluffyCoersion {
    type Output = HirEagerCoersion;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        match self {
            FluffyCoersion::Trivial(slf) => HirEagerCoersion::Trivial(slf.to_hir_eager(builder)),
            FluffyCoersion::Never => HirEagerCoersion::Never,
            FluffyCoersion::WrapInSome => HirEagerCoersion::WrapInSome,
            FluffyCoersion::PlaceToLeash => HirEagerCoersion::PlaceToLeash,
            FluffyCoersion::Deref(slf) => HirEagerCoersion::Deref(slf.to_hir_eager(builder)),
        }
    }
}

impl ToHirEager for TrivialFluffyCoersion {
    type Output = TrivialHirEagerCoersion;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        TrivialHirEagerCoersion {
            expectee_place: HirPlace::from_fluffy(self.expectee_place),
        }
    }
}

impl ToHirEager for DerefFluffyCoersion {
    type Output = DerefHirEagerCoersion;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        match *self {
            DerefFluffyCoersion::Leash => DerefHirEagerCoersion::Leash,
            DerefFluffyCoersion::Ref { lifetime } => DerefHirEagerCoersion::Ref {
                lifetime: HirLifetime::from_fluffy(lifetime),
            },
        }
    }
}
