use crate::*;

impl EntityRoutePtr {
    pub fn verify_consistency_with_base_route(self, base_route: EntityRoutePtr) {
        match base_route.variant {
            EntityRouteVariant::Any { .. } => return,
            _ => assert_eq!(
                std::mem::discriminant(&self.variant),
                std::mem::discriminant(&base_route.variant)
            ),
        }
        match self.variant {
            EntityRouteVariant::Root { .. } => assert_eq!(self.ident(), base_route.ident()),
            EntityRouteVariant::Package { .. } => (),
            EntityRouteVariant::Child { parent, ident } => match base_route.variant {
                EntityRouteVariant::Child {
                    parent: other_parent,
                    ident: other_ident,
                } => {
                    assert_eq!(ident, other_ident);
                    parent.verify_consistency_with_base_route(other_parent);
                }
                _ => panic!(),
            },
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => match base_route.variant {
                EntityRouteVariant::Root { .. } => todo!(),
                EntityRouteVariant::Package { .. } => todo!(),
                EntityRouteVariant::Child { .. } => todo!(),
                EntityRouteVariant::TypeAsTraitMember {
                    ty: other_ty,
                    trai: other_trai,
                    ident: other_ident,
                } => {
                    ty.verify_consistency_with_base_route(other_ty);
                    trai.verify_consistency_with_base_route(other_trai);
                    assert_eq!(ident, other_ident)
                }
                EntityRouteVariant::TargetInputValue => todo!(),
                EntityRouteVariant::TargetOutputType => todo!(),
                EntityRouteVariant::Any { .. } => todo!(),
                EntityRouteVariant::ThisType { .. } => todo!(),
            },
            EntityRouteVariant::TargetInputValue => todo!(),
            EntityRouteVariant::TargetOutputType => todo!(),
            EntityRouteVariant::Any { .. } => todo!(),
            EntityRouteVariant::ThisType { .. } => todo!(),
        }
    }
}
