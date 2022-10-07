use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityRouteMenu {
    pub clone_trait: EntityRoutePtr,
    pub copy_trait: EntityRoutePtr,
    pub void_type: EntityRoutePtr,
    pub i32_ty: EntityRoutePtr,
    pub vec_ty: EntityRoutePtr,
    pub std_ops_index_trai: EntityRoutePtr,
    pub std_mod: EntityRoutePtr,
    pub std_ops_mod: EntityRoutePtr,
    pub std_slice_cyclic_slice: EntityRoutePtr,
}
