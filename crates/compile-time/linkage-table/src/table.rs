use smallvec::SmallVec;

use crate::*;

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
    ElementAccess {
        opd_uids: SmallVec<[EntityUid; 2]>,
        access_kind: MemberAccessKind,
    },
    StructFieldAccess {
        this_ty_uid: EntityUid,
        field_ident: CustomIdentifier,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MemberAccessKind {
    Move,
    Ref,
    BorrowMut,
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

    pub(crate) fn element_access(
        &self,
        opd_uids: SmallVec<[EntityUid; 2]>,
        access_kind: MemberAccessKind,
    ) -> Option<Linkage> {
        self.linkage(LinkageKey::ElementAccess {
            opd_uids,
            access_kind,
        })
    }

    fn linkage(&self, key: LinkageKey) -> Option<Linkage> {
        self.linkages
            .read(|entries| entries.get(&key).map(|compiled_routine| *compiled_routine))
    }
}
