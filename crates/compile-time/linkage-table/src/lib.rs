use std::collections::HashMap;

use check_utils::*;
use sync_utils::ARwLock;
use vm::EntityUid;
use vm::{BoxedValue, EvalValue, Linkage, StackValue, VMResult};
use word::CustomIdentifier;

pub trait SearchLinkage {
    fn linkage_table(&self) -> &LinkageTable;
}

#[derive(Debug, Default, Clone)]
pub struct LinkageTable {
    linkages: ARwLock<HashMap<LinkageKey, Linkage>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkageKey {
    VecConstructor {
        element_ty_uid: EntityUid,
    },
    StructConstructor {
        ty_uid: EntityUid,
    },
    Routine {
        routine_uid: EntityUid,
    },
    ElemAccess {
        this_ty_uid: EntityUid,
    },
    StructFieldAccess {
        this_ty_uid: EntityUid,
        field_ident: CustomIdentifier,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct MembAccessKey {
    this_ty_uid: EntityUid,
    field_ident: CustomIdentifier,
}

impl LinkageTable {
    pub fn vec_constructor(&self, element_ty_uid: EntityUid) -> Linkage {
        let routine_key = LinkageKey::VecConstructor { element_ty_uid };
        if let Some(compiled_routine) = self.linkage(routine_key) {
            compiled_routine
        } else {
            Linkage {
                call: construct_virtual_vec,
                nargs: 0,
            }
        }
    }

    pub fn struct_constructor(&self, ty_uid: EntityUid) -> Option<Linkage> {
        self.linkage(LinkageKey::StructConstructor { ty_uid })
    }

    pub fn routine(&self, routine_uid: EntityUid) -> Option<Linkage> {
        self.linkage(LinkageKey::Routine { routine_uid })
    }

    pub fn struct_field_access(
        &self,
        this_ty_uid: EntityUid,
        field_ident: CustomIdentifier,
    ) -> Option<Linkage> {
        self.linkage(LinkageKey::StructFieldAccess {
            this_ty_uid,
            field_ident,
        })
    }

    fn linkage(&self, key: LinkageKey) -> Option<Linkage> {
        self.linkages
            .read(|entries| entries.get(&key).map(|compiled_routine| *compiled_routine))
    }

    fn elem_access_fp(&self, this_ty_uid: EntityUid) -> Option<Linkage> {
        self.linkage(LinkageKey::ElemAccess { this_ty_uid })
    }
}

fn construct_virtual_vec<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    should_eq!(values.len(), 0);
    Ok(StackValue::Boxed(BoxedValue::new(
        Vec::<EvalValue<'eval>>::new(),
    )))
}
