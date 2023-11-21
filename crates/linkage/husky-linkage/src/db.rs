use crate::*;
use husky_entity_path::EntityPathDb;

pub trait LinkageDb: salsa::DbWithJar<LinkageJar> + EntityPathDb {}

impl<Db> LinkageDb for Db where Db: salsa::DbWithJar<LinkageJar> + EntityPathDb {}

#[salsa::jar(db = LinkageDb)]
pub struct LinkageJar(
    Linkage,
    crate::dependency::linkage_path_dependencies,
    crate::root::root_linkage_paths,
    crate::version_stamp::LinkageVersionStamp,
);
