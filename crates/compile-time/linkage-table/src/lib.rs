mod table;
mod vec;

pub use table::*;

use check_utils::*;
use entity_route::{EntityRouteKind, EntityRoutePtr};
use map_collect::MapCollect;
use print_utils::p;
use semantics_entity::EntityDefnQueryGroup;
use std::collections::HashMap;
use sync_utils::ARwLock;
use vec::*;
use vm::EntityUid;
use vm::{BoxedValue, EvalValue, Linkage, StackValue, VMResult};
use word::{CustomIdentifier, RootIdentifier};

pub trait ResolveLinkage: EntityDefnQueryGroup {
    fn linkage_table(&self) -> &LinkageTable;

    fn element_access_linkage(
        &self,
        opd_tys: Vec<EntityRoutePtr>,
        access_kind: MemberAccessKind,
    ) -> Linkage {
        if let Some(linkage) = self
            .linkage_table()
            .element_access(opd_tys.map(|ty| self.entity_uid(*ty)), access_kind)
        {
            linkage
        } else {
            match opd_tys[0].kind {
                EntityRouteKind::Root {
                    ident: RootIdentifier::Vec,
                } => {
                    should_eq!(opd_tys.len(), 2);
                    Linkage {
                        call: match access_kind {
                            MemberAccessKind::Move => virtual_vec_element_move_access,
                            MemberAccessKind::Ref => virtual_vec_element_ref_access,
                            MemberAccessKind::BorrowMut => todo!(),
                        },
                        nargs: 2,
                    }
                }
                _ => todo!(),
            }
        }
    }
}
