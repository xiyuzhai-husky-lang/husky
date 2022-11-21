use husky_entity_path::{EntityPath, PackagePath};
use salsa::DbWithJar;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourcePathData {
    Module(EntityPath),
    CorgiToml(PackagePath),
}

#[salsa::interned(jar = SourcePathJar)]
pub struct SourcePath {
    data: SourcePathData,
}

#[salsa::jar(db = SourcePathDb)]
pub struct SourcePathJar(SourcePath);

pub trait SourcePathDb: DbWithJar<SourcePathJar> {}

impl<T> SourcePathDb for T where T: DbWithJar<SourcePathJar> {}
