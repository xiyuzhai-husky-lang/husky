use super::*;
use husky_devsoul_interface::HuskyJarIndex;
use husky_manifest::helpers::upstream::HasAllUpstreamPackages;
use husky_vfs::path::{linktime_target_path::LinktimeTargetPath, package_path::PackagePath};

pub fn package_path_from_jar_index(
    target_path: LinktimeTargetPath,
    jar_index: HuskyJarIndex,
    db: &::salsa::Db,
) -> PackagePath {
    target_path.all_upstream_packages(db).unwrap()[jar_index.index()]
}
