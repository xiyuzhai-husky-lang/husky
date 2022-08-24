use crate::*;

impl EntityRoute {
    pub fn is_self_ty_alias(&self) -> bool {
        matches!(self.variant, EntityRouteVariant::ThisType { .. })
    }

    pub fn is_implemented(&self) -> bool {
        for spatial_argument in self.spatial_arguments.iter() {
            match spatial_argument {
                SpatialArgument::Const(_) => todo!(),
                SpatialArgument::EntityRoute(route) => {
                    if !route.is_implemented() {
                        return false;
                    }
                }
            }
        }
        match self.variant {
            EntityRouteVariant::Root { ident } => true,
            EntityRouteVariant::Package { main, ident } => todo!(),
            EntityRouteVariant::Child { parent, ident } => parent.is_implemented(),
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => ty.is_implemented(),
            EntityRouteVariant::TargetInputValue => todo!(),
            EntityRouteVariant::TargetOutputType => todo!(),
            EntityRouteVariant::Any {
                ident,
                husky_entity_kind,
                file,
                range,
            } => true,
            EntityRouteVariant::ThisType { file, range } => false,
        }
    }
}
