use crate::*;
use husky_entity_path::EntityPathDb;
use husky_package_path::{PackagePathData, PackagePathDb};
use std::path::{Path, PathBuf};

pub trait SourcePathDb:
    DbWithJar<SourcePathJar> + HasSourcePathConfig + PackagePathDb + EntityPathDb
{
    fn source_path_db(&self) -> &dyn SourcePathDb;
    fn it_source_path(&self, data: SourcePathData) -> SourcePath;
    fn it_corgi_toml_path(&self, package: PackagePath) -> SourcePath {
        self.it_source_path(SourcePathData::CorgiToml(package))
    }
    fn it_module_path(&self, entity: EntityPath) -> SourcePath {
        self.it_source_path(SourcePathData::Module(entity))
    }
}

impl<T> SourcePathDb for T
where
    T: DbWithJar<SourcePathJar> + HasSourcePathConfig + PackagePathDb + EntityPathDb,
{
    fn source_path_db(&self) -> &dyn SourcePathDb {
        self
    }

    fn it_source_path(&self, data: SourcePathData) -> SourcePath {
        SourcePath::new(self, data)
    }
}
