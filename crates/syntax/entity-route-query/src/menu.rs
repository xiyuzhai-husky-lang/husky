use crate::*;
use entity_route::*;
use std::sync::Arc;
use word::RootIdentifier;

pub(crate) fn entity_route_menu(db: &dyn EntityRouteSalsaQueryGroup) -> Arc<EntityRouteMenu> {
    let std_mod = EntityRoutePtr::Root(RootIdentifier::Std);
    let std_ops_mod = db.intern_entity_route(EntityRoute {
        kind: EntityRouteKind::Child {
            parent: std_mod,
            ident: db.intern_word("ops").custom(),
        },
        generic_arguments: vec![],
    });
    let std_ops_index_trai = db.intern_entity_route(EntityRoute {
        kind: EntityRouteKind::Child {
            parent: std_ops_mod,
            ident: db.intern_word("Index").custom(),
        },
        generic_arguments: vec![],
    });
    Arc::new(EntityRouteMenu {
        clone_trait: EntityRoutePtr::Root(RootIdentifier::CloneTrait),
        copy_trait: EntityRoutePtr::Root(RootIdentifier::CopyTrait),
        void_type: EntityRoutePtr::Root(RootIdentifier::Void),
        i32_ty: EntityRoutePtr::Root(RootIdentifier::I32),
        this_ty: EntityRoutePtr::ThisType,
        list_ty: EntityRoutePtr::Root(RootIdentifier::List),
        std_mod,
        std_ops_mod,
        std_ops_index_trai,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityRouteMenu {
    pub clone_trait: EntityRoutePtr,
    pub copy_trait: EntityRoutePtr,
    pub void_type: EntityRoutePtr,
    pub i32_ty: EntityRoutePtr,
    pub this_ty: EntityRoutePtr,
    pub list_ty: EntityRoutePtr,
    pub std_ops_index_trai: EntityRoutePtr,
    pub std_mod: EntityRoutePtr,
    pub std_ops_mod: EntityRoutePtr,
}
