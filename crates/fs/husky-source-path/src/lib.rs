mod db;
mod jar;
mod physical;

pub use db::*;
pub use jar::SourcePathJar;

use husky_entity_path::EntityPath;
use husky_package_path::PackagePath;
use physical::*;
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
