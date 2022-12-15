use husky_absolute_path::AbsolutePath;
use husky_entity_path::{EntityPath, EntityPathData};
use husky_package_path::{PackagePath, PackagePathData};
use salsa::Durability;

use crate::*;

pub(crate) fn package_manifest_path(
    db: &dyn VfsDb,
    package: PackagePath,
) -> VfsResult<AbsolutePath> {
    AbsolutePath::new(&package_dir(db, package).as_ref()?.join("Corgi.toml")).map_err(|e| todo!())
}

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn package_dir(db: &dyn VfsDb, package: PackagePath) -> VfsResult<AbsolutePath> {
    match db.package_path_data(package) {
        PackagePathData::Builtin { toolchain, .. } => {
            let name = db.package_name(package);
            // ad hoc;
            // shall include toolchain
            AbsolutePath::new(&db.vfs_config().library_dir().join(name)).map_err(|e| e.into())
        }
        PackagePathData::Global { version } => todo!(),
        PackagePathData::Local(path) => Ok(path.clone()),
        PackagePathData::Git(_) => todo!(),
    }
}

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn module_path(db: &dyn VfsDb, entity_path: EntityPath) -> VfsResult<AbsolutePath> {
    match db.dt_entity_path(entity_path) {
        EntityPathData::Crate { package, kind } => {
            AbsolutePath::new(&package_dir(db, package).as_ref()?.join(kind.path()))
                .map_err(|e| e.into())
        }
        EntityPathData::Childpath { parent, ident } => {
            let parent_module_path = module_path(db, parent).as_ref()?;
            let dir = match db.dt_entity_path(parent) {
                EntityPathData::Crate { package, kind } => {
                    parent_module_path.parent().unwrap().to_owned()
                }
                EntityPathData::Childpath { parent, ident } => {
                    parent_module_path.with_extension("")
                }
            };
            AbsolutePath::new(&dir.join(db.dt_ident(ident)).with_extension("hsy"))
                .map_err(|e| e.into())
        }
    }
}
