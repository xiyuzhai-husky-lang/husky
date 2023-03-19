#![feature(trait_upcasting)]
mod builder;
mod db;
mod error;
mod sections;

pub use self::db::*;
pub use self::error::*;
pub use self::sections::*;

use self::builder::*;
use husky_corgi_config_ast::*;
use husky_vfs::*;

#[salsa::jar(db = CorgiConfigDb)]
pub struct CorgiConfigJar(
    package_corgi_config,
    package_corgi_config_paths_aux,
    package_registry_path,
    root_corgi_config_path,
);

#[derive(Debug, PartialEq, Eq)]
pub struct CorgiConfig {
    registry_section: RegistrySection,
    errors: Vec<CorgiConfigError>,
}

#[salsa::tracked(jar = CorgiConfigJar)]
pub(crate) fn package_registry_path(
    db: &dyn CorgiConfigDb,
    package: PackagePath,
) -> VfsResult<RegistryPath> {
    let corgi_config = package.corgi_config(db);
    todo!()
}

pub trait HasCorgiConfig: Copy {
    fn corgi_config(self, db: &dyn CorgiConfigDb) -> VfsResult<&CorgiConfig>;

    fn registry_path(self, db: &dyn CorgiConfigDb) -> VfsResult<RegistryPath>;
}

impl HasCorgiConfig for PackagePath {
    fn corgi_config(self, db: &dyn CorgiConfigDb) -> VfsResult<&CorgiConfig> {
        package_corgi_config(db, self)
            .as_ref()
            .map_err(|e| e.clone())
    }

    fn registry_path(self, db: &dyn CorgiConfigDb) -> VfsResult<RegistryPath> {
        package_registry_path(db, self)
    }
}

#[salsa::tracked(jar = CorgiConfigJar, return_ref)]
fn package_corgi_config(
    db: &dyn CorgiConfigDb,
    package_path: PackagePath,
) -> VfsResult<CorgiConfig> {
    let corgi_config_paths = package_corgi_config_paths(db, package_path)?;
    let mut builder = CorgiConfigBuilder::new(db);
    for path in corgi_config_paths {
        builder.read(*path)?
    }
    Ok(builder.finish())
}

fn package_corgi_config_paths(
    db: &dyn CorgiConfigDb,
    package_path: PackagePath,
) -> VfsResult<&[DiffPath]> {
    package_corgi_config_paths_aux(db, package_path)
        .as_ref()
        .map(|v| v as &[DiffPath])
        .map_err(|e| e.clone())
}

#[salsa::tracked(jar = CorgiConfigJar, return_ref)]
fn package_corgi_config_paths_aux(
    db: &dyn CorgiConfigDb,
    package_path: PackagePath,
) -> VfsResult<Vec<DiffPath>> {
    let dir = package_path.dir(db)?;
    collect_corgi_config_paths_starting_from_dir(db, dir)
}

fn collect_corgi_config_paths_starting_from_dir(
    db: &dyn CorgiConfigDb,
    dir: DiffPath,
) -> VfsResult<Vec<DiffPath>> {
    let mut paths = dir
        .abs_path(db)?
        .ancestors()
        .map(|path| DiffPath::try_new(db, path.join(".corgi/config.toml")))
        .collect::<VfsResult<Vec<_>>>()?;
    paths.push(root_corgi_config_path(db)?);
    Ok(paths)
}

#[salsa::tracked(jar = CorgiConfigJar)]
fn root_corgi_config_path(db: &dyn CorgiConfigDb) -> VfsResult<DiffPath> {
    DiffPath::try_new(db, husky_fs_specs::root_corgi_config_path()?)
}
