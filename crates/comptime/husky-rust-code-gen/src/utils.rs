use crate::*;

pub(crate) fn is_defn_static(db: &dyn RustCodeGenQueryGroup, entity_route: EntityRoutePtr) -> bool {
    let entity_route = entity_route.intrinsic();
    match entity_route.variant {
        EntityRouteVariant::Root { .. } => true,
        EntityRouteVariant::Package { .. } => false,
        EntityRouteVariant::Child { parent, .. } => db.is_defn_static(parent),
        EntityRouteVariant::TypeAsTraitMember { ty, .. } => {
            msg_once!("ad hoc");
            db.is_defn_static(ty)
        }
        EntityRouteVariant::TargetInputValue => todo!(),
        EntityRouteVariant::Any { .. } => todo!(),
        EntityRouteVariant::ThisType { .. } => todo!(),
        EntityRouteVariant::TargetOutputType => todo!(),
    }
}

pub(crate) fn contains_spatial_parameters(
    db: &dyn RustCodeGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> bool {
    let entity_route = entity_route.intrinsic();
    if entity_route.spatial_arguments.len() > 0 {
        return true;
    }
    match entity_route.variant {
        EntityRouteVariant::Root { .. } => false,
        EntityRouteVariant::Package { .. } => false,
        EntityRouteVariant::Child { parent, .. } => db.contains_spatial_parameters(parent),
        EntityRouteVariant::TypeAsTraitMember { ty, .. } => db.contains_spatial_parameters(ty),
        EntityRouteVariant::TargetInputValue => todo!(),
        EntityRouteVariant::Any { .. } => todo!(),
        EntityRouteVariant::ThisType { .. } => todo!(),
        EntityRouteVariant::TargetOutputType => todo!(),
    }
}
