#[salsa::db(husky_coword::jar::CowordJar, visored_zfs_ty::jar::VdZfsTypeJar)]
pub(crate) struct DB {}
