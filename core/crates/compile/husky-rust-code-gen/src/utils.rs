use crate::*;

pub(crate) fn is_defn_static(db: &dyn RustCodeGenQueryGroup, entity_route: EntityRoutePtr) -> bool {
    let entity_route = entity_route.deref_route();
    match entity_route.kind {
        EntityRouteKind::Root { ident } => true,
        EntityRouteKind::Package { main, ident } => false,
        EntityRouteKind::Child { parent, ident } => db.is_defn_static(parent),
        EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
            msg_once!("ad hoc");
            db.is_defn_static(ty)
        }
        EntityRouteKind::Input { main } => todo!(),
        EntityRouteKind::Generic { ident, entity_kind } => todo!(),
        EntityRouteKind::ThisType => todo!(),
    }
}

pub(crate) fn contains_spatial_parameters(
    db: &dyn RustCodeGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> bool {
    let entity_route = entity_route.deref_route();
    if entity_route.spatial_arguments.len() > 0 {
        return true;
    }
    match entity_route.kind {
        EntityRouteKind::Root { ident } => false,
        EntityRouteKind::Package { main, ident } => false,
        EntityRouteKind::Child { parent, ident } => db.contains_spatial_parameters(parent),
        EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
            db.contains_spatial_parameters(ty)
        }
        EntityRouteKind::Input { main } => todo!(),
        EntityRouteKind::Generic { ident, entity_kind } => todo!(),
        EntityRouteKind::ThisType => todo!(),
    }
}
