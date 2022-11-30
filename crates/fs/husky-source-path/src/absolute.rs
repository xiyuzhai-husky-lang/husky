use ::absolute_path::AbsolutePath;
use husky_package_path::PackagePathData;

use crate::*;

#[salsa::tracked(jar = SourcePathJar, return_ref)]
pub(crate) fn source_to_absolute_path(
    db: &dyn SourcePathDb,
    path: SourcePath,
) -> SourcePathResult<AbsolutePath> {
    match path.data(db) {
        SourcePathData::Module(_) => todo!(),
        SourcePathData::CorgiToml(package) => resolve_package_path(db, package)?
            .join("Corgi.toml")
            .map_err(|e| e.into()),
    }
}

fn resolve_package_path(
    db: &dyn SourcePathDb,
    package: PackagePath,
) -> SourcePathResult<AbsolutePath> {
    match db.package_path_data(package) {
        PackagePathData::Builtin { toolchain, .. } => {
            let name = db.package_name(package);
            // ad hoc;
            // shall include toolchain
            AbsolutePath::new(db.source_path_config().library_dir().join(name))
                .map_err(|e| e.into())
        }
        PackagePathData::Global { version } => todo!(),
        PackagePathData::Local(path) => AbsolutePath::new(path.to_owned()).map_err(|e| e.into()),
        PackagePathData::Git(_) => todo!(),
    }
}
