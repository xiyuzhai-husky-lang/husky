use husky_entity_syntax::EntitySyntaxQueryGroup;
use smallvec::SmallVec;
use static_defn::__StaticLinkageKey;
use upcast::Upcast;
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

    pub(crate) fn type_call_linkage(
        &self,
        db: &dyn EntitySyntaxQueryGroup,
        ty_uid: EntityUid,
    ) -> Option<__Linkage> {
        self.get_linkage(db, LinkageKey::TypeCall { ty_uid })
    }

    pub(crate) fn routine_linkage(
        &self,
        db: &dyn EntitySyntaxQueryGroup,
        routine_uid: EntityUid,
    ) -> Option<__Linkage> {
        self.get_linkage(db, LinkageKey::Routine { routine_uid })
    }

    pub(crate) fn struct_field_access_linkage_source(
        &self,
        db: &dyn EntitySyntaxQueryGroup,
        this_ty_uid: EntityUid,
        field_ident: CustomIdentifier,
        field_binding: Binding,
    ) -> Option<__Linkage> {
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
        db: &dyn EntitySyntaxQueryGroup,
        opd_uids: SmallVec<[EntityUid; 2]>,
    ) -> Option<__Linkage> {
        self.get_linkage(db, LinkageKey::ElementAccess { opd_uids })
    }

    fn get_linkage(&self, db: &dyn EntitySyntaxQueryGroup, key: LinkageKey) -> Option<__Linkage> {
        let result = self
            .linkages
            .read(|entries| entries.get(&key).map(|linkage_source| *linkage_source));
        if result.is_none() {
            let static_key = key.into_static(db);
            println!("linkage miss for {key:?}")
        }
        result
    }
}
