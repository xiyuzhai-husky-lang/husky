use crate::*;
use husky_entity_path::EntityPathDb;
use husky_hir_defn::db::HirDefnDb;

pub trait LinkageDb: salsa::DbWithJar<LinkageJar> + EntityPathDb + HirDefnDb {}

impl<Db> LinkageDb for Db where Db: salsa::DbWithJar<LinkageJar> + EntityPathDb + HirDefnDb {}

#[salsa::jar(db = LinkageDb)]
pub struct LinkageJar(
    crate::linkage::Linkage,
    crate::dependency::linkage_path_dependencies,
    crate::root::root_linkage_paths,
    crate::version_stamp::LinkageVersionStamp,
    crate::template_argument::ty::LinkageTypePathLeading,
    crate::template_argument::ty::LinkageTypeRitchie,
);
