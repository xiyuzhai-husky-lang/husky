use entity_route::{EntityRouteKind, GenericArgument};
use word::RootIdentifier;

use crate::*;

#[test]
fn test_list_ty_decl() {
    let mut db = HuskyLangCompileTime::default();
    let list_i32_route = db.intern_entity_route(EntityRoute {
        kind: EntityRouteKind::Root {
            ident: RootIdentifier::List,
        },
        generic_arguments: vec![GenericArgument::EntityRoute(db.entity_route_menu().i32_ty)],
    });
    let list_ty_decl = db.ty_decl(db.entity_route_menu().list_ty).unwrap();
}
