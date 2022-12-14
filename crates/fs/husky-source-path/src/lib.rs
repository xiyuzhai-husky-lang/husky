#![feature(absolute_path)]
mod absolute;
mod config;
mod db;
mod error;
mod jar;

pub use config::{
    HasSourcePathConfig, SourcePathConfig, SourcePathConfigImpl, SourcePathConfigMimic,
};
pub use db::*;
pub use error::*;
pub use jar::SourcePathJar;

use absolute::*;
use husky_entity_path::EntityPath;
use husky_package_path::PackagePath;
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
