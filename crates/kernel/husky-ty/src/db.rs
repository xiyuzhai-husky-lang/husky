use super::*;

pub trait TypeDb: salsa::DbWithJar<TypeJar> + EtherealTermDb {}

impl<Db> TypeDb for Db where Db: salsa::DbWithJar<TypeJar> + EtherealTermDb {}
