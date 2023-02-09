use crate::*;

pub trait HirDb: salsa::DbWithJar<HirJar> {}
