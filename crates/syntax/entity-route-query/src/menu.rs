use crate::*;
use entity_route::*;
use std::sync::Arc;
use word::RootIdentifier;

pub(crate) fn entity_route_menu(db: &dyn EntityRouteSalsaQueryGroup) -> Arc<EntityRouteMenu> {
    Arc::new(EntityRouteMenu {
        clone_trait: db.intern_entity_route(EntityRoute {
            kind: EntityRouteKind::Root {
                ident: RootIdentifier::CloneTrait,
            },
            generic_arguments: vec![],
        }),
        void_type: db.intern_entity_route(EntityRoute {
            kind: EntityRouteKind::Root {
                ident: RootIdentifier::Void,
            },
            generic_arguments: vec![],
        }),
        i32_type: db.intern_entity_route(EntityRoute {
            kind: EntityRouteKind::Root {
                ident: RootIdentifier::I32,
            },
            generic_arguments: vec![],
        }),
        this_type: db.intern_entity_route(EntityRoute {
            kind: EntityRouteKind::ThisType,
            generic_arguments: vec![],
        }),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityRouteMenu {
    pub clone_trait: EntityRoutePtr,
    pub void_type: EntityRoutePtr,
    pub i32_type: EntityRoutePtr,
    pub this_type: EntityRoutePtr,
}
