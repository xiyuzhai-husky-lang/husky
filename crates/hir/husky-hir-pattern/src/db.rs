use crate::*;

pub trait HirPatternDb: salsa::DbWithJar<HirPatternJar> {}

#[salsa::jar(db = HirPatternDb)]
pub struct HirPatternJar(HirPattern);
