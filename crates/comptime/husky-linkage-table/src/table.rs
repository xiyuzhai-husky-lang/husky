use crate::*;
use __husky::init::__StaticLinkageKey;
use husky_vm::__Linkage;
use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub struct LinkageTable {
    linkages: ASafeRwLock<HashMap<LinkageKey, __Linkage>>,
    pub(crate) config: LinkageTableConfig,
}

impl LinkageTable {
    pub fn new(config: LinkageTableConfig) -> Self {
        Self {
            linkages: Default::default(),
            config,
        }
    }

    pub fn load(
        &self,
        db: &dyn ResolveLinkage,
        static_linkages: &[(__StaticLinkageKey, __Linkage)],
    ) {
        let new_linkages: HashMap<LinkageKey, __Linkage> = static_linkages
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

    pub(crate) fn feature_eager_block_linkage(&self, feature_uid: EntityUid) -> Option<__Linkage> {
        self.get_linkage(LinkageKey::FeatureEagerBlock { uid: feature_uid })
    }

    pub(crate) fn routine_linkage(&self, routine_uid: EntityUid) -> Option<__Linkage> {
        self.get_linkage(LinkageKey::Routine { routine_uid })
    }

    pub(crate) fn field_linkage_source(
        &self,
        this_ty_uid: EntityUid,
        field_ident: CustomIdentifier,
    ) -> Option<__Linkage> {
        self.get_linkage(LinkageKey::StructField {
            this_ty_uid,
            field_ident,
        })
    }

    pub(crate) fn element_access(&self, opd_uids: SmallVec<[EntityUid; 2]>) -> Option<__Linkage> {
        self.get_linkage(LinkageKey::Index { opd_uids })
    }

    fn get_linkage(&self, key: LinkageKey) -> Option<__Linkage> {
        self.linkages
            .read(|entries| entries.get(&key).map(|linkage_source| *linkage_source))
    }
}
