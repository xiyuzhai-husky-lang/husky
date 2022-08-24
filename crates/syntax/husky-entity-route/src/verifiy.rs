use crate::*;

impl EntityRoutePtr {
    pub fn verify_consistency_with_base_route(self, other: EntityRoutePtr) {
        assert_eq!(
            std::mem::discriminant(&self.variant),
            std::mem::discriminant(&other.variant)
        );
        match self.variant {
            EntityRouteVariant::Root { ident } => assert_eq!(self.ident(), other.ident()),
            EntityRouteVariant::Package { main, ident } => todo!(),
            EntityRouteVariant::Child { parent, ident } => match other.variant {
                EntityRouteVariant::Child {
                    parent: other_parent,
                    ident: other_ident,
                } => {
                    assert_eq!(ident, other_ident);
                    parent.verify_consistency_with_base_route(other_parent);
                }
                _ => panic!(),
            },
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => match other.variant {
                EntityRouteVariant::Root { ident } => todo!(),
                EntityRouteVariant::Package { main, ident } => todo!(),
                EntityRouteVariant::Child { parent, ident } => todo!(),
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
                EntityRouteVariant::Any {
                    ident,
                    husky_entity_kind,
                    file,
                    range,
                } => todo!(),
                EntityRouteVariant::ThisType { file, range } => todo!(),
            },
            EntityRouteVariant::TargetInputValue => todo!(),
            EntityRouteVariant::TargetOutputType => todo!(),
            EntityRouteVariant::Any {
                ident,
                husky_entity_kind,
                file,
                range,
            } => todo!(),
            EntityRouteVariant::ThisType { file, range } => todo!(),
        }
    }
}
