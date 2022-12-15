mod config;
mod db;
mod error;
mod jar;
#[cfg(test)]
mod tests;

pub use config::{
    HasSourcePathConfig, SourcePathConfig, SourcePathConfigImpl, SourcePathConfigMimic,
};
pub use db::*;
pub use error::*;
pub use jar::SourcePathJar;

use husky_entity_path::EntityPath;
use husky_package_path::{PackagePath, PackagePathData};
use salsa::{DbWithJar, Durability};

#[salsa::interned(jar = SourcePathJar)]
pub struct SourcePath {
    pub data: SourcePathData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourcePathData {
    Module(EntityPath),
    CorgiToml(PackagePath),
}

impl SourcePath {
    pub fn durability(self, db: &dyn SourcePathDb) -> salsa::Durability {
        match self.data(db) {
            SourcePathData::Module(entity_path) => match db.is_builtin_entity(entity_path) {
                true => Durability::HIGH,
                false => Durability::LOW,
            },
            SourcePathData::CorgiToml(package) => match db.package_path_data(package) {
                PackagePathData::Builtin { .. } => Durability::HIGH,
                PackagePathData::Global { version } => todo!(),
                PackagePathData::Local(_) => Durability::LOW,
                PackagePathData::Git(_) => todo!(),
            },
        }
    }
}
