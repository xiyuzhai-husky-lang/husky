use crate::*;
use either::*;
use husky_fly_term::{
    deref::DedirectionFlyCoercion, redirection::RedirectionFlyCoercion, trival::TrivialFlyCoercion,
    FlyCoercion,
};
use husky_hir_ty::{lifetime::HirLifetime, quary::HirQuary};

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirEagerCoercion {
    Trivial(TrivialHirEagerCoercion),
    Never,
    WrapInSome,
    Redirection(RedirectionHirEagerCoercion),
    Dedirection(DedirectionHirEagerCoercion),
}

impl HirEagerCoercion {
    pub const TRIVIAL_TRANSIENT: Self = HirEagerCoercion::Trivial(TrivialHirEagerCoercion {
        expectee_quary: HirQuary::Transient,
    });

    pub fn quary_after_coercion(self) -> HirQuary {
        match self {
            HirEagerCoercion::Trivial(slf) => slf.place_after_coercion(),
            HirEagerCoercion::Dedirection(slf) => slf.place_after_coercion(),
            HirEagerCoercion::Never
            | HirEagerCoercion::WrapInSome
            | HirEagerCoercion::Redirection(_) => HirQuary::Transient,
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
pub enum RedirectionHirEagerCoercion {
    Releash,
    Reref,
    RerefMut,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DedirectionHirEagerCoercion {
    Deleash,
    Deref { lifetime: HirLifetime },
    DerefMut,
}

impl DedirectionHirEagerCoercion {
    fn place_after_coercion(self) -> HirQuary {
        match self {
            DedirectionHirEagerCoercion::Deleash => HirQuary::Leashed { place_idx: None },
            DedirectionHirEagerCoercion::Deref { lifetime } => HirQuary::Ref {
                guard: Right(lifetime),
            },
            DedirectionHirEagerCoercion::DerefMut => todo!(),
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
            FlyCoercion::Redirection(slf) => {
                HirEagerCoercion::Redirection(slf.to_hir_eager(builder))
            }
            FlyCoercion::Dedirection(slf) => {
                HirEagerCoercion::Dedirection(slf.to_hir_eager(builder))
            }
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

impl ToHirEager for RedirectionFlyCoercion {
    type Output = RedirectionHirEagerCoercion;

    fn to_hir_eager(&self, _builder: &mut HirEagerExprBuilder) -> Self::Output {
        match *self {
            RedirectionFlyCoercion::Releash => RedirectionHirEagerCoercion::Releash,
            RedirectionFlyCoercion::Reref => RedirectionHirEagerCoercion::Reref,
            RedirectionFlyCoercion::RerefMut => RedirectionHirEagerCoercion::RerefMut,
        }
    }
}

impl ToHirEager for DedirectionFlyCoercion {
    type Output = DedirectionHirEagerCoercion;

    fn to_hir_eager(&self, _builder: &mut HirEagerExprBuilder) -> Self::Output {
        match *self {
            DedirectionFlyCoercion::Deleash => DedirectionHirEagerCoercion::Deleash,
            DedirectionFlyCoercion::Deref { lifetime } => DedirectionHirEagerCoercion::Deref {
                lifetime: HirLifetime::from_fly(lifetime),
            },
        }
    }
}
