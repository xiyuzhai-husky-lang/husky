use crate::*;

pub trait LinkagePathDb: salsa::DbWithJar<LinkagePathJar> {}

impl<Db> LinkagePathDb for Db where Db: salsa::DbWithJar<LinkagePathJar> {}

#[salsa::jar(db = LinkagePathDb)]
pub struct LinkagePathJar(LinkagePath, LinkageDeps);
