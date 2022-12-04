use ::absolute_path::AbsolutePath;
use husky_entity_path::EntityPathData;
use husky_package_path::PackagePathData;

use crate::*;

#[salsa::tracked(jar = SourcePathJar, return_ref)]
pub(crate) fn source_absolute_path(
    db: &dyn SourcePathDb,
    path: SourcePath,
) -> SourcePathResult<AbsolutePath> {
    Ok(match path.data(db) {
        SourcePathData::Module(entity_path) => match db.dt_entity_path(entity_path) {
            EntityPathData::Crate { package, kind } => package_absolute_path(db, package)
                .as_ref()?
                .join(kind.path())?,
            EntityPathData::Childpath { parent, ident } => todo!(),
        },
        SourcePathData::CorgiToml(package) => package_absolute_path(db, package)
            .as_ref()?
            .join("Corgi.toml")?,
    })
}

#[salsa::tracked(jar = SourcePathJar, return_ref)]
pub(crate) fn package_absolute_path(
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
