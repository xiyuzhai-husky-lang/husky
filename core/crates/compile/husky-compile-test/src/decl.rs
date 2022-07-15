use husky_entity_route::entity_route_menu;
use husky_entity_route::SpatialArgument;
use thin_vec::{thin_vec, ThinVec};
use word::RootIdentifier;

use crate::*;

#[test]
fn test_vec_ty_decl() {
    let db = HuskyCompileTime::new_default(__root_defn);
    let _vec_i32_route = db.make_route(
        EntityRoutePtr::Root(RootIdentifier::Vec),
        thin_vec![SpatialArgument::EntityRoute(entity_route_menu().i32_ty)],
    );
    let _vec_ty_decl = db.ty_decl(entity_route_menu().vec_ty).unwrap();
}
