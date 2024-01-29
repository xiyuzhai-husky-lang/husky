use crate::*;

pub trait HirPatternDb: salsa::DbWithJar<HirPatternJar> {}

#[salsa::jar]
pub struct HirPatternJar(HirPattern);
