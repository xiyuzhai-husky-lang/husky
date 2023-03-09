use crate::*;

pub trait RawTypeDb: salsa::DbWithJar<RawTypeJar> + SignatureDb {}

impl<Db> RawTypeDb for Db where Db: salsa::DbWithJar<RawTypeJar> + SignatureDb {}
