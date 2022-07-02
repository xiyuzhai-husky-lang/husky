use crate::*;

pub(crate) fn is_defn_static(db: &dyn RustCodeGenQueryGroup, entity_route: EntityRoutePtr) -> bool {
    let entity_route = entity_route.deref_route();
    if entity_route.spatial_arguments.len() > 0 {
        return false;
    }
    match entity_route.kind {
        EntityRouteKind::Root { ident } => true,
        EntityRouteKind::Package { main, ident } => false,
        EntityRouteKind::Child { parent, ident } => db.is_defn_static(parent),
        EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => todo!(),
        EntityRouteKind::Input { main } => todo!(),
        EntityRouteKind::Generic { ident, entity_kind } => todo!(),
        EntityRouteKind::ThisType => todo!(),
    }
}
