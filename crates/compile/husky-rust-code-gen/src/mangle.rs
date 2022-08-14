use super::*;
use convert_case::{Case, Casing};
use husky_write_utils::w;
use std::fmt::Write;

pub(crate) fn mangled_intrinsic_ty(
    db: &dyn RustCodeGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> Arc<String> {
    db.mangled_ty(entity_route.intrinsic())
}

pub(crate) fn mangled_ty(
    db: &dyn RustCodeGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> Arc<String> {
    msg_once!("ad hoc");
    Arc::new(if entity_route.spatial_arguments.len() > 0 {
        let mut result = entity_route.ident().as_str().to_string();
        w!(result; "_");
        for subroute in entity_route.spatial_arguments.iter() {
            let subroute = subroute.take_entity_route();
            write!(result, "_{}", db.mangled_ty(subroute)).unwrap();
        }
        result
    } else {
        entity_route.ident().as_str().to_string()
    })
}

pub(crate) fn mangled_intrinsic_ty_vtable(
    db: &dyn RustCodeGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> Arc<String> {
    db.mangled_ty_vtable(entity_route.intrinsic())
}
pub(crate) fn mangled_ty_vtable(
    db: &dyn RustCodeGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> Arc<String> {
    Arc::new(match entity_route {
        EntityRoutePtr::Root(_) => {
            format!("__{}_VTABLE", entity_route.ident().as_str().to_uppercase())
        }
        EntityRoutePtr::Custom(_) => format!(
            "__{}_VTABLE",
            &db.mangled_ty(entity_route).to_case(Case::UpperSnake)
        ),
        EntityRoutePtr::ThisType => todo!(),
    })
}
