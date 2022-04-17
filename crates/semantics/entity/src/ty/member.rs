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
