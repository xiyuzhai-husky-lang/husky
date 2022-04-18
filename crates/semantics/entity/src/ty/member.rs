mod field;
mod method;

pub use field::*;
use map_collect::MapCollect;
pub use method::*;

use super::*;

// impl HasKey<CustomIdentifier> for TypeMemberDefn {
//     fn key(&self) -> CustomIdentifier {
//         match self {
//             TypeMemberDefn::Field(field) => field.ident,
//             TypeMemberDefn::Method(method) => method.ident,
//         }
//     }
// }

pub fn collect_all_members(
    type_members: &[Arc<EntityDefn>],
    trait_impls: &[Arc<EntityDefn>],
) -> Avec<EntityDefn> {
    let mut members = type_members.to_vec();
    for trait_impl in trait_impls {
        todo!()
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
