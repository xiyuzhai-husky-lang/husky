use crate::EntityRouteItd;

// todo: mutable ref
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanonicalTy {
    // ??f32 option_level = 2
    // ?[]?f32 option_level = 1
    option_level: u8,
    qual: CanonicalQualifier,
    intrinsic_ty: EntityRouteItd,
}

impl From<EntityRouteItd> for CanonicalTy {
    fn from(route: EntityRouteItd) -> Self {
        route.canonicalize()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CanonicalQualifier {
    Intrinsic,
    EvalRef,
    TempRef,
    TempRefMut,
}

impl std::fmt::Display for CanonicalTyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanonicalTyKind::Intrinsic => "Intrinsic",
            CanonicalTyKind::Optional => "Optional",
            CanonicalTyKind::EvalRef => "EvalRef",
            CanonicalTyKind::OptionalEvalRef => "OptionalEvalRef",
            CanonicalTyKind::TempRefMut => todo!(),
        }
        .fmt(f)
    }
}

impl CanonicalTy {
    #[inline(always)]
    pub fn intrinsic_ty(self) -> EntityRouteItd {
        self.intrinsic_ty
    }
    pub fn new(option_level: u8, qual: CanonicalQualifier, intrinsic_ty: EntityRouteItd) -> Self {
        assert!(intrinsic_ty.is_intrinsic());
        Self {
            option_level,
            qual,
            intrinsic_ty,
        }
    }

    pub fn with_eval_ref(self) -> Self {
        Self {
            option_level: self.option_level,
            qual: match self.qual {
                CanonicalQualifier::Intrinsic | CanonicalQualifier::EvalRef => {
                    CanonicalQualifier::EvalRef
                }
                CanonicalQualifier::TempRef => todo!(),
                CanonicalQualifier::TempRefMut => todo!(),
            },
            intrinsic_ty: self.intrinsic_ty,
        }
    }

    pub fn with_option(self) -> Self {
        Self {
            option_level: self.option_level + 1,
            qual: self.qual,
            intrinsic_ty: self.intrinsic_ty,
        }
    }

    pub fn option_level(&self) -> u8 {
        self.option_level
    }

    pub fn kind(&self) -> CanonicalTyKind {
        match (self.option_level, self.qual) {
            (1, CanonicalQualifier::Intrinsic) => CanonicalTyKind::Optional,
            (1, CanonicalQualifier::EvalRef) => CanonicalTyKind::OptionalEvalRef,
            (1, CanonicalQualifier::TempRef) => todo!(),
            (1, CanonicalQualifier::TempRefMut) => todo!(),
            (0, CanonicalQualifier::Intrinsic) => CanonicalTyKind::Intrinsic,
            (0, CanonicalQualifier::EvalRef) => CanonicalTyKind::EvalRef,
            (0, CanonicalQualifier::TempRef) => todo!(),
            (0, CanonicalQualifier::TempRefMut) => CanonicalTyKind::TempRefMut,
            _ => todo!(),
        }
    }

    pub fn is_intrinsic_route_primitive(&self) -> bool {
        self.intrinsic_ty.is_primitive()
    }

    pub fn qual(self) -> CanonicalQualifier {
        self.qual
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalTyKind {
    Intrinsic,
    Optional,
    EvalRef,
    OptionalEvalRef,
    TempRefMut,
}
