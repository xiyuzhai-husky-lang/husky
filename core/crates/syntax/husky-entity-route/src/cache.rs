use crate::*;
use check_utils::should;
use print_utils::epin;
use singleton::singleton;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Default)]
pub struct TyRouteCache {
    data: Mutex<HashMap<std::any::TypeId, EntityRoutePtr>>,
}

singleton! { TyRouteCache }

pub fn new_ty_route_cache() -> Arc<TyRouteCacheSingletonKeeper> {
    Arc::new(TyRouteCache::default().into())
}

fn ty_route_cache() -> &'static TyRouteCache {
    __access_singleton()
}

pub fn try_get_ty_route(type_id: std::any::TypeId) -> Option<EntityRoutePtr> {
    let ty_route_cache = &ty_route_cache();
    ty_route_cache
        .data
        .lock()
        .unwrap()
        .get(&type_id)
        .map(|e| *e)
}

pub fn insert_new_ty_route(type_id: std::any::TypeId, ty: EntityRoutePtr) {
    should!(ty_route_cache()
        .data
        .lock()
        .unwrap()
        .insert(type_id, ty)
        .is_none())
}

pub fn ty_route_with<T: 'static>(f: impl FnOnce() -> EntityRoutePtr) -> EntityRoutePtr {
    let type_id = std::any::TypeId::of::<T>();
    if let Some(ty) = try_get_ty_route(type_id) {
        ty
    } else {
        let ty = f();
        insert_new_ty_route(type_id, ty);
        ty
    }
}
