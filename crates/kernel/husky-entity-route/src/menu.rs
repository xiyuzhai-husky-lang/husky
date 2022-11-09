use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityRouteMenu {
    pub clone_trait: EntityRouteItd,
    pub copy_trait: EntityRouteItd,
    pub void_type: EntityRouteItd,
    pub i32_ty: EntityRouteItd,
    pub vec_ty: EntityRouteItd,
    pub std_ops_index_trai: EntityRouteItd,
    pub std_mod: EntityRouteItd,
    pub std_ops_mod: EntityRouteItd,
    pub std_slice_cyclic_slice: EntityRouteItd,
}
