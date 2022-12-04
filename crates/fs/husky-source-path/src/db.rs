use crate::*;
use absolute_path::AbsolutePath;
use husky_entity_path::EntityPathDb;
use husky_package_path::{PackagePathData, PackagePathDb};
use std::path::{Path, PathBuf};

pub trait SourcePathDb:
    DbWithJar<SourcePathJar> + HasSourcePathConfig + PackagePathDb + EntityPathDb
{
    fn it_source_path(&self, data: SourcePathData) -> SourcePath;
    fn source_path_from_physical_path(&self, path: &Path) -> SourcePathResult<Option<SourcePath>>;
    fn source_path_jar(&self) -> &SourcePathJar;
    fn source_durability(&self, path: SourcePath) -> Durability;
    fn it_corgi_toml_path(&self, package: PackagePath) -> SourcePath {
        self.it_source_path(SourcePathData::CorgiToml(package))
    }
    fn it_module_path(&self, entity: EntityPath) -> SourcePath {
        self.it_source_path(SourcePathData::Module(entity))
    }
    // todo: improve this to &AbsolutePath
    fn source_absolute_path(&self, path: SourcePath) -> SourcePathResult<AbsolutePath>;
}

impl<T> SourcePathDb for T
where
    T: DbWithJar<SourcePathJar> + HasSourcePathConfig + PackagePathDb + EntityPathDb,
{
    fn it_source_path(&self, data: SourcePathData) -> SourcePath {
        SourcePath::new(self, data)
    }

    fn source_path_from_physical_path(&self, path: &Path) -> SourcePathResult<Option<SourcePath>> {
        self.source_path_jar().source_path_from_physical_path(path)
    }

    fn source_path_jar(&self) -> &SourcePathJar {
        &<Self as salsa::storage::HasJar<SourcePathJar>>::jar(self).0
    }

    fn source_durability(&self, path: SourcePath) -> Durability {
        match path.data(self) {
            SourcePathData::Module(_) => todo!(),
            SourcePathData::CorgiToml(package) => match self.package_path_data(package) {
                PackagePathData::Builtin { .. } => Durability::HIGH,
                PackagePathData::Global { version } => todo!(),
                PackagePathData::Local(_) => Durability::LOW,
                PackagePathData::Git(_) => todo!(),
            },
        }
    }

    fn source_absolute_path(&self, path: SourcePath) -> SourcePathResult<AbsolutePath> {
        self.source_path_jar()
            .physical_path(path, || source_absolute_path(self, path).clone())
    }
}
