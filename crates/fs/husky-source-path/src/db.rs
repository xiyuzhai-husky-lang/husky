use husky_package_path::{PackagePathData, PackagePathDb};

use crate::*;
use std::path::Path;

pub trait SourcePathDb: DbWithJar<SourcePathJar> + PackagePathDb {
    fn it_source_path(&self, data: SourcePathData) -> SourcePath;
    fn source_path_jar(&self) -> &SourcePathJar;
    fn source_durability(&self, path: SourcePath) -> Durability;
    // todo: improve this to &PhysicalPath
    fn with_physical_path(&self, path: SourcePath) -> std::io::Result<PhysicalPath>;
}

impl<T> SourcePathDb for T
where
    T: DbWithJar<SourcePathJar> + PackagePathDb,
{
    fn it_source_path(&self, data: SourcePathData) -> SourcePath {
        SourcePath::new(self, data)
    }

    fn source_path_jar(&self) -> &SourcePathJar {
        &<Self as salsa::storage::HasJar<SourcePathJar>>::jar(self).0
    }

    fn source_durability(&self, path: SourcePath) -> Durability {
        todo!()
        // match self {
        //     HuskyFileClass::Library => Durability::HIGH,
        //     HuskyFileClass::Publish => Durability::HIGH,
        //     HuskyFileClass::User => Durability::LOW,
        // }
    }

    fn with_physical_path(&self, path: SourcePath) -> std::io::Result<PhysicalPath> {
        self.source_path_jar()
            .physical_path(path, || resolve_physical_path(self, path))
    }
}

fn resolve_physical_path(db: &dyn SourcePathDb, path: SourcePath) -> std::io::Result<PhysicalPath> {
    match path.data(db) {
        SourcePathData::Module(_) => todo!(),
        SourcePathData::CorgiToml(package) => match db.package_path_data(package) {
            PackagePathData::Builtin { toolchain } => todo!(),
            PackagePathData::Global { version } => todo!(),
            PackagePathData::Local(_) => todo!(),
            PackagePathData::Git(_) => todo!(),
        },
    }
}
