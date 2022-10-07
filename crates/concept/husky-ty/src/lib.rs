use husky_entity_route::EntityRoutePtr;

mod calculus;
mod intern;

pub enum TyExpr {
    Entity(EntityRoutePtr),
    TemplateInstantiation {
        template: EntityRoutePtr,
        temporal_arguments: Vec<TemporalArgument>,
        spatial_arguments: Vec<SpatialArgument>,
    },
    Prop, // reserved
}

pub enum TemporalArgument {
    Lifetime,
}

pub enum SpatialArgument {
    Lifetime,
}
