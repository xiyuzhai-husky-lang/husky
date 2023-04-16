use super::*;

pub trait EtherealTypeDb: salsa::DbWithJar<EtherealTypeJar> + EtherealTermDb {}

impl<Db> EtherealTypeDb for Db where Db: salsa::DbWithJar<EtherealTypeJar> + EtherealTermDb {}
