use husky_absolute_path::AbsolutePath;
use husky_entity_path::EntityPathData;
use husky_package_path::{PackagePath, PackagePathData};
use husky_source_path::{SourcePath, SourcePathData};
use salsa::Durability;

use crate::*;

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn absolute_path_from_source_path(
    db: &dyn VfsDb,
    path: SourcePath,
) -> VfsResult<AbsolutePath> {
    unsafe {
        Ok(match path.data(db.source_path_db()) {
            SourcePathData::Module(entity_path) => match db.dt_entity_path(entity_path) {
                EntityPathData::Crate { package, kind } => {
                    package_absolute_path(db, package)?.join(db.absolute_path_db(), kind.path())?
                }
                EntityPathData::Childpath { parent, ident } => {
                    let parent_source_absolute_path = absolute_path_from_source_path(
                        db,
                        db.it_source_path(SourcePathData::Module(parent)),
                    )?;
                    let dir = match db.dt_entity_path(parent) {
                        EntityPathData::Crate { package, kind } => parent_source_absolute_path
                            .path(db.absolute_path_db())
                            .parent()
                            .unwrap()
                            .to_owned(),
                        EntityPathData::Childpath { parent, ident } => parent_source_absolute_path
                            .path(db.absolute_path_db())
                            .with_extension(""),
                    };
                    AbsolutePath::new_from_owned(
                        db.absolute_path_db(),
                        dir.join(db.dt_ident(ident)).with_extension("hsy"),
                    )?
                }
            },
            SourcePathData::CorgiToml(package) => {
                package_absolute_path(db, package)?.join(db.absolute_path_db(), "Corgi.toml")?
            }
        })
    }
}

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn source_path_from_absolute_path(
    db: &dyn VfsDb,
    abs_path: AbsolutePath,
) -> VfsResult<Option<SourcePath>> {
    unsafe {
        let path = abs_path.path(db.absolute_path_db());
        let Some(extension) = path.extension().and_then(|s| s.to_str()) else {
            return Ok(None)
        };
        match extension {
            "hsy" => {
                db.file_content(abs_path, todo!());
                todo!()
            }
            "toml" => match path.file_name().and_then(|s| s.to_str()) {
                Some("Corgi.toml") => todo!(),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn package_absolute_path(
    db: &dyn VfsDb,
    package: PackagePath,
) -> VfsResult<AbsolutePath> {
    match db.package_path_data(package) {
        PackagePathData::Builtin { toolchain, .. } => {
            let name = db.package_name(package);
            // ad hoc;
            // shall include toolchain
            AbsolutePath::new_from_owned(
                db.absolute_path_db(),
                db.source_path_config().library_dir().join(name),
            )
            .map_err(|e| e.into())
        }
        PackagePathData::Global { version } => todo!(),
        PackagePathData::Local(path) => {
            AbsolutePath::new_from_owned(db.absolute_path_db(), path.to_owned())
                .map_err(|e| e.into())
        }
        PackagePathData::Git(_) => todo!(),
    }
}
