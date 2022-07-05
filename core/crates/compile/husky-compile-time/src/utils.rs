use husky_entity_route::{insert_new_ty_route, try_get_ty_route};

use crate::*;

impl HuskyCompileTime {
    pub fn ty_route_from_static(&self, type_id: std::any::TypeId, text: &str) -> EntityRoutePtr {
        if let Some(ty) = try_get_ty_route(type_id) {
            ty
        } else {
            let ty = self.parse_route_from_text(text);
            insert_new_ty_route(type_id, ty);
            ty
        }
    }
}
