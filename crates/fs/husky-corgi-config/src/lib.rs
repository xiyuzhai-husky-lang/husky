#![feature(let_chains)]
#![feature(trait_upcasting)]
mod builder;
mod db;
mod error;
mod sections;
pub mod transpilation_setup;


pub use self::error::*;
pub use self::sections::*;

use self::builder::*;
use husky_corgi_config_ast::*;
use husky_vfs::{error::VfsResult, *};


#[salsa::jar(db = CorgiConfigDb)]
pub struct CorgiConfigJar(
    package_corgi_config,
    package_corgi_config_paths_aux,
    package_registry_path,
    root_corgi_config_path,
    crate::transpilation_setup::linktime_target_transpilation_setup,
    crate::transpilation_setup::TranspilationSetup,
);

#[derive(Debug, PartialEq, Eq)]
pub struct CorgiConfig {
    registry_section: RegistrySection,
    errors: Vec<CorgiConfigError>,
}

#[salsa::tracked(jar = CorgiConfigJar, return_ref)]
pub(crate) fn package_registry_path(
    db: &::salsa::Db,
    package: PackagePath,
) -> CorgiConfigResult<RegistryPath> {
    let corgi_config = package.corgi_config(db)?;
    Ok(corgi_config.registry_section.path)
}

pub trait HasCorgiConfig: Copy {
    fn corgi_config(self, db: &::salsa::Db) -> CorgiConfigResultRef<&CorgiConfig>;

    fn registry_path(self, db: &::salsa::Db) -> CorgiConfigResultRef<RegistryPath>;
}

impl HasCorgiConfig for PackagePath {
    fn corgi_config(self, db: &::salsa::Db) -> CorgiConfigResultRef<&CorgiConfig> {
        package_corgi_config(db, self).as_ref().map_err(|e| e)
    }

    fn registry_path(self, db: &::salsa::Db) -> CorgiConfigResultRef<RegistryPath> {
        package_registry_path(db, self).as_ref().copied()
    }
}

#[salsa::tracked(jar = CorgiConfigJar, return_ref)]
fn package_corgi_config(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> CorgiConfigResult<CorgiConfig> {
    let corgi_config_paths = package_corgi_config_paths(db, package_path)?;
    let mut builder = CorgiConfigBuilder::new(db);
    for path in corgi_config_paths {
        builder.read(*path)?
    }
    Ok(builder.finish())
}

fn package_corgi_config_paths(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> VfsResult<&[VirtualPath]> {
    package_corgi_config_paths_aux(db, package_path)
        .as_ref()
        .map(|v| v as &[VirtualPath])
        .map_err(|e| e.clone())
}

#[salsa::tracked(jar = CorgiConfigJar, return_ref)]
fn package_corgi_config_paths_aux(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> VfsResult<Vec<VirtualPath>> {
    let dir = package_path.dir(db)?;
    collect_corgi_config_paths_starting_from_dir(db, dir)
}

fn collect_corgi_config_paths_starting_from_dir(
    db: &::salsa::Db,
    dir: VirtualPath,
) -> VfsResult<Vec<VirtualPath>> {
    let mut paths = dir
        .abs_path(db)?
        .ancestors()
        .map(|path| VirtualPath::try_new(db, path.join(".corgi/config.toml")))
        .collect::<VfsResult<Vec<_>>>()?;
    paths.push(root_corgi_config_path(db)?);
    Ok(paths)
}

#[salsa::tracked(jar = CorgiConfigJar)]
fn root_corgi_config_path(db: &::salsa::Db) -> VfsResult<VirtualPath> {
    VirtualPath::try_new(db, husky_fs_specs::root_corgi_config_path()?)
}
