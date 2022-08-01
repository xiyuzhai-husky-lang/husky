use super::*;
use husky_write_utils::w;
use std::fmt::Write;

pub(crate) fn mangle_ty_vtable(
    db: &dyn RustCodeGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> Arc<String> {
    msg_once!("ad hoc");
    Arc::new(if entity_route.spatial_arguments.len() > 0 {
        let mut result = entity_route.ident().as_str().to_string();
        w!(result; "_");
        for subroute in entity_route.spatial_arguments.iter() {
            let subroute = subroute.take_entity_route();
            write!(result, "_{}", db.mangle_ty_vtable(subroute)).unwrap();
        }
        result
    } else {
        entity_route.ident().as_str().to_string()
    })
}
