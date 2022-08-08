use crate::*;
use husky_print_utils::epin;
use husky_word::{intern_word, RootIdentifier};
use singleton::singleton;
use std::sync::Arc;
use thin_vec::thin_vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityRouteMenu {
    pub clone_trait: EntityRoutePtr,
    pub copy_trait: EntityRoutePtr,
    pub void_type: EntityRoutePtr,
    pub i32_ty: EntityRoutePtr,
    pub this_ty: EntityRoutePtr,
    pub vec_ty: EntityRoutePtr,
    pub std_ops_index_trai: EntityRoutePtr,
    pub std_mod: EntityRoutePtr,
    pub std_ops_mod: EntityRoutePtr,
    pub std_slice_cyclic_slice: EntityRoutePtr,
}

singleton! { EntityRouteMenu }

pub fn entity_route_menu() -> &'static EntityRouteMenu {
    __access_singleton()
}

pub fn new_entity_route_menu() -> Arc<EntityRouteMenuSingletonKeeper> {
    let std_mod = EntityRoutePtr::Root(RootIdentifier::Std);
    let std_ops_mod = make_subroute(std_mod, intern_word("ops").custom(), thin_vec![]);
    let std_ops_index_trai = make_subroute(std_ops_mod, intern_word("Index").custom(), thin_vec![]);
    let std_slice_mod = make_subroute(std_mod, intern_word("slice").custom(), thin_vec![]);
    let std_slice_cyclic_slice = make_subroute(
        std_slice_mod,
        intern_word("CyclicSlice").custom(),
        thin_vec![],
    );
    Arc::new(
        EntityRouteMenu {
            clone_trait: EntityRoutePtr::Root(RootIdentifier::CloneTrait),
            copy_trait: EntityRoutePtr::Root(RootIdentifier::CopyTrait),
            void_type: EntityRoutePtr::Root(RootIdentifier::Void),
            i32_ty: EntityRoutePtr::Root(RootIdentifier::I32),
            this_ty: EntityRoutePtr::ThisType,
            vec_ty: EntityRoutePtr::Root(RootIdentifier::Vec),
            std_mod,
            std_ops_mod,
            std_ops_index_trai,
            std_slice_cyclic_slice,
        }
        .into(),
    )
}
