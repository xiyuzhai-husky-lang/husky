use smallvec::SmallVec;
use static_defn::__StaticLinkageKey;
use vm::__Linkage;

use crate::*;

#[derive(Debug, Default, Clone)]
pub struct LinkageTable {
    linkages: ASafeRwLock<HashMap<LinkageKey, __Linkage>>,
}

impl LinkageTable {
    pub fn load(
        &self,
        db: &dyn ResolveLinkage,
        linkages: &'static [(__StaticLinkageKey, __Linkage)],
    ) {
        let new_linkages: HashMap<LinkageKey, __Linkage> = linkages
            .iter()
            .map(|(static_key, linkage)| {
                let key = LinkageKey::from_static(db, *static_key);
                (key, *linkage)
            })
            .collect();
        self.linkages.write(|linkages| {
            should_eq!(linkages.len(), 0);
            *linkages = new_linkages
        })
    }

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
        self.linkages
            .read(|entries| entries.get(&key).map(|linkage_source| *linkage_source))
    }
}
