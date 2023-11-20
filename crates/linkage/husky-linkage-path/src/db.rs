use crate::*;
use husky_entity_path::EntityPathDb;

pub trait LinkagePathDb: salsa::DbWithJar<LinkagePathJar> + EntityPathDb {}

impl<Db> LinkagePathDb for Db where Db: salsa::DbWithJar<LinkagePathJar> + EntityPathDb {}

#[salsa::jar(db = LinkagePathDb)]
pub struct LinkagePathJar(
    LinkagePath,
    crate::dependency::linkage_path_dependencies,
    crate::root::root_linkage_paths,
);
