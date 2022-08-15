use crate::EntityRoutePtr;

// todo: mutable ref
pub struct CanonicalEntityRoutePtr {
    intrinsic: EntityRoutePtr,
    kind: CanonicalEntityRoutePtrKind,
}

impl std::fmt::Display for CanonicalEntityRoutePtrKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanonicalEntityRoutePtrKind::Intrinsic => "Intrinsic",
            CanonicalEntityRoutePtrKind::Optional => "Optional",
            CanonicalEntityRoutePtrKind::EvalRef => "EvalRef",
            CanonicalEntityRoutePtrKind::OptionalEvalRef => "OptionalEvalRef",
        }
        .fmt(f)
    }
}

impl CanonicalEntityRoutePtr {
    pub(crate) fn new(intrinsic: EntityRoutePtr, kind: CanonicalEntityRoutePtrKind) -> Self {
        assert!(intrinsic.is_intrinsic());
        Self { intrinsic, kind }
    }

    pub fn intrinsic(&self) -> EntityRoutePtr {
        self.intrinsic
    }

    pub fn kind(&self) -> CanonicalEntityRoutePtrKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalEntityRoutePtrKind {
    Intrinsic,
    Optional,
    EvalRef,
    OptionalEvalRef,
}
