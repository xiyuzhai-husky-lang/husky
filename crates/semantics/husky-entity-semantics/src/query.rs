use crate::dependence::*;
use crate::*;
use husky_entity_route::EntityRouteItd;
use husky_semantics_error::*;
use husky_vm::EntityUid;
use std::sync::Arc;
use sync_utils::ASafeRwLock;

use utils::module_contains_features;

#[salsa::query_group(EntityQueryGroupStorage)]
pub trait EntityDefnQueryGroup: husky_ast::AstQueryGroup + StoreEntityRoute {
    fn main_defn(&self, target_entrance: husky_file::FileItd) -> SemanticResultArc<MainDefn>;
    fn entity_defn(&self, route: EntityRouteItd) -> SemanticResultArc<EntityDefn>;
    fn member_defn(&self, route: EntityRouteItd) -> Arc<EntityDefn>;
    fn entity_immediate_dependees(
        &self,
        entity_route: EntityRouteItd,
    ) -> SemanticResultArc<DependeeMap>;
    fn entity_dependees(&self, entity_route: EntityRouteItd) -> SemanticResultArc<DependeeMap>;
    fn subentity_defns(
        &self,
        entity_route: EntityRouteItd,
    ) -> SemanticResultArc<Vec<Arc<EntityDefn>>>;
    fn entity_defn_uid(&self, entity_route: EntityRouteItd) -> EntityDefnUid;
    fn entity_uid(&self, entity_route: EntityRouteItd) -> EntityUid;
    fn visualizer(&self, ty: EntityRouteItd) -> Arc<Visualizer>;
    fn visual_ty(&self, ty: EntityRouteItd) -> VisualTy;
    fn module_contains_features(&self, module_route: EntityRouteItd) -> bool;
}

pub trait StoreEntityRoute {
    fn entity_route_store(&self) -> &EntityRouteStore;

    fn entity_route_by_uid(&self, uid: EntityUid) -> EntityRouteItd {
        self.entity_route_store().get(uid)
    }
}

#[derive(Debug, Default, Clone)]
pub struct EntityRouteStore {
    internal: ASafeRwLock<Vec<EntityRouteItd>>,
}

impl EntityRouteStore {
    fn add(&self, entity_route: EntityRouteItd) -> EntityUid {
        self.internal.write(|internal: &mut Vec<EntityRouteItd>| {
            let raw = internal.len();
            internal.push(entity_route);
            unsafe { EntityUid::from_raw(raw as u64) }
        })
    }

    fn get(&self, uid: EntityUid) -> EntityRouteItd {
        self.internal.read(|internal| internal[uid.raw() as usize])
    }
}

pub(crate) fn entity_uid(db: &dyn EntityDefnQueryGroup, entity_route: EntityRouteItd) -> EntityUid {
    // responds to changes in either defn or defns of dependees
    if !entity_route.is_intrinsic() {
        panic!("expect intrinsic, but get `{}` instead", entity_route)
    }
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
        EntitySource::Module { .. } => todo!(),
        EntitySource::WithinModule { .. } => {
            let _defn = db.entity_defn(entity_route);
            let _dependees = db.entity_dependees(entity_route);
        }
        EntitySource::Any { .. } => todo!(),
        EntitySource::ThisType { .. } => todo!(),
    }
    db.entity_route_store().add(entity_route)
}
