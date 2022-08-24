use crate::EntityRoutePtr;

// todo: mutable ref
pub struct CanonicalEntityRoutePtr {
    intrinsic_route: EntityRoutePtr,
    kind: CanonicalEntityRoutePtrKind,
}

impl std::fmt::Display for CanonicalEntityRoutePtrKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanonicalEntityRoutePtrKind::Intrinsic => "Intrinsic",
            CanonicalEntityRoutePtrKind::Optional => "Optional",
            CanonicalEntityRoutePtrKind::EvalRef => "EvalRef",
            CanonicalEntityRoutePtrKind::OptionalEvalRef => "OptionalEvalRef",
            CanonicalEntityRoutePtrKind::TempRefMut => todo!(),
        }
        .fmt(f)
    }
}

impl CanonicalEntityRoutePtr {
    pub(crate) fn new(intrinsic_route: EntityRoutePtr, kind: CanonicalEntityRoutePtrKind) -> Self {
        assert!(intrinsic_route.is_intrinsic());
        Self {
            intrinsic_route,
            kind,
        }
    }

    pub fn intrinsic_route(&self) -> EntityRoutePtr {
        self.intrinsic_route
    }

    pub fn kind(&self) -> CanonicalEntityRoutePtrKind {
        self.kind
    }

    pub fn is_intrinsic_route_primitive(&self) -> bool {
        self.intrinsic_route.is_primitive()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalEntityRoutePtrKind {
    Intrinsic,
    Optional,
    EvalRef,
    OptionalEvalRef,
    TempRefMut,
}
