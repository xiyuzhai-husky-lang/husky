use crate::*;
use husky_check_utils::should;
use husky_print_utils::epin;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Default)]
pub struct TyRouteCache {
    data: Mutex<HashMap<std::any::TypeId, EntityRoutePtr>>,
}

// pub fn new_ty_route_cache() -> Arc<TyRouteCacheSingletonKeeper> {
//     Arc::new(TyRouteCache::default().into())
// }

// pub fn ty_route_with<T: 'static>(f: impl FnOnce() -> EntityRoutePtr) -> EntityRoutePtr {
//     let type_id = std::any::TypeId::of::<T>();
//     if let Some(ty) = try_get_ty_route(type_id) {
//         ty
//     } else {
//         let ty = f();
//         insert_new_ty_route(type_id, ty);
//         ty
//     }
// }
