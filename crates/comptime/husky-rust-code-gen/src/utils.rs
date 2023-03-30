use crate::*;

pub(crate) fn is_defn_static(_db: &dyn RustTranspileDb, _entity_path: Term) -> bool {
    todo!()
    // let entity_path = entity_path.intrinsic();
    // match entity_path.variant {
    //     EntityRouteVariant::Root { .. } => true,
    //     EntityRouteVariant::Package { .. } => false,
    //     EntityRouteVariant::Child { parent, .. } => db.is_defn_static(parent),
    //     EntityRouteVariant::TraitForTypeMember { ty, .. } => {
    //         msg_once!("ad hoc");
    //         db.is_defn_static(ty)
    //     }
    //     EntityRouteVariant::TargetInputValue => todo!(),
    //     EntityRouteVariant::Any { .. } => todo!(),
    //     EntityRouteVariant::ThisType { .. } => todo!(),
    //     EntityRouteVariant::TargetOutputType => todo!(),
    // }
}

pub(crate) fn contains_spatial_parameters(_db: &dyn RustTranspileDb, _entity_path: Term) -> bool {
    todo!()
    // let entity_path = entity_path.intrinsic();
    // if entity_path.spatial_arguments.len() > 0 {
    //     return true;
    // }
    // match entity_path.variant {
    //     EntityRouteVariant::Root { .. } => false,
    //     EntityRouteVariant::Package { .. } => false,
    //     EntityRouteVariant::Child { parent, .. } => db.contains_spatial_parameters(parent),
    //     EntityRouteVariant::TraitForTypeMember { ty, .. } => db.contains_spatial_parameters(ty),
    //     EntityRouteVariant::TargetInputValue => todo!(),
    //     EntityRouteVariant::Any { .. } => todo!(),
    //     EntityRouteVariant::ThisType { .. } => todo!(),
    //     EntityRouteVariant::TargetOutputType => todo!(),
    // }
}
