use crate::EntityRoutePtr;

// todo: mutable ref
pub struct CanonicalEntityRoutePtr {
    intrinsic: EntityRoutePtr,
    kind: CanonicalEntityRoutePtrKind,
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
    Ref,
    OptionalRef,
}
