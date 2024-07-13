use crate::region::{
    linkage_virtual_vmir_region, linkage_vmir_region, VirtualVmirRegion, VmirRegion,
};
use husky_devsoul::linktime::IsLinktime;
use husky_devsoul_interface::IsLinkageImpl;
use husky_linkage::{
    linkage::{virtual_linkage_impl::VirtualLinkageImpl, Linkage},
    version_stamp::LinkageVersionStamp,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub trait IsVmirStorage<LinkageImpl: IsLinkageImpl> {
    type VmirRegionWrapper<'a>: std::ops::Deref<Target = VmirRegion<LinkageImpl>>;

    fn linkage_vmir_region<'db, Linktime: IsLinktime<LinkageImpl = LinkageImpl>>(
        &self,
        linkage: Linkage,
        db: &'db ::salsa::Db,
        linktime: &Linktime,
    ) -> Option<Self::VmirRegionWrapper<'db>>;
}

#[derive(Default)]
pub struct VirtualVmirStorage;

impl IsVmirStorage<VirtualLinkageImpl> for VirtualVmirStorage {
    type VmirRegionWrapper<'db> = &'db VirtualVmirRegion;

    fn linkage_vmir_region<'db, Linktime: IsLinktime<LinkageImpl = VirtualLinkageImpl>>(
        &self,
        linkage: Linkage,
        db: &'db salsa::Db,
        _linktime: &Linktime,
    ) -> Option<Self::VmirRegionWrapper<'db>> {
        linkage_virtual_vmir_region(db, linkage)
    }
}

/// # standard

pub struct StandardVmirStorage<LinkageImpl: IsLinkageImpl>(
    Arc<Mutex<StandardVmirStorageImpl<LinkageImpl>>>,
);

pub struct StandardVmirStorageImpl<LinkageImpl: IsLinkageImpl> {
    map: HashMap<Linkage, (LinkageVersionStamp, Arc<VmirRegion<LinkageImpl>>)>,
}

impl<LinkageImpl: IsLinkageImpl> IsVmirStorage<LinkageImpl> for StandardVmirStorage<LinkageImpl> {
    type VmirRegionWrapper<'a> = Arc<VmirRegion<LinkageImpl>>;

    fn linkage_vmir_region<'db, Linktime: IsLinktime<LinkageImpl = LinkageImpl>>(
        &self,
        linkage: Linkage,
        db: &'db ::salsa::Db,
        linktime: &Linktime,
    ) -> Option<Self::VmirRegionWrapper<'db>> {
        self.0
            .lock()
            .unwrap()
            .linkage_vmir_region(linkage, db, linktime)
    }
}

impl<LinkageImpl: IsLinkageImpl> StandardVmirStorageImpl<LinkageImpl> {
    fn linkage_vmir_region<Linktime: IsLinktime<LinkageImpl = LinkageImpl>>(
        &mut self,
        linkage: Linkage,
        db: &::salsa::Db,
        linktime: &Linktime,
    ) -> Option<Arc<VmirRegion<LinkageImpl>>> {
        use version_stamp::HasVersionStamp;

        // it's okay to just return None if linkage_vmir_region returns None
        // if linkage_vmir_region returns None, it means linkage is not defined in Husky itself, ffi or something
        // so the computation will be fast and no need to cache it
        match self.map.entry(linkage) {
            std::collections::hash_map::Entry::Occupied(entry) => {
                let (linkage_version_stamp, vim_region) = entry.into_mut();
                if *linkage_version_stamp != linkage.version_stamp(db) {
                    *vim_region = Arc::new(linkage_vmir_region(linkage, db, linktime)?)
                }
                Some(vim_region.clone())
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                let vim_region = Arc::new(linkage_vmir_region(linkage, db, linktime)?);
                entry.insert((linkage.version_stamp(db), vim_region.clone()));
                Some(vim_region)
            }
        }
    }
}
