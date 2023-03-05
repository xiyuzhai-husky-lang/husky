use crate::*;
use husky_precise_term::PreciseTermDb;

pub trait PreciseTypeDb: salsa::DbWithJar<PreciseTypeJar> + PreciseTermDb {}

impl<Db> PreciseTypeDb for Db where Db: salsa::DbWithJar<PreciseTypeJar> + PreciseTermDb {}
