use crate::*;

pub trait FluffyTermDb: salsa::DbWithJar<FluffyTermJar> + TypeDb {}

impl<Db> FluffyTermDb for Db where Db: salsa::DbWithJar<FluffyTermJar> + TypeDb {}
