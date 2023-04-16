use crate::*;

pub trait FluffyTermDb: salsa::DbWithJar<FluffyTermJar> + EtherealTermDb {}

impl<Db> FluffyTermDb for Db where Db: salsa::DbWithJar<FluffyTermJar> + EtherealTermDb {}
