use crate::*;

pub trait TracePathDb: salsa::DbWithJar<TracePathJar> + EntityPathDb {}

#[salsa::jar(db = TracePathDb)]
pub struct TracePathJar(TracePath);
