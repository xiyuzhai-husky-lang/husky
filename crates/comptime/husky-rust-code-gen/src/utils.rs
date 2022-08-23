use crate::*;

pub(crate) fn is_defn_static(db: &dyn RustCodeGenQueryGroup, entity_route: EntityRoutePtr) -> bool {
    let entity_route = entity_route.intrinsic();
    match entity_route.variant {
        EntityRouteVariant::Root { ident } => true,
        EntityRouteVariant::Package { main, ident } => false,
        EntityRouteVariant::Child { parent, ident } => db.is_defn_static(parent),
        EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => {
            msg_once!("ad hoc");
            db.is_defn_static(ty)
        }
        EntityRouteVariant::TargetInputValue => todo!(),
        EntityRouteVariant::Any {
            ident,
            husky_entity_kind,
            ..
        } => todo!(),
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
        EntityRouteVariant::Root { ident } => false,
        EntityRouteVariant::Package { main, ident } => false,
        EntityRouteVariant::Child { parent, ident } => db.contains_spatial_parameters(parent),
        EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => {
            db.contains_spatial_parameters(ty)
        }
        EntityRouteVariant::TargetInputValue => todo!(),
        EntityRouteVariant::Any {
            ident,
            husky_entity_kind,
            ..
        } => todo!(),
        EntityRouteVariant::ThisType { .. } => todo!(),
        EntityRouteVariant::TargetOutputType => todo!(),
    }
}
