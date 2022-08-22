use crate::*;
use husky_check_utils::should;
use husky_entity_route::SpatialArgument;
use husky_instantiate::InstantiationContext;
use husky_linkage_table::LinkageKey;
use husky_word::RootIdentifier;
use std::collections::{HashMap, HashSet};
mod context;
mod impl_entity;
mod impl_expr;
mod impl_stmt;
pub use context::*;

pub(crate) struct LinkageCollector<'a> {
    db: &'a dyn RustCodeGenQueryGroup,
    linkages: VecSet<EntityRoutePtr>,
    context: LinkageCollectorContext,
}

impl<'a> LinkageCollector<'a> {
    pub(crate) fn insert(&mut self, entity_route: EntityRoutePtr) {
        match entity_route.variant {
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => {
                if trai == self.db.entity_route_menu().clone_trait {
                    return;
                }
            }
            EntityRouteVariant::TargetInputValue { .. } => return,
            EntityRouteVariant::Root {
                ident: RootIdentifier::Vec,
            } => {
                // ad hoc
                if entity_route.spatial_arguments.len() > 0 {
                    self.insert(self.db.subroute(
                        entity_route,
                        self.db.intern_word("ilen").custom(),
                        Default::default(),
                    ))
                }
            }
            _ => (),
        }
        for argument in entity_route.spatial_arguments.iter() {
            match argument {
                SpatialArgument::Const(_) => (),
                SpatialArgument::EntityRoute(route) => self.insert(*route),
            }
        }
        self.linkages.insert(entity_route.intrinsic())
    }

    fn produce_from_entity_defn(
        mut self,
        entity_route: EntityRoutePtr,
    ) -> Arc<VecSet<EntityRoutePtr>> {
        let defn = self.db.entity_defn(entity_route).unwrap();
        self.collect_from_entity_defn(&defn);
        Arc::new(self.linkages)
    }
}

pub(crate) fn entity_immediate_link_dependees(
    db: &dyn RustCodeGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> Arc<VecSet<EntityRoutePtr>> {
    if entity_route.spatial_arguments.len() > 0 {
        let entity_defn = db.entity_defn(entity_route).unwrap();
        let spatial_parameters = entity_defn.spatial_parameters();
        let ctx = InstantiationContext {
            db: db.upcast(),
            spatial_parameters,
            spatial_arguments: &entity_route.spatial_arguments,
        };
        use husky_instantiate::Instantiable;
        let mut set: VecSet<_> = db
            .entity_immediate_link_dependees(db.base_route(entity_route))
            .iter()
            .map(|entity_route| {
                entity_route
                    .instantiate(&ctx)
                    .take_entity_route()
                    .intrinsic()
            })
            .collect();
        for spatial_argument in &entity_route.spatial_arguments {
            match spatial_argument {
                SpatialArgument::Const(_) => (),
                SpatialArgument::EntityRoute(route) => set.insert(route.intrinsic()),
            }
        }
        Arc::new(set)
    } else {
        LinkageCollector {
            db,
            linkages: Default::default(),
            context: LinkageCollectorContext::Base,
        }
        .produce_from_entity_defn(entity_route)
    }
}

pub(crate) fn entity_link_dependees(
    db: &dyn RustCodeGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> Arc<VecSet<EntityRoutePtr>> {
    let mut dependees = (*db.entity_immediate_link_dependees(entity_route)).clone();
    visit_all(db, &mut dependees, 0);
    return Arc::new(dependees);

    fn visit_all(
        db: &dyn RustCodeGenQueryGroup,
        dependees: &mut VecSet<EntityRoutePtr>,
        start: usize,
    ) {
        let len0 = dependees.len();
        for subroute in dependees
            .iter()
            .map(|subroute| *subroute)
            .collect::<Vec<_>>()
        {
            match subroute.variant {
                EntityRouteVariant::Any { .. } => continue,
                _ => (),
            }
            let subroute_dependees = db.entity_immediate_link_dependees(subroute.intrinsic());
            dependees.extend(&subroute_dependees)
        }
        if dependees.len() > len0 {
            visit_all(db, dependees, len0)
        }
    }
}
