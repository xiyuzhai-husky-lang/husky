use crate::region::{
    linket_virtual_vmir_region, linket_vmir_region, VirtualVmirRegion, VmirRegion,
};
use husky_linket::{linket::Linket, version_stamp::LinketVersionStamp};
use husky_linket_impl::linket_impl::IsLinketImpl;
use husky_linktime::IsLinktime;
use husky_virtual_linket_impl::VirtualLinketImpl;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub trait IsVmirStorage<LinketImpl: IsLinketImpl> {
    type VmirRegionWrapper<'a>: std::ops::Deref<Target = VmirRegion<LinketImpl>>;

    fn linket_vmir_region<'db, Linktime: IsLinktime<LinketImpl = LinketImpl>>(
        &self,
        linket: Linket,
        db: &'db ::salsa::Db,
        linktime: &Linktime,
    ) -> Option<Self::VmirRegionWrapper<'db>>;
}

#[derive(Default)]
pub struct VirtualVmirStorage;

impl IsVmirStorage<VirtualLinketImpl> for VirtualVmirStorage {
    type VmirRegionWrapper<'db> = &'db VirtualVmirRegion;

    fn linket_vmir_region<'db, Linktime: IsLinktime<LinketImpl = VirtualLinketImpl>>(
        &self,
        linket: Linket,
        db: &'db salsa::Db,
        _linktime: &Linktime,
    ) -> Option<Self::VmirRegionWrapper<'db>> {
        linket_virtual_vmir_region(db, linket)
    }
}

/// # standard

pub struct DevVmirStorage<LinketImpl: IsLinketImpl>(
    Arc<Mutex<StandardVmirStorageImpl<LinketImpl>>>,
);

pub struct StandardVmirStorageImpl<LinketImpl: IsLinketImpl> {
    map: HashMap<Linket, (LinketVersionStamp, Arc<VmirRegion<LinketImpl>>)>,
}

impl<LinketImpl: IsLinketImpl> Default for DevVmirStorage<LinketImpl> {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(StandardVmirStorageImpl {
            map: HashMap::new(),
        })))
    }
}

impl<LinketImpl: IsLinketImpl> IsVmirStorage<LinketImpl> for DevVmirStorage<LinketImpl> {
    type VmirRegionWrapper<'a> = Arc<VmirRegion<LinketImpl>>;

    fn linket_vmir_region<'db, Linktime: IsLinktime<LinketImpl = LinketImpl>>(
        &self,
        linket: Linket,
        db: &'db ::salsa::Db,
        linktime: &Linktime,
    ) -> Option<Self::VmirRegionWrapper<'db>> {
        self.0
            .lock()
            .unwrap()
            .linket_vmir_region(linket, db, linktime)
    }
}

impl<LinketImpl: IsLinketImpl> StandardVmirStorageImpl<LinketImpl> {
    fn linket_vmir_region<Linktime: IsLinktime<LinketImpl = LinketImpl>>(
        &mut self,
        linket: Linket,
        db: &::salsa::Db,
        linktime: &Linktime,
    ) -> Option<Arc<VmirRegion<LinketImpl>>> {
        use version_stamp::HasVersionStamp;

        // it's okay to just return None if linket_vmir_region returns None
        // if linket_vmir_region returns None, it means linket is not defined in Husky itself, ffi or something
        // so the computation will be fast and no need to cache it
        match self.map.entry(linket) {
            std::collections::hash_map::Entry::Occupied(entry) => {
                let (linket_version_stamp, vim_region) = entry.into_mut();
                if *linket_version_stamp != linket.version_stamp(db) {
                    *vim_region = Arc::new(linket_vmir_region(linket, db, linktime)?)
                }
                Some(vim_region.clone())
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                let vim_region = Arc::new(linket_vmir_region(linket, db, linktime)?);
                entry.insert((linket.version_stamp(db), vim_region.clone()));
                Some(vim_region)
            }
        }
    }
}
