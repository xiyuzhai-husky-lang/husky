use crate::*;
use husky_precise_term::RawTermDb;

pub trait PreciseTermDb: salsa::DbWithJar<PreciseTypeJar> + RawTermDb {}

impl<Db> PreciseTermDb for Db where Db: salsa::DbWithJar<PreciseTypeJar> + RawTermDb {}
