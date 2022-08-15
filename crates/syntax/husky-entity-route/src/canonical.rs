use crate::EntityRoutePtr;

// todo: mutable ref
pub struct CanonicalEntityRoutePtr {
    pub intrinsic: EntityRoutePtr,
    pub is_option: bool,
    pub is_ref: bool,
}

impl CanonicalEntityRoutePtr {
    pub fn kind(&self) -> CanonicalEntityRoutePtrKind {
        match (self.is_option, self.is_ref) {
            (true, true) => CanonicalEntityRoutePtrKind::OptionalRef,
            (true, false) => CanonicalEntityRoutePtrKind::Optional,
            (false, true) => CanonicalEntityRoutePtrKind::Ref,
            (false, false) => CanonicalEntityRoutePtrKind::Intrinsic,
        }
    }
}

pub enum CanonicalEntityRoutePtrKind {
    Intrinsic,
    Optional,
    Ref,
    OptionalRef,
}
