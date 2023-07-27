use crate::*;
mod impl_expr;
mod impl_item;
mod impl_stmt;

pub(crate) struct LinkageCollector<'a> {
    db: &'a dyn RustTranspileDb,
    linkages: VecSet<EtherealTerm>,
}

impl<'a> LinkageCollector<'a> {
    pub(crate) fn insert(&mut self, _item_path: EtherealTerm) {
        todo!()
        // match item_path.variant {
        //     EntityRouteVariant::TraitForTypeMember { trai, .. } => {
        //         if trai == self.db.item_route_menu().clone_trait {
        //             return;
        //         }
        //     }
        //     EntityRouteVariant::TargetInputValue { .. } => return,
        //     EntityRouteVariant::Root {
        //         ident: RootBuiltinIdent::Vec,
        //     } => {
        //         // ad hoc
        //         if item_path.spatial_arguments.len() > 0 {
        //             self.insert(self.db.subroute(
        //                 item_path,
        //                 self.db.it_coword("ilen").custom(),
        //                 Default::default(),
        //             ))
        //         }
        //     }
        //     _ => (),
        // }
        // for argument in item_path.spatial_arguments.iter() {
        //     match argument {
        //         SpatialArgument::Const(_) => (),
        //         SpatialArgument::EntityRoute(route) => self.insert(*route),
        //     }
        // }
        // self.linkages.insert(item_path.intrinsic())
    }

    fn produce_from_item_defn(self, _item_path: EtherealTerm) -> Arc<VecSet<EtherealTerm>> {
        todo!()
        // let defn = self.db.item_defn(item_path).unwrap();
        // self.collect_from_item_defn(&defn);
        // Arc::new(self.linkages)
    }
}

pub(crate) fn item_immediate_link_dependees(
    _db: &dyn RustTranspileDb,
    _item_route: EtherealTerm,
) -> Arc<VecSet<EtherealTerm>> {
    todo!()
    // if item_path.spatial_arguments.len() > 0 {
    //     let item_defn = db.item_defn(item_path).unwrap();
    //     let spatial_parameters = item_defn.spatial_parameters();
    //     let ctx = InstantiationContext {
    //         db: db.upcast(),
    //         spatial_parameters,
    //         spatial_arguments: &item_path.spatial_arguments,
    //     };
    //     use husky_instantiate::Instantiable;
    //     let mut set: VecSet<_> = db
    //         .item_immediate_link_dependees(db.base_route(item_path))
    //         .iter()
    //         .map(|item_path| {
    //             item_path
    //                 .instantiate(&ctx)
    //                 .take_item_route()
    //                 .intrinsic()
    //         })
    //         .collect();
    //     for spatial_argument in &item_path.spatial_arguments {
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
    //     .produce_from_item_defn(item_path)
    // }
}

pub(crate) fn item_link_dependees(
    db: &dyn RustTranspileDb,
    item_path: EtherealTerm,
) -> Arc<VecSet<EtherealTerm>> {
    let mut dependees = (*db.item_immediate_link_dependees(item_path)).clone();
    visit_all(db, &mut dependees, 0);
    return Arc::new(dependees);

    fn visit_all(_db: &dyn RustTranspileDb, _dependees: &mut VecSet<EtherealTerm>, _start: usize) {
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
        //     let subroute_dependees = db.item_immediate_link_dependees(subroute.intrinsic());
        //     dependees.extend(&subroute_dependees)
        // }
        // if dependees.len() > len0 {
        //     visit_all(db, dependees, len0)
        // }
    }
}
