#[salsa::jar]
pub struct VmirJar(
    crate::region::VmirRegion,
    crate::region::linkage_vmir_region,
);
