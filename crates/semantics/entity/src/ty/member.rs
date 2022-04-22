mod field;
mod method;

pub use field::*;
pub use method::*;

use super::*;

pub fn collect_all_members(
    type_members: &[Arc<EntityDefn>],
    trait_impls: &[Arc<TraitImplDefn>],
) -> Avec<EntityDefn> {
    let mut members = type_members.to_vec();
    for trait_impl in trait_impls {
        members.extend(
            trait_impl
                .member_impls
                .iter()
                .map(|member_impl| member_impl.clone()),
        );
    }
    Arc::new(members)
}

pub fn member_defn(db: &dyn EntityDefnQueryGroup, member_route: EntityRoutePtr) -> Arc<EntityDefn> {
    let ty = member_route.parent();
    let ty_defn = db.entity_defn(ty).unwrap();
    let member_idx = db.member_idx(member_route);
    match ty_defn.variant {
        EntityDefnVariant::Type { ref members, .. } => members[member_idx.0 as usize].clone(),
        _ => panic!(),
    }
}
