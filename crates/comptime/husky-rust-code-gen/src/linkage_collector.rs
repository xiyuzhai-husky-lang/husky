use crate::*;
mod impl_entity;
mod impl_expr;
mod impl_stmt;

pub(crate) struct LinkageCollector<'a> {
    db: &'a dyn RustTranspileDb,
    linkages: VecSet<Term>,
}

impl<'a> LinkageCollector<'a> {
    pub(crate) fn insert(&mut self, _entity_path: Term) {
        todo!()
        // match entity_path.variant {
        //     EntityRouteVariant::TypeAsTraitMember { trai, .. } => {
        //         if trai == self.db.entity_route_menu().clone_trait {
        //             return;
        //         }
        //     }
        //     EntityRouteVariant::TargetInputValue { .. } => return,
        //     EntityRouteVariant::Root {
        //         ident: RootBuiltinIdentifier::Vec,
        //     } => {
        //         // ad hoc
        //         if entity_path.spatial_arguments.len() > 0 {
        //             self.insert(self.db.subroute(
        //                 entity_path,
        //                 self.db.it_word("ilen").custom(),
        //                 Default::default(),
        //             ))
        //         }
        //     }
        //     _ => (),
        // }
        // for argument in entity_path.spatial_arguments.iter() {
        //     match argument {
        //         SpatialArgument::Const(_) => (),
        //         SpatialArgument::EntityRoute(route) => self.insert(*route),
        //     }
        // }
        // self.linkages.insert(entity_path.intrinsic())
    }

    fn produce_from_entity_defn(self, _entity_path: Term) -> Arc<VecSet<Term>> {
        todo!()
        // let defn = self.db.entity_defn(entity_path).unwrap();
        // self.collect_from_entity_defn(&defn);
        // Arc::new(self.linkages)
    }
}

pub(crate) fn entity_immediate_link_dependees(
    _db: &dyn RustTranspileDb,
    _entity_route: Term,
) -> Arc<VecSet<Term>> {
    todo!()
    // if entity_path.spatial_arguments.len() > 0 {
    //     let entity_defn = db.entity_defn(entity_path).unwrap();
    //     let spatial_parameters = entity_defn.spatial_parameters();
    //     let ctx = InstantiationContext {
    //         db: db.upcast(),
    //         spatial_parameters,
    //         spatial_arguments: &entity_path.spatial_arguments,
    //     };
    //     use husky_instantiate::Instantiable;
    //     let mut set: VecSet<_> = db
    //         .entity_immediate_link_dependees(db.base_route(entity_path))
    //         .iter()
    //         .map(|entity_path| {
    //             entity_path
    //                 .instantiate(&ctx)
    //                 .take_entity_route()
    //                 .intrinsic()
    //         })
    //         .collect();
    //     for spatial_argument in &entity_path.spatial_arguments {
    //         match spatial_argument {
    //             SpatialArgument::Const(_) => (),
    //             SpatialArgument::EntityRoute(route) => set.insert(route.intrinsic()),
    //         }
    //     }
    //     Arc::new(set)
    // } else {
    //     LinkageCollector {
    //         db,
    //         linkages: Default::default(),
    //     }
    //     .produce_from_entity_defn(entity_path)
    // }
}

pub(crate) fn entity_link_dependees(
    db: &dyn RustTranspileDb,
    entity_path: Term,
) -> Arc<VecSet<Term>> {
    let mut dependees = (*db.entity_immediate_link_dependees(entity_path)).clone();
    visit_all(db, &mut dependees, 0);
    return Arc::new(dependees);

    fn visit_all(_db: &dyn RustTranspileDb, _dependees: &mut VecSet<Term>, _start: usize) {
        todo!()
        // let len0 = dependees.len();
        // for subroute in dependees[start..]
        //     .iter()
        //     .copied()
        //     .collect::<Vec<_>>()
        // {
        //     match subroute.variant {
        //         EntityRouteVariant::Any { .. } => continue,
        //         _ => (),
        //     }
        //     let subroute_dependees = db.entity_immediate_link_dependees(subroute.intrinsic());
        //     dependees.extend(&subroute_dependees)
        // }
        // if dependees.len() > len0 {
        //     visit_all(db, dependees, len0)
        // }
    }
}
