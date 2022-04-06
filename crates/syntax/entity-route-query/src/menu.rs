use crate::*;
use entity_route::*;
use std::sync::Arc;
use word::RootIdentifier;

pub(crate) fn entity_route_menu(db: &dyn EntityRouteSalsaQueryGroup) -> Arc<EntityRouteMenu> {
    Arc::new(EntityRouteMenu {
        clone_trait: db.intern_scope(EntityRoute {
            kind: EntityRouteKind::Root {
                ident: RootIdentifier::CloneTrait,
            },
            generics: vec![],
        }),
        void_type: db.intern_scope(EntityRoute {
            kind: EntityRouteKind::Root {
                ident: RootIdentifier::Void,
            },
            generics: vec![],
        }),
        i32_type: db.intern_scope(EntityRoute {
            kind: EntityRouteKind::Root {
                ident: RootIdentifier::I32,
            },
            generics: vec![],
        }),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityRouteMenu {
    pub clone_trait: EntityRoutePtr,
    pub void_type: EntityRoutePtr,
    pub i32_type: EntityRoutePtr,
}
