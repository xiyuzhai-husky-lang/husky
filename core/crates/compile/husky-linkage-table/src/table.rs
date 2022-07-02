use smallvec::SmallVec;
use static_defn::__StaticLinkageKey;
use vm::__Linkage;

use crate::*;

#[derive(Debug, Default, Clone)]
pub struct LinkageSourceTable {
    linkage_sources: ARwLock<HashMap<LinkageKey, __Linkage>>,
}

impl LinkageSourceTable {
    pub(crate) fn load(linkages: impl Iterator<Item = (__StaticLinkageKey, __Linkage)>) {}

    pub(crate) fn type_call_linkage(&self, ty_uid: EntityUid) -> Option<__Linkage> {
        self.get_linkage(LinkageKey::TypeCall { ty_uid })
    }

    pub(crate) fn routine_linkage(&self, routine_uid: EntityUid) -> Option<__Linkage> {
        self.get_linkage(LinkageKey::Routine { routine_uid })
    }

    pub(crate) fn struct_field_access_linkage_source(
        &self,
        this_ty_uid: EntityUid,
        field_ident: CustomIdentifier,
        field_binding: Binding,
    ) -> Option<__Linkage> {
        self.get_linkage(LinkageKey::StructFieldAccess {
            this_ty_uid,
            field_ident,
        })
    }

    pub(crate) fn element_access(&self, opd_uids: SmallVec<[EntityUid; 2]>) -> Option<__Linkage> {
        self.get_linkage(LinkageKey::ElementAccess { opd_uids })
    }

    fn get_linkage(&self, key: LinkageKey) -> Option<__Linkage> {
        self.linkage_sources
            .read(|entries| entries.get(&key).map(|linkage_source| *linkage_source))
    }
}
