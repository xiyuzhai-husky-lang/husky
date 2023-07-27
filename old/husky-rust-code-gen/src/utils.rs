use crate::*;

pub(crate) fn is_defn_static(_db: &dyn RustTranspileDb, _item_path: EtherealTerm) -> bool {
    todo!()
    // let item_path = item_path.intrinsic();
    // match item_path.variant {
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

pub(crate) fn contains_spatial_parameters(
    _db: &dyn RustTranspileDb,
    _item_path: EtherealTerm,
) -> bool {
    todo!()
    // let item_path = item_path.intrinsic();
    // if item_path.spatial_arguments.len() > 0 {
    //     return true;
    // }
    // match item_path.variant {
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
