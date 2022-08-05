use crate::*;

impl Instantiable for EntityRoutePtr {
    type Target = SpatialArgument;

    fn instantiate(&self, ctx: &InstantiationContext) -> SpatialArgument {
        match ctx.db.entity_kind(*self).unwrap() {
            EntityKind::Module => SpatialArgument::EntityRoute(*self),
            EntityKind::EnumLiteral => todo!(),
            _ => {
                let (variant, mut generics) = match self.variant {
                    EntityRouteVariant::Package { .. } => panic!(),
                    EntityRouteVariant::Root { ident } => (self.variant.clone(), thin_vec![]),
                    EntityRouteVariant::Child { parent, ident } => (
                        EntityRouteVariant::Child {
                            parent: parent.instantiate(ctx).take_entity_route(),
                            ident,
                        },
                        thin_vec![],
                    ),
                    EntityRouteVariant::Input { main } => todo!(),
                    EntityRouteVariant::Any { ident, .. } => {
                        if let Some(idx) = ctx.find_generic(ident) {
                            match ctx.spatial_arguments[idx] {
                                SpatialArgument::Const(value) => {
                                    should_eq!(self.spatial_arguments.len(), 0);
                                    return SpatialArgument::Const(value);
                                }
                                SpatialArgument::EntityRoute(route) => {
                                    (route.variant.clone(), route.spatial_arguments.clone())
                                }
                            }
                        } else {
                            p!(ident, ctx.spatial_parameters);
                            todo!()
                        }
                    }
                    EntityRouteVariant::ThisType => (EntityRouteVariant::ThisType, thin_vec![]),
                    EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => (
                        EntityRouteVariant::TypeAsTraitMember {
                            ty: ty.instantiate(ctx).take_entity_route(),
                            trai: trai.instantiate(ctx).take_entity_route(),
                            ident,
                        },
                        thin_vec![],
                    ),
                };
                // convention: A<B,C> = A<B><C>
                generics.extend(self.spatial_arguments.instantiate(ctx));
                SpatialArgument::EntityRoute(ctx.db.intern_entity_route(EntityRoute {
                    variant,
                    temporal_arguments: thin_vec![],
                    spatial_arguments: generics,
                }))
            }
        }
    }
}
