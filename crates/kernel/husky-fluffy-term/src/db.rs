use crate::*;

pub trait FluffyTermDb: salsa::DbWithJar<FluffyTermJar> + EtherealTypeDb {}

impl<Db> FluffyTermDb for Db where Db: salsa::DbWithJar<FluffyTermJar> + EtherealTypeDb {}
