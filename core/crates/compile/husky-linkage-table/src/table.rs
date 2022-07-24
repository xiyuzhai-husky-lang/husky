use __husky::init::__StaticLinkageKey;
use smallvec::SmallVec;
use upcast::Upcast;
use vm::LinkageDeprecated;

use crate::*;

#[derive(Debug, Clone)]
pub struct LinkageTable {
    linkages: ASafeRwLock<HashMap<LinkageKey, LinkageDeprecated>>,
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
        static_linkages: &[(__StaticLinkageKey, LinkageDeprecated)],
    ) {
        let new_linkages: HashMap<LinkageKey, LinkageDeprecated> = static_linkages
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

    pub(crate) fn type_call_linkage(
        &self,
        db: &dyn EntityDefnQueryGroup,
        ty_uid: EntityUid,
    ) -> Option<LinkageDeprecated> {
        self.get_linkage(db, LinkageKey::TypeCall { ty_uid })
    }

    pub(crate) fn feature_eager_block_linkage(
        &self,
        db: &dyn EntityDefnQueryGroup,
        feature_uid: EntityUid,
    ) -> Option<LinkageDeprecated> {
        self.get_linkage(db, LinkageKey::FeatureEagerBlock { uid: feature_uid })
    }

    pub(crate) fn routine_linkage(
        &self,
        db: &dyn EntityDefnQueryGroup,
        routine_uid: EntityUid,
    ) -> Option<LinkageDeprecated> {
        self.get_linkage(db, LinkageKey::Routine { routine_uid })
    }

    pub(crate) fn struct_field_access_linkage_source(
        &self,
        db: &dyn EntityDefnQueryGroup,
        this_ty_uid: EntityUid,
        field_ident: CustomIdentifier,
        field_binding: Binding,
    ) -> Option<LinkageDeprecated> {
        self.get_linkage(
            db,
            LinkageKey::StructFieldAccess {
                this_ty_uid,
                field_ident,
            },
        )
    }

    pub(crate) fn element_access(
        &self,
        db: &dyn EntityDefnQueryGroup,
        opd_uids: SmallVec<[EntityUid; 2]>,
    ) -> Option<LinkageDeprecated> {
        self.get_linkage(db, LinkageKey::ElementAccess { opd_uids })
    }

    fn get_linkage(
        &self,
        db: &dyn EntityDefnQueryGroup,
        key: LinkageKey,
    ) -> Option<LinkageDeprecated> {
        self.linkages
            .read(|entries| entries.get(&key).map(|linkage_source| *linkage_source))
    }
}
