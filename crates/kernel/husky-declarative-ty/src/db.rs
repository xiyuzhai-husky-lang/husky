use crate::*;

pub trait DeclarativeTypeDb: salsa::DbWithJar<DeclarativeTypeJar> + SignatureDb {}

impl<Db> DeclarativeTypeDb for Db where Db: salsa::DbWithJar<DeclarativeTypeJar> + SignatureDb {}
