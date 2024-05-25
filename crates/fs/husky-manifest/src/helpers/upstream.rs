use crate::{HasManifest, ManifestResult, ManifestResultRef};
use husky_vfs::{
    linktime_target_path::{LinktimeTargetPath, LinktimeTargetPathData},
    PackagePath,
};
use vec_like::VecSet;

pub trait HasAllUpstreamPackages: Copy {
    fn all_upstream_packages(self, db: &::salsa::Db) -> ManifestResultRef<&[PackagePath]>;
}

impl HasAllUpstreamPackages for LinktimeTargetPath {
    fn all_upstream_packages(self, db: &salsa::Db) -> ManifestResultRef<&[PackagePath]> {
        linktime_target_path_all_upstream_packages(db, self)
            .as_ref()
            .map(|v| v as &[_])
    }
}

#[salsa::tracked(return_ref)]
fn linktime_target_path_all_upstream_packages(
    db: &::salsa::Db,
    target_path: LinktimeTargetPath,
) -> ManifestResult<VecSet<PackagePath>> {
    Ok(match target_path.data(db) {
        LinktimeTargetPathData::Package(package_path) => {
            VecSet::from_iter(package_path.full_dependencies(db)?.iter().copied())
        }
        LinktimeTargetPathData::Workspace(_) => todo!(),
    })
}
