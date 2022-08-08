use crate::dependence::*;
use crate::*;
use husky_entity_route::EntityRoutePtr;
use infer_total::InferQueryGroup;
use semantics_error::*;
use std::sync::{Arc, Mutex};
use sync_utils::ASafeRwLock;
use upcast::Upcast;
use vm::EntityUid;

#[salsa::query_group(EntityQueryGroupStorage)]
pub trait EntityDefnQueryGroup:
    InferQueryGroup + husky_ast::AstQueryGroup + Upcast<dyn InferQueryGroup> + StoreEntityRoute
{
    fn main_defn(&self, target_entrance: husky_file::FilePtr) -> SemanticResultArc<MainDefn>;
    fn entity_defn(&self, route: EntityRoutePtr) -> SemanticResultArc<EntityDefn>;
    fn member_defn(&self, route: EntityRoutePtr) -> Arc<EntityDefn>;
    fn entity_immediate_dependees(
        &self,
        entity_route: EntityRoutePtr,
    ) -> SemanticResultArc<DependeeMap>;
    fn entity_dependees(&self, entity_route: EntityRoutePtr) -> SemanticResultArc<DependeeMap>;
    fn subentity_defns(
        &self,
        entity_route: EntityRoutePtr,
    ) -> SemanticResultArc<Vec<Arc<EntityDefn>>>;
    fn entity_defn_uid(&self, entity_route: EntityRoutePtr) -> EntityDefnUid;
    fn entity_uid(&self, entity_route: EntityRoutePtr) -> EntityUid;
    fn visualizer(&self, ty: EntityRoutePtr) -> Arc<Visualizer>;
    fn visual_ty(&self, ty: EntityRoutePtr) -> VisualTy;
}

pub trait StoreEntityRoute {
    fn entity_route_store(&self) -> &EntityRouteStore;

    fn entity_route_by_uid(&self, uid: EntityUid) -> EntityRoutePtr {
        self.entity_route_store().get(uid)
    }
}

#[derive(Debug, Default, Clone)]
pub struct EntityRouteStore {
    internal: ASafeRwLock<Vec<EntityRoutePtr>>,
}

impl EntityRouteStore {
    fn add(&self, entity_route: EntityRoutePtr) -> EntityUid {
        self.internal.write(|internal: &mut Vec<EntityRoutePtr>| {
            let raw = internal.len();
            internal.push(entity_route);
            unsafe { EntityUid::from_raw(raw as u64) }
        })
    }

    fn get(&self, uid: EntityUid) -> EntityRoutePtr {
        self.internal.read(|internal| internal[uid.raw() as usize])
    }
}

pub(crate) fn entity_uid(db: &dyn EntityDefnQueryGroup, entity_route: EntityRoutePtr) -> EntityUid {
    // responds to changes in either defn or defns of dependees
    let entity_source = db.entity_source(entity_route).unwrap();
    match entity_source {
        // in the future, we should make a difference between entity in current pack and depending packs
        EntitySource::StaticModuleItem(_)
        | EntitySource::StaticTypeMember(_)
        | EntitySource::StaticTraitMember(_)
        | EntitySource::StaticTypeAsTraitMember
        | EntitySource::StaticEnumVariant(_) => (),
        EntitySource::TargetInput { .. } => (), // ad hoc, should consider the task config block
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::Module { file } => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let _defn = db.entity_defn(entity_route);
            let _dependees = db.entity_dependees(entity_route);
        }
        EntitySource::Any { .. } => todo!(),
    }
    db.entity_route_store().add(entity_route)
}
