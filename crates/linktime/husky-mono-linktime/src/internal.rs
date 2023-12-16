mod libraries;
mod linkage_impls;

use self::linkage_impls::generate_linkage_impls;
use self::{libraries::MonoLinkageLibraries, linkage_impls::LinkageImplMap};
use crate::*;
use husky_linkage::version_stamp::LinkageVersionStamp;
use husky_vfs::linktime_target_path::LinktimeTargetPath;
use version_stamp::HasVersionStamp;

pub struct MonoLinkTimeInternal<LinkageImpl>
where
    LinkageImpl: IsLinkageImpl,
{
    target_path: LinktimeTargetPath,
    linkage_storage: MonoLinkageLibraries,
    linkage_impls: LinkageImplMap<LinkageImpl>,
}

impl<LinkageImpl: IsLinkageImpl> MonoLinkTimeInternal<LinkageImpl>
where
    LinkageImpl: IsLinkageImpl,
{
    pub(crate) fn new(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self {
        let Ok(linkage_storage) = MonoLinkageLibraries::generate(target_path, db) else {
            todo!()
        };
        let linkage_impls = generate_linkage_impls(target_path, &linkage_storage, db);
        Self {
            target_path,
            linkage_storage,
            linkage_impls,
        }
    }

    pub(crate) fn get_linkage_impl(
        &self,
        linkage: Linkage,
        db: &::salsa::Db,
    ) -> Option<LinkageImpl> {
        let Some(&(deps, linkage_impl)) = self.linkage_impls.get(&linkage) else {
            use husky_print_utils::p;
            use salsa::DebugWithDb;
            let linkages: Vec<Linkage> = self.linkage_impls.clone().into_keys().collect();
            p!(linkages.debug_with(db));
            p!(linkage.debug(db));
            unreachable!()
        };
        (deps == linkage.version_stamp(db)).then_some(linkage_impl)
    }

    /// still need the key to avoid redundant reload when two attempts simultaneously want to lock
    pub(crate) fn get_linkage_impl_with_reload(
        &mut self,
        key: Linkage,
        db: &::salsa::Db,
    ) -> LinkageImpl {
        let (deps, linkage) = self
            .linkage_impls
            .get(&key)
            .copied()
            .expect("should be some");
        if deps == key.version_stamp(db) {
            return linkage;
        }
        self.reload(db);
        self.linkage_impls
            .get(&key)
            .copied()
            .expect("should be some")
            .1
    }

    fn reload(&mut self, db: &::salsa::Db) {
        self.linkage_storage = MonoLinkageLibraries::generate(self.target_path, db).unwrap();
        self.linkage_impls = generate_linkage_impls(self.target_path, &self.linkage_storage, db)
    }
}
