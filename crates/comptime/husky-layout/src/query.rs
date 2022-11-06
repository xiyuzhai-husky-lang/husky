use husky_entity_route::EntityRoutePtr;

#[salsa::query_group(HuskyLayoutQueryGroupStorage)]
pub trait HuskyLayoutQueryGroup {
    fn reg_memory_kind(&self, ty: EntityRoutePtr) -> RegMemoryKind;
}

fn reg_memory_kind(db: &dyn HuskyLayoutQueryGroup, ty: EntityRoutePtr) -> RegMemoryKind {
    todo!()
    // let ty = ty.intrinsic();
    // if ty.is_primitive() {
    //     RegMemoryKind::Direct
    // } else {
    //     if db.is_copyable(ty).unwrap() {
    //         RegMemoryKind::BoxCopyable
    //     } else {
    //         RegMemoryKind::BoxNonCopyable
    //     }
    // }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegMemoryKind {
    Direct,
    BoxCopyable,
    BoxNonCopyable,
}

impl std::fmt::Display for RegMemoryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegMemoryKind::Direct => "Direct",
            RegMemoryKind::BoxCopyable => "BoxCopyable",
            RegMemoryKind::BoxNonCopyable => "BoxNonCopyable",
        }
        .fmt(f)
    }
}
