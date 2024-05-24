use super::HasSynopsis;
use crate::{ManifestResult, ManifestResultRef};
use husky_vfs::PackagePath;

#[derive(Debug, PartialEq, Eq)]
pub enum PackageSynopsis {}

impl HasSynopsis for PackagePath {
    type Synopsis = PackageSynopsis;

    fn synopsis<'a>(self, db: &'a salsa::Db) -> ManifestResultRef<'a, &'a Self::Synopsis> {
        package_synopsis(db, self).as_ref()
    }
}

#[salsa::tracked(return_ref)]
fn package_synopsis(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ManifestResult<PackageSynopsis> {
    todo!()
}

#[test]
fn package_synopsis_works() {
    // todo!()
}
