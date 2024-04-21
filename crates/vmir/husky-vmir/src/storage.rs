use crate::region::VmirRegion;
use husky_linkage::{linkage::Linkage, version_stamp::LinkageVersionStamp};
use husky_task::linktime::IsLinktime;
use husky_task_interface::IsLinkageImpl;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct VmirStorage<LinkageImpl: IsLinkageImpl>(Arc<Mutex<VmirStorageImpl<LinkageImpl>>>);

pub struct VmirStorageImpl<LinkageImpl: IsLinkageImpl> {
    map: HashMap<Linkage, (LinkageVersionStamp, Arc<VmirRegion<LinkageImpl>>)>,
}

impl<LinkageImpl: IsLinkageImpl> VmirStorage<LinkageImpl> {
    pub fn linkage_vmir_region<Linktime: IsLinktime<LinkageImpl = LinkageImpl>>(
        &self,
        linkage: Linkage,
        db: &::salsa::Db,
        linktime: &Linktime,
    ) -> Option<Arc<VmirRegion<LinkageImpl>>> {
        self.0.lock().unwrap().linkage_vmir_region()
    }
}

impl<LinkageImpl: IsLinkageImpl> VmirStorageImpl<LinkageImpl> {
    fn linkage_vmir_region(&mut self) -> Option<Arc<VmirRegion<LinkageImpl>>> {
        todo!()
    }
}
