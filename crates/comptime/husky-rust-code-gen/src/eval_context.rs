use crate::*;

pub(crate) fn needs_eval_context(_db: &dyn RustTranspileDb, _item_path: EtherealTerm) -> bool {
    todo!()
    // let item_link_dependees = db.item_link_dependees(item_path);
    // for link_route in item_link_dependees.iter() {
    //     let link_item_kind = db.husky_entity_taxonomy(*link_route).unwrap();
    //     match link_item_kind {
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
