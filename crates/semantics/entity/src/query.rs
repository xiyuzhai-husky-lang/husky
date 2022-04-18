use std::sync::{Arc, Mutex};

use crate::dependence::*;
use crate::*;
use entity_route::{EntityRoutePtr, EntitySource};
use infer_total::InferQueryGroup;
use semantics_error::*;
use sync_utils::ARwLock;
use upcast::Upcast;
use vm::EntityUid;

#[salsa::query_group(EntityQueryGroupStorage)]
pub trait EntityDefnQueryGroup:
    InferQueryGroup + ast::AstQueryGroup + Upcast<dyn InferQueryGroup> + StoreEntityRoute
{
    fn main_defn(&self, main_file: file::FilePtr) -> SemanticResultArc<MainDefn>;
    fn entity_defn(&self, route: EntityRoutePtr) -> SemanticResultArc<EntityDefn>;
    fn member_defn(&self, route: EntityRoutePtr) -> Arc<EntityDefn>;
    fn entity_immediate_dependees(&self, scope: EntityRoutePtr) -> SemanticResultArc<DependeeMap>;
    fn entity_dependees(&self, scope: EntityRoutePtr) -> SemanticResultArc<DependeeMap>;
    fn subentity_defns(&self, scope: EntityRoutePtr) -> SemanticResultArc<Vec<Arc<EntityDefn>>>;
    fn entity_defn_uid(&self, scope: EntityRoutePtr) -> EntityDefnUid;
    fn entity_uid(&self, scope: EntityRoutePtr) -> EntityUid;
}

pub trait StoreEntityRoute {
    fn entity_route_store(&self) -> &EntityRouteStore;

    fn entity_route_by_uid(&self, uid: EntityUid) -> EntityRoutePtr {
        self.entity_route_store().get(uid)
    }
}

#[derive(Debug, Default, Clone)]
pub struct EntityRouteStore {
    internal: ARwLock<Vec<EntityRoutePtr>>,
}

impl EntityRouteStore {
    fn add(&self, entity_route: EntityRoutePtr) -> EntityUid {
        self.internal.write(|internal: &mut Vec<EntityRoutePtr>| {
            let raw = internal.len();
            internal.push(entity_route);
            EntityUid { raw }
        })
    }

    fn get(&self, uid: EntityUid) -> EntityRoutePtr {
        self.internal.read(|internal| internal[uid.raw()])
    }
}

pub(crate) fn entity_uid(db: &dyn EntityDefnQueryGroup, entity_route: EntityRoutePtr) -> EntityUid {
    // responds to changes in either defn or defns of dependees
    let entity_source = db.entity_source(entity_route).unwrap();
    match entity_source {
        // in the future, we should make a difference between entity in current pack and depending packs
        EntitySource::StaticModuleItem(_) | EntitySource::StaticTypeMember => (),
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::Module { file } => todo!(),
        EntitySource::Input { main } => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let _defn = db.entity_defn(entity_route);
            let _dependees = db.entity_dependees(entity_route);
        }
    }
    db.entity_route_store().add(entity_route)
}
