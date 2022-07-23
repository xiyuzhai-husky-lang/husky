use husky_entity_route::{EntityRoute, EntityRouteKind, EntityRoutePtr, SpatialArgument};
use husky_entity_syntax::EntitySyntaxSalsaQueryGroup;
use husky_print_utils::p;
use thin_vec::thin_vec;
use word::CustomIdentifier;

pub struct ImplementationContext<'a> {
    db: &'a dyn EntitySyntaxSalsaQueryGroup,
    this_ty: EntityRoutePtr,
    member_impls: &'a [(CustomIdentifier, SpatialArgument)],
}

impl<'a> ImplementationContext<'a> {
    pub fn new(
        db: &'a dyn EntitySyntaxSalsaQueryGroup,
        this_ty: EntityRoutePtr,
        member_impls: &'a [(CustomIdentifier, SpatialArgument)],
    ) -> Self {
        Self {
            db,
            this_ty,
            member_impls,
        }
    }

    pub fn opt_spatial_argument(&self, ident0: CustomIdentifier) -> Option<SpatialArgument> {
        self.member_impls
            .iter()
            .find_map(|(ident, generic_argument)| {
                if *ident == ident0 {
                    Some(*generic_argument)
                } else {
                    None
                }
            })
    }

    pub fn spatial_argument(&self, ident: CustomIdentifier) -> SpatialArgument {
        self.opt_spatial_argument(ident).unwrap()
    }
}

impl<'a> std::fmt::Debug for ImplementationContext<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Implementor")
            .field("this_ty", &self.this_ty)
            .finish()
    }
}

pub trait Implementable {
    type Target;

    fn implement(&self, implementor: &ImplementationContext) -> Self::Target;
}

impl Implementable for EntityRoutePtr {
    type Target = SpatialArgument;

    fn implement(&self, ctx: &ImplementationContext) -> Self::Target {
        let (kind, mut spatial_arguments) = match self.kind {
            EntityRouteKind::Root { ident } => todo!(),
            EntityRouteKind::Package { main, ident } => todo!(),
            EntityRouteKind::Child { parent, ident } => match parent.kind {
                EntityRouteKind::ThisType => {
                    let route = ctx.spatial_argument(ident).take_entity_route();
                    (route.kind, route.spatial_arguments.clone())
                }
                _ => {
                    p!(parent);
                    todo!()
                }
            },
            EntityRouteKind::Input { main } => todo!(),
            EntityRouteKind::Generic { ident, entity_kind } => todo!(),
            EntityRouteKind::ThisType => (ctx.this_ty.kind, ctx.this_ty.spatial_arguments.clone()),
            EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => match ty.kind {
                EntityRouteKind::ThisType => {
                    if let Some(spatial_argument) = ctx.opt_spatial_argument(ident) {
                        match spatial_argument {
                            SpatialArgument::EntityRoute(_) => todo!(),
                            SpatialArgument::Const(_) => todo!(),
                        }
                    } else {
                        (self.kind, thin_vec![])
                    }
                    // (route.kind, route.spatial_arguments.clone())
                }
                _ => {
                    p!(ty);
                    todo!()
                }
            },
        };
        for spatial_argument in self.spatial_arguments.iter() {
            spatial_arguments.push(spatial_argument.implement(ctx))
        }
        SpatialArgument::EntityRoute(ctx.db.intern_entity_route(EntityRoute {
            kind,
            temporal_arguments: thin_vec![],
            spatial_arguments,
        }))
    }
}

impl Implementable for SpatialArgument {
    type Target = Self;

    fn implement(&self, implementor: &ImplementationContext) -> Self::Target {
        match self {
            SpatialArgument::Const(value) => SpatialArgument::Const(*value),
            SpatialArgument::EntityRoute(entity_route) => entity_route.implement(implementor),
        }
    }
}
