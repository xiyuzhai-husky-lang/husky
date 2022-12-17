use husky_entity_taxonomy::{EntityClass, MemberKind, TyKingdom};

use crate::*;

pub(crate) fn needs_eval_context(db: &dyn RustTranspileDb, entity_path: Term) -> bool {
    todo!()
    // let entity_link_dependees = db.entity_link_dependees(entity_path);
    // for link_route in entity_link_dependees.iter() {
    //     let link_entity_taxonomy = db.husky_entity_taxonomy(*link_route).unwrap();
    //     match link_entity_taxonomy {
    //         EntityKind::Feature => return true,
    //         EntityKind::Main => panic!(),
    //         EntityKind::Member(member_kind) => match member_kind {
    //             MemberKind::Field => todo!(),
    //             MemberKind::Method { .. } => (),
    //             MemberKind::Call => todo!(),
    //             MemberKind::TraitAssociatedType => todo!(),
    //             MemberKind::TraitAssociatedConstSize => todo!(),
    //             MemberKind::TraitAssociatedAny => todo!(),
    //         },
    //         EntityKind::Type(TyKind::ThickFp) => return true,
    //         _ => (),
    //     }
    // }
    // false
}
