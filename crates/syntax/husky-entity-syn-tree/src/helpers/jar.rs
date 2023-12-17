use super::*;
use husky_manifest::HasAllPackages;
use husky_task_prelude::TaskJarIndex;
use husky_vfs::linktime_target_path::LinktimeTargetPath;

pub fn package_path_from_jar_index(
    target_path: LinktimeTargetPath,
    jar_index: TaskJarIndex,
    db: &::salsa::Db,
) -> PackagePath {
    target_path.all_packages(db).unwrap()[jar_index.index()]
}
