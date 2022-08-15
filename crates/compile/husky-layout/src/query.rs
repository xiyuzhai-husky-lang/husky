use husky_entity_route::EntityRoutePtr;

#[salsa::query_group(HuskyLayoutQueryGroupStorage)]
pub trait HuskyLayoutQueryGroup {
    fn reg_memory_kind(&self, ty: EntityRoutePtr) -> RegMemoryKind;
}

fn reg_memory_kind(db: &dyn HuskyLayoutQueryGroup, ty: EntityRoutePtr) -> RegMemoryKind {
    // ad hoc
    assert!(ty.is_intrinsic());
    if ty.is_primitive() {
        RegMemoryKind::Direct
    } else {
        RegMemoryKind::Box
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegMemoryKind {
    Direct,
    Box,
}

impl std::fmt::Display for RegMemoryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegMemoryKind::Direct => "direct",
            RegMemoryKind::Box => "box",
        }
        .fmt(f)
    }
}
